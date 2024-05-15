

# Freelance Marketplace on Stellar Blockchain

 This project implements a freelance marketplace service built on the Stellar blockchain using Rust. The contract facilitates secure escrow transactions between clients and freelancers.

 ## Features

 - Initiating escrow agreements
 - Secure deposits into escrow accounts
 - Milestone-based release of funds
 - Dispute resolution through refunds
   
 ## Technology Stack

 - Stellar blockchain
 - Rust Programming Language
 - Soroban SDK
 ## Getting Started

 This project requires a working understanding of Rust development and the Stellar blockchain.

 - Ensure you have Rust and the Soroban SDK installed on your development machine.
 - Clone this repository to your local development environment.
 - Build the project using the cargo build command.
 - Deploy the contract to a Stellar network (e.g., testnet) using the appropriate tools.
 ## Usage

 The contract exposes several functions to manage escrow interactions:

 - initiate_escrow: Creates a new escrow agreement between a client and freelancer.
 - deposit_funds: Allows clients or freelancers to deposit funds into an existing escrow account.
 - release_funds: Enables clients to release funds to freelancers upon completion of milestones.
 - refund_funds: Initiates a refund process for the client if necessary.
   
## Further Development

 This is a basic implementation of a freelance marketplace service. I plan on imporving it with :

--Multi-signature support for escrow accounts

--Dispute resolution mechanisms

--Integration with a user interface

--Reputation management system for freelancers
