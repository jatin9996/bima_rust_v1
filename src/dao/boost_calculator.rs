#![cfg_attr(not(feature = "std"), no_std)]

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::dependencies::system_start::SystemStart; // Import SystemStart
use borsh::{BorshDeserialize, BorshSerialize}; // Import Borsh traits
use bitcoin::{self, Transaction}; // Import bitcoin crate and Transaction struct
use archnetwork::transaction_to_sign::TransactionToSign; // Import TransactionToSign
use archnetwork::pubkey::Pubkey; // Import Pubkey
use arch_program::utxo::UtxoMeta; // Import UtxoMeta

use arch_program::{
    program::{get_account_script_pubkey, get_bitcoin_tx, set_transaction_to_sign},
    transaction_to_sign::TransactionToSign,
    input_to_sign::InputToSign,
    pubkey::Pubkey as ArchPubkey,
    msg,
    program_error::ProgramError,
    helper::get_state_transition_tx,
};

#[derive(BorshSerialize, BorshDeserialize)] // Derive Borsh traits
pub struct BoostCalculator {
    locker: Pubkey, // Change AccountId to Pubkey
    max_boost_grace_weeks: u32,
    account_weekly_lock_pct: HashMap<(Pubkey, u32), u64>,
    total_weekly_weights: HashMap<u32, u64>,
    transactions: Vec<Transaction>,
    utxos: HashMap<OutPoint, UtxoMeta>, // Add UTXO management
}

impl BoostCalculator {
    pub fn new(locker: Pubkey, grace_weeks: u32, system_start: SystemStart) -> Self { // Change AccountId to Pubkey
        let current_week = system_start.get_week(); // Use SystemStart for getting the current week
        BoostCalculator {
            locker,
            max_boost_grace_weeks: grace_weeks + current_week,
            account_weekly_lock_pct: HashMap::new(),
            total_weekly_weights: HashMap::new(),
            transactions: Vec::new(), // Initialize the transactions vector
            utxos: HashMap::new(), // Initialize the UTXO map
        }
    }

    pub fn get_week(&self, system_start: &SystemStart) -> u32 {
        system_start.get_week() as u32
    }

    pub fn get_boosted_amount(&self, account: Pubkey, amount: u64, previous_amount: u64, total_weekly_emissions: u64) -> u64 { // Change AccountId to Pubkey
        let week = self.get_week();
        if week < self.max_boost_grace_weeks {
            return amount;
        }

        let adjusted_week = week - 1;
        let account_weight = self.account_weekly_lock_pct.get(&(account, adjusted_week)).copied().unwrap_or(0);
        let total_weight = self.total_weekly_weights.get(&adjusted_week).copied().unwrap_or(1);
        let pct = 1_000_000_000 * account_weight / total_weight;

        self.calculate_adjusted_amount(amount, previous_amount, total_weekly_emissions, pct)
    }

    fn calculate_adjusted_amount(&self, amount: u64, previous_amount: u64, total_weekly_emissions: u64, pct: u64) -> u64 {
        let base_amount = amount * pct / 1_000_000_000;
        if total_weekly_emissions > 0 {
            let emission_factor = previous_amount * 1_000_000_000 / total_weekly_emissions;
            base_amount * emission_factor / 1_000_000_000
        } else {
            base_amount
        }
    }

    // Modify the add_transaction method to use Archnetwork functions
    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), ProgramError> {
        let tx_bytes = self.create_transaction_bytes(&transaction);
        let inputs_to_sign = vec![InputToSign {
            index: 0,
            signer: self.locker.clone(),
        }];
        let transaction_to_sign = TransactionToSign {
            tx_bytes: &tx_bytes,
            inputs_to_sign: &inputs_to_sign,
        };

        // Log the transaction to sign
        msg!("Transaction to sign: {:?}", transaction_to_sign);

        set_transaction_to_sign(&[self.locker.clone()], transaction_to_sign)?;
        self.transactions.push(transaction);

        // Create or fetch a state transition transaction
        let state_transition_tx = get_state_transition_tx(&[self.locker.clone()]);
        msg!("State transition transaction: {:?}", state_transition_tx);

        Ok(())
    }

    pub fn add_utxo(&mut self, tx: &Transaction, vout: u32, value: u64, script_pubkey: Vec<u8>) {
        let outpoint = OutPoint::new(tx.txid(), vout);
        let utxo_meta = UtxoMeta {
            txid: tx.txid(),
            vout,
            value,
            script_pubkey,
        };
        self.utxos.insert(outpoint, utxo_meta);
    }

    pub fn spend_utxo(&mut self, outpoint: OutPoint) -> Result<(), ProgramError> {
        if self.utxos.remove(&outpoint).is_none() {
            return Err(ProgramError::Custom(502)); // UTXO not found
        }
        Ok(())
    }

    fn create_transaction_bytes(&self, transaction: &Transaction) -> Vec<u8> {
        // Use get_bitcoin_tx to retrieve the transaction bytes
        get_bitcoin_tx(transaction)
    }

    fn sign_transaction(&self, transaction: &TransactionToSign) {
        // Simulate signing the transaction
        msg!("Signing transaction with inputs: {:?}", transaction.inputs_to_sign);
    }
}

// Define traits for BoostCalculator and TokenLocker
pub trait IBoostCalculator {
    fn get_boosted_amount(&self, account: Pubkey, amount: u64, previous_amount: u64, total_weekly_emissions: u64) -> u64; // Change AccountId to Pubkey
}

pub trait ITokenLocker {
    fn lock_tokens(&self, account: Pubkey, amount: u64); // Change AccountId to Pubkey
    fn claim_tokens(&self, account: Pubkey) -> u64; // Change AccountId to Pubkey
}