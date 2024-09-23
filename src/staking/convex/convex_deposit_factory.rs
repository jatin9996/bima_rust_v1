#![cfg_attr(not(feature = "std"), no_std)]

use std::collections::HashMap;
use borsh::{BorshSerialize, BorshDeserialize};
use secp256k1::{Secp256k1, Message, PublicKey, Signature};
use arch_program::{
    account::AccountInfo,
    entrypoint,
    instruction::Instruction,
    program::{invoke, next_account_info, set_return_data},
    program_error::ProgramError,
    pubkey::Pubkey, // Ensure Pubkey is imported
    system_instruction::SystemInstruction,
    transaction_to_sign::TransactionToSign, // Ensure this import is present
    utxo::UtxoMeta,
};
use bitcoin::{self, Transaction}; // Import bitcoin crate and Transaction struct

/// Struct representing ConvexDepositToken
#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub struct ConvexDepositToken {
    pid: u64,
    owner: Pubkey,
}

impl ConvexDepositToken {
    pub fn initialize(&mut self, pid: u64, owner: Pubkey) {
        self.pid = pid;
        self.owner = owner;
    }

    // Further functions for ConvexDepositToken would go here...
}

/// BabelOwnable equivalent structure in Rust
#[derive(BorshSerialize, BorshDeserialize)]
pub struct BabelOwnable {
    babel_core: Pubkey,
    owner: Pubkey,
}

impl BabelOwnable {
    pub fn new(babel_core: Pubkey, owner: Pubkey) -> Self {
        Self { babel_core, owner }
    }

    pub fn only_owner(&self, caller: Pubkey) -> Result<(), ProgramError> {
        if caller != self.owner {
            return Err(ProgramError::Custom(1)); // Custom error for "Only owner"
        }
        Ok(())
    }
}

/// Main factory contract for deploying ConvexDepositToken clones
#[derive(BorshSerialize, BorshDeserialize)]
pub struct ConvexFactory {
    babel_ownable: BabelOwnable,
    deposit_token_impl: ConvexDepositToken,
    deployed_tokens: HashMap<u64, Pubkey>, // Mapping pid -> deposit token address
}

impl ConvexFactory {
    pub fn new(babel_core: Pubkey, deposit_token_impl: ConvexDepositToken) -> Self {
        Self {
            babel_ownable: BabelOwnable::new(babel_core, babel_core), // Assuming the Babel core is the owner
            deposit_token_impl,
            deployed_tokens: HashMap::new(),
        }
    }

    /// Deploy a new ConvexDepositToken instance with a unique `pid`
    pub fn deploy_new_instance(
        &mut self,
        pid: u64,
        caller: Pubkey,
        account_info: &AccountInfo,
    ) -> Result<Pubkey, ProgramError> {
        // Ensure only the owner can deploy new instances
        self.babel_ownable.only_owner(caller)?;

        // Clone the deposit token
        let mut new_token = self.deposit_token_impl.clone();
        new_token.initialize(pid, caller);

        // Register the new token in the deployed tokens map
        let new_token_address = self.create_deterministic_address(pid)?;
        self.deployed_tokens.insert(pid, new_token_address);

        // Emit event (simulated in Rust)
        self.emit_new_deployment_event(pid, new_token_address);

        Ok(new_token_address)
    }

    /// Predict the address for a ConvexDepositToken based on `pid`
    pub fn get_deposit_token(&self, pid: u64) -> Option<Pubkey> {
        self.deployed_tokens.get(&pid).cloned()
    }

    /// Emit event for new deployment (this would be simulated in Rust)
    fn emit_new_deployment_event(&self, pid: u64, token_address: Pubkey) {
        // Logging or external output to indicate new deployment
        // Simulate Solidity's event emission in the log
        msg!("NewDeployment: pid: {}, deposit_token: {}", pid, token_address);
    }

    /// Simulate deterministic address creation (can be a hash or some logic to derive an address)
    fn create_deterministic_address(&self, pid: u64) -> Result<Pubkey, ProgramError> {
        // Here, we simulate generating a new deterministic address for the token
        // In practice, this could involve hashing the pid or using some other
        // method to ensure the same input always gives the same output address.

        // For simplicity, we'll use the pid as a base and generate a Pubkey from it
        let pubkey_bytes = [0u8; 32]; // Placeholder bytes for the public key
        Ok(Pubkey::new(&pubkey_bytes))
    }
}

fn msg(message: &str) {
    // Simulating the msg functionality from Solidity in Rust
    println!("{}", message);
}