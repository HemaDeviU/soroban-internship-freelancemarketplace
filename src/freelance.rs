#![no_std]

use soroban_sdk::{ contract, contractimpl, contracttype, Address, Env, Vec, String };

#[derive(Clone)]
#[contracttype]
pub enum StorageKey {
    EscrowID,
}

#[derive(Clone)]
#[contracttype]
pub struct Escrow {
    client: Address,
    freelancer: Address,
    total_amount: u64,
    milestones: Vec<Milestone>,
    released_amount: u64,
    state: EscrowState,
}

#[derive(Clone)]
#[contracttype]
pub enum EscrowState {
    Created,
    InProgress,
    Completed,
    Refunded,
}

#[derive(Clone)]
#[contracttype]
pub struct Milestone {
    description: String,
    amount: u64,
    completed: bool,
}

#[contract]
pub struct EscrowServiceContract;

#[contractimpl]
impl EscrowServiceContract {
    pub fn initiate_escrow(
        env: Env,
        from: Address,
        freelancer: Address,
        total_amount: u64,
        milestones: Vec<Milestone>
    ) {
        // Ensure that the transaction sender is authorized to initiate escrow
        from.require_auth();

        // Generate a new unique escrow ID
        let escrow_id =
            env.storage().instance().get::<_, u64>(&StorageKey::EscrowID).unwrap_or(0) + 1;

        // Create the escrow
        let escrow = Escrow {
            client: from,
            freelancer,
            total_amount,
            milestones,
            released_amount: 0,
            state: EscrowState::Created,
        };

        // Store the escrow in the contract storage
        env.storage().instance().set(&escrow_id, &escrow);
        env.storage().instance().set(&StorageKey::EscrowID, &escrow_id);
    }

    pub fn deposit_funds(env: Env, from: Address, escrow_id: u64, amount: u64) {
        // Ensure that the transaction sender is authorized to deposit funds
        from.require_auth();

        // Retrieve the escrow from the contract storage
        let mut escrow: Escrow = env.storage().instance().get(&escrow_id).unwrap();

        // Verify that the sender is involved in the escrow
        if from != escrow.client && from != escrow.freelancer {
            panic!("Only client or freelancer can deposit funds");
        }

        // Update the released amount
        escrow.released_amount += amount;

        // Update the escrow in the contract storage
        env.storage().instance().set(&escrow_id, &escrow);
    }

    pub fn release_funds(env: Env, from: Address, escrow_id: u64, milestone_index: u32) {
        // Ensure that the transaction sender is authorized to release funds
        from.require_auth();

        // Retrieve the escrow from the contract storage
        let mut escrow: Escrow = env.storage().instance().get(&escrow_id).unwrap();

        // Verify that the sender is the client
        if from != escrow.client {
            panic!("Only client can release funds");
        }

        // Verify that the milestone exists and is completed
        if milestone_index >= (escrow.milestones.len() as u32) {
            panic!("Invalid milestone index");
        }

        // Update the state to reflect the released funds
        escrow.state = EscrowState::InProgress;

        // Update the released amount based on the milestone
        let milestone_amount = escrow.milestones
            .iter()
            .find(|m| m.completed)
            .map_or(0, |m| m.amount);
        escrow.released_amount += milestone_amount;

        // Update the escrow in the contract storage
        env.storage().instance().set(&escrow_id, &escrow);
    }

    pub fn refund_funds(env: Env, from: Address, escrow_id: u64) {
        // Ensure that the transaction sender is authorized to refund funds
        from.require_auth();

        // Retrieve the escrow from the contract storage
        let mut escrow: Escrow = env.storage().instance().get(&escrow_id).unwrap();

        // Verify that the sender is the client
        if from != escrow.client {
            panic!("Only client can request refund");
        }

        // Update the state to reflect the refunded funds
        escrow.state = EscrowState::Refunded;

        // Update the escrow in the contract storage
        env.storage().instance().set(&escrow_id, &escrow);
    }
}
