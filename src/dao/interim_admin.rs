#![cfg_attr(not(feature = "std"), no_std)]

use borsh::{BorshDeserialize, BorshSerialize};
use log::{info, warn};
use crate::interfaces::babel_core::BabelCore; // Ensure BabelCore is in scope
use arch_program::{
    account::AccountInfo,
    entrypoint,
    helper::get_state_transition_tx,
    input_to_sign::InputToSign,
    instruction::Instruction,
    msg,
    program::{get_account_script_pubkey, get_bitcoin_tx, get_network_xonly_pubkey, invoke, next_account_info, set_return_data, set_transaction_to_sign, validate_utxo_ownership},
    program_error::ProgramError,
    pubkey::Pubkey, // Ensure Pubkey is imported
    system_instruction::SystemInstruction,
    transaction_to_sign::TransactionToSign,
    utxo::UtxoMeta,
};
use bitcoin::{self, Transaction}; // Import bitcoin crate and Transaction struct
use archnetwork::transaction_to_sign::TransactionToSign; // Import TransactionToSign
use std::collections::HashMap;


#[derive(BorshSerialize, BorshDeserialize)]
struct Proposal {
    created_at: u64,
    can_execute_after: u64,
    processed: bool,
}

#[derive(BorshSerialize, BorshDeserialize)]
struct Action {
    target: String, // Assuming target is a string identifier
    data: Vec<u8>,
}

#[derive(BorshSerialize, BorshDeserialize)]
struct InterimAdmin {
    babel_core: String,
    proposals: Vec<Proposal>,
    proposal_payloads: Vec<Vec<Action>>,
    daily_proposals_count: Vec<u32>,
    guardian: Option<String>,
    owner: String, // Ownership management
}

// Define UTXO structure
struct UtxoSet {
    utxos: HashMap<OutPoint, UtxoMeta>,
}

impl UtxoSet {
    pub fn new() -> Self {
        UtxoSet {
            utxos: HashMap::new(),
        }
    }

    pub fn add_utxo(&mut self, tx: &Transaction, vout: u32, value: u64, script_pubkey: Script) {
        let outpoint = OutPoint::new(tx.txid(), vout);
        let utxo = UtxoMeta {
            txid: tx.txid(),
            vout,
            value,
            script_pubkey,
        };
        self.utxos.insert(outpoint, utxo);
    }

    pub fn spend_utxo(&mut self, outpoint: OutPoint) {
        self.utxos.remove(&outpoint);
    }
}

impl InterimAdmin {
    pub fn new(babel_core: String, owner: String) -> Self {
        Self {
            babel_core,
            proposals: Vec::new(),
            proposal_payloads: Vec::new(),
            daily_proposals_count: vec![0; 365], // Tracking for each day of the year
            guardian: None,
            owner,
        }
    }

    pub fn set_guardian(&mut self, caller: &str, guardian: String) {
        if self.is_owner(caller) && self.guardian.is_none() {
            self.guardian = Some(guardian);
            info!("Guardian set");
        } else {
            warn!("Guardian already set or unauthorized access");
        }
    }

    pub fn create_new_proposal(&mut self, caller: &str, payload: Vec<Action>) {
        if !self.is_owner(caller) {
            warn!("Unauthorized attempt to create proposal");
            return;
        }

        let current_time = self.get_current_time();
        let day = (current_time / 86400) as usize;
        assert!(self.daily_proposals_count[day] < 3, "MAX_DAILY_PROPOSALS reached");

        let proposal_id = self.proposals.len();
        self.proposals.push(Proposal {
            created_at: current_time,
            can_execute_after: current_time + 86400,
            processed: false,
        });
        self.proposal_payloads.push(payload);
        self.daily_proposals_count[day] += 1;


        // Create a transaction to sign
        let tx_bytes = self.create_transaction_bytes(proposal_id as u64);
        let inputs_to_sign = vec![InputToSign {
            index: 0,
            signer: caller.to_string(),
        }];
        let transaction = TransactionToSign::new(tx_bytes, inputs_to_sign);

        // Set the transaction to sign using Arch SDK
        set_transaction_to_sign(&[/* accounts */], transaction);

        // Use Arch SDK to log the proposal creation
        msg!("Proposal {} created", proposal_id);

        // Add UTXO management
        let mut utxo_set = UtxoSet::new();
        // Assuming we have a transaction and its details
        let tx = get_bitcoin_tx(); // Fetch the transaction
        utxo_set.add_utxo(&tx, 0, 1000, get_account_script_pubkey(caller)); // Example values

        // Use Arch SDK to log the proposal creation
        msg!("Proposal {} created", proposal_id);

    }

    pub fn cancel_proposal(&mut self, caller: &str, index: usize) {
        if let Some(guardian) = &self.guardian {
            if (caller == guardian || self.is_owner(caller)) && !self.proposals[index].processed {
                self.proposals[index].processed = true;


                // Create a transaction to sign
                let tx_bytes = self.create_transaction_bytes(index as u64);
                let inputs_to_sign = vec![InputToSign {
                    index: 0,
                    signer: caller.to_string(),
                }];
                let transaction = TransactionToSign::new(tx_bytes, inputs_to_sign);

                // Set the transaction to sign using Arch SDK
                set_transaction_to_sign(&[/* accounts */], transaction);
                // Use Arch SDK to log the proposal cancellation
                msg!("Proposal {} cancelled by {}", index, caller);
            } else {
                warn!("Unauthorized or already processed");
            }
        }
    }

    fn get_current_time(&self) -> u64 {
        // Fetch the current time from the blockchain using Arch SDK

        match arch_program::clock::get() {
            Ok(clock) => clock.unix_timestamp as u64,
            Err(_) => {
                warn!("Failed to fetch current time");
                0
            }
        }

    }

    fn is_owner(&self, caller: &str) -> bool {
        self.owner == caller
    }

    fn create_transaction_bytes(&self, amount: u64) -> Vec<u8> {
        // Simulate creating raw transaction bytes
        vec![amount as u8] // Example, adjust as needed
    }

    fn sign_transaction(&self, transaction: &TransactionToSign) {
        // Simulate signing the transaction
        println!("Signing transaction with inputs: {:?}", transaction.inputs_to_sign);
    }

    //  function to demonstrate handling Bitcoin transactions
    pub fn handle_bitcoin_transaction(&self, tx: Transaction) {
        // Process the Bitcoin transaction
        info!("Handling Bitcoin transaction: {:?}", tx);

        // Validate UTXO ownership using Arch SDK
        if let Err(e) = validate_utxo_ownership(&[/* accounts */], &tx) {
            warn!("UTXO ownership validation failed: {:?}", e);
            return;
        }

        // E logic: Check if the transaction has at least one input and one output
        if tx.input.is_empty() || tx.output.is_empty() {
            warn!("Transaction must have at least one input and one output");
            return;
        }

        // Example logic: Log the details of the first input and output
        let first_input = &tx.input[0];
        let first_output = &tx.output[0];
        info!("First input: {:?}", first_input);
        info!("First output: {:?}", first_output);

        //  logic: Check if the first output value is above a certain threshold
        let threshold = 1000; // Example threshold value in satoshis
        if first_output.value < threshold {
            warn!("First output value is below the threshold");
            return;
        }

        // If all checks pass, log success
        msg!("Bitcoin transaction processed successfully");

        // Add UTXO management
        let mut utxo_set = UtxoSet::new();
        utxo_set.add_utxo(&tx, 0, first_output.value, first_output.script_pubkey.clone());
    }
}