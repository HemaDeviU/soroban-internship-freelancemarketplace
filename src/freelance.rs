#![no_std]

use soroban_sdk::{ contract, contractimpl, contracttype, Address, Env, Vec, String };

#[derive(Clone)]
#[contracttype]
pub struct User {
 
}

#[derive(Clone)]
#[contracttype]
pub enum UserType {
  Client,
  Freelancer,
}

#[derive(Clone)]
#[contracttype]
pub struct Project {
  id: u64, // unique identifier
  client: Address,
  title: String,
  description: String,
  category: String,
  budget: u64,
  deadline: u64, // Unix timestamp for deadline
  milestones: Vec<Milestone>,
  status: ProjectStatus, // Open, InProgress, Completed, Cancelled
}

#[derive(Clone)]
#[contracttype]
pub enum ProjectStatus {
  Open,
  InProgress,
  Completed,
  Cancelled,
}

#[derive(Clone)]
#[contracttype]
pub struct Milestone {
  description: String,
  amount: u64,
  completed: bool,
  deadline: u64, // Unix timestamp for deadline (optional)
}

#[derive(Clone)]
#[contracttype]
pub struct Rating {
  from: Address, // rater (client)
  to: Address, // freelancer being rated
  rating: u8, // 1-5 star rating
  comment: String, // Optional comment
}

#[derive(Clone)]
#[contracttype]
pub struct Escrow {
  project_id: u64,
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
pub enum StorageKey {
  ProjectCount,
  UserCount, // Removed as user data is not stored
  Projects(u64), // Key for each project by ID
  Escrows(u64),  // Key for each escrow by ID
}

pub struct EscrowServiceContract;

#[contractimpl]
impl EscrowServiceContract {

  // Project Management
  pub fn post_project(
    env: Env,
    from: Address, // Client address
    title: String,
    description: String,
    category: String,
    budget: u64,
    deadline: u64, // Unix timestamp for deadline
    milestones: Vec<Milestone>,
  ) -> Result<u64, String> {
    // Ensure sender address is valid (basic check)
    if !env.accounts().is_valid_address(&from) {
      return Err(String::from("Invalid client address"));
    }

    let project_count = env.storage().instance().get::<_, u64>(&StorageKey::ProjectCount).unwrap_or(0);
    let project = Project {
      id: project_count + 1,
      client: from,
      title,
      description,
      category,
      budget,
      deadline,
      milestones,
      status: ProjectStatus::Open,
    };
    // Store project details in separate storage (consider database)
    env.storage().instance().set(&StorageKey::Projects(project_count + 1), &project);
    env.storage().instance().set(&StorageKey::ProjectCount, &(project_count + 1));
    Ok(project_count + 1)
  }

  // ... other project management functions (e.g., view projects, update project)

  // Escrow Management
  pub fn initiate_escrow(
    env: Env,
    from: Address, // Client address
    project_id: u64,
    freelancer: Address, // Freelancer address
  ) -> Result<(), String> {
    // Ensure sender address is valid (basic check)
    if !env.accounts().is_valid_address(&from) {
      return Err(String::from("Invalid client address"));
    }

    let project = env.storage().instance().get::<_, Project>(&StorageKey::Projects(project_id))?;
    // Ensure project exists and client address matches the project owner
    if project.is_none() || project.unwrap().client != from {
        return Err(String::from("Unauthorized: Only client who posted the project can initiate escrow"));
      }
  
      // Ensure project is open
      if project.unwrap().status != ProjectStatus::Open {
        return Err(String::from("Project is not open for escrow initiation"));
      }
  
      let escrow = Escrow {
        project_id,
        client: project.unwrap().client,
        freelancer,
        total_amount: project.unwrap().budget,
        milestones: project.unwrap().milestones.clone(),
        released_amount: 0,
        state: EscrowState::Created,
      };
  
      // Store escrow details
      let escrow_id = env.storage().instance().get::<_, u64>(&StorageKey::EscrowCount).unwrap_or(0) + 1;
      env.storage().instance().set(&StorageKey::Escrows(escrow_id), &escrow);
      env.storage().instance().set(&StorageKey::EscrowCount, &escrow_id);
  
      // Update project status
      let mut updated_project = project.unwrap().clone();
      updated_project.status = ProjectStatus::InProgress;
      env.storage().instance().set(&StorageKey::Projects(project_id), &updated_project);
  
      Ok(())
    }
  
    pub fn deposit_funds(env: Env, from: Address, escrow_id: u64, amount: u64) -> Result<(), String> {
      // Ensure sender address is valid (basic check)
      if !env.accounts().is_valid_address(&from) {
        return Err(String::from("Invalid address"));
      }
  
      let escrow = env.storage().instance().get::<_, Escrow>(&StorageKey::Escrows(escrow_id))?;
      
      // Verify if sender is involved in the escrow (client or freelancer address)
      if escrow.client != from && escrow.freelancer != from {
        return Err(String::from("Unauthorized: Only client or freelancer can deposit funds"));
      }
  
      // Update escrow state and released amount
      let mut updated_escrow = escrow.clone();
      updated_escrow.released_amount += amount;
      if updated_escrow.released_amount == updated_escrow.total_amount {
        updated_escrow.state = EscrowState::InProgress;
      }
      env.storage().instance().set(&StorageKey::Escrows(escrow_id), &updated_escrow);
  
      Ok(())
    }
  
    pub fn release_funds(env: Env, from: Address, escrow_id: u64, milestone_index: u32) -> Result<(), String> {
      // Ensure sender address is valid (basic check)
      if !env.accounts().is_valid_address(&from) {
        return Err(String::from("Invalid client address"));
      }
  
      let mut escrow = env.storage().instance().get::<_, Escrow>(&StorageKey::Escrows(escrow_id))?;
  
      // Verify milestone index and completion
      if milestone_index >= escrow.milestones.len() as u32 {
        return Err(String::from("Invalid milestone index"));
      }
      if !escrow.milestones[milestone_index as usize].completed {
        return Err(String::from("Milestone not marked as completed"));
      }
  
      // Calculate amount to release for the milestone
      let mut released_amount = 0;
      for i in 0..milestone_index as usize {
        released_amount += escrow.milestones[i].amount;
      }
  
      // Ensure sufficient funds are available
      if escrow.released_amount < released_amount {
        return Err(String::from("Insufficient funds deposited in escrow"));
      }
  
      // Update escrow state and released amount
      escrow.released_amount = released_amount;
      if escrow.released_amount == escrow.total_amount {
        escrow.state = EscrowState::Completed;
      }
      env.storage().instance().set(&StorageKey::Escrows(escrow_id), &escrow);
  
      Ok(())
    }
  
    pub fn refund_funds(env: Env, from: Address, escrow_id: u64) -> Result<(), String> {
      // Ensure sender address is valid (basic check)
      if !env.accounts().is_valid_address(&from) {
        return Err(String::from("Invalid client address"));
      }
  
      let mut escrow = env.storage().instance().get::<_, Escrow>(&StorageKey::Escrows(escrow_id))?;
  
      // Ensure escrow is in a refundable state
      if escrow.state != EscrowState::Created {
        return Err(String::from("Refund not allowed in current escrow state"));
      }
  
      // Update escrow state
      escrow.state = EscrowState::Refunded;
      env.storage().instance().set(&StorageKey::Escrows(escrow_id), &escrow);
  
      Ok(())
    }
  
    
