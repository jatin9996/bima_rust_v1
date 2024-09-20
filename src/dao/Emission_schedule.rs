#![cfg_attr(not(feature = "std"), no_std)]

use borsh::{BorshDeserialize, BorshSerialize};
use ink_prelude::collections::VecDeque;
use arch_program::{
    account::AccountInfo,
    entrypoint,
    helper::get_state_transition_tx,
    input_to_sign::InputToSign,
    instruction::Instruction,
    msg,
    program::{get_account_script_pubkey, get_bitcoin_tx, get_network_xonly_pubkey, invoke, next_account_info, set_return_data, set_transaction_to_sign, validate_utxo_ownership},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction::SystemInstruction,
    transaction_to_sign::TransactionToSign,
    utxo::UtxoMeta,

};
use bitcoin::{self, Transaction}; // Import the bitcoin crate and Transaction struct



#[derive(BorshSerialize, BorshDeserialize)]
struct BabelOwnable {
    owner: AccountId,
}

impl BabelOwnable {
    fn new(owner: AccountId) -> Self {
        Self { owner }
    }

    fn only_owner(&self, caller: AccountId) {
        assert_eq!(caller, self.owner, "Only owner can call this function");
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
struct EmissionSchedule {
    owner: AccountId,
    system_start: Timestamp,
    vault: AccountId,
    voter: AccountId,
    lock_weeks: u64,
    lock_decay_weeks: u64,
    weekly_pct: u64,
    scheduled_weekly_pct: VecDeque<(u64, u64)>,
    babel_ownable: BabelOwnable,
    utxos: Vec<UtxoMeta>, // Add UTXO metadata
}

impl EmissionSchedule {
    pub fn new(
        owner: Pubkey, // Changed AccountId to Pubkey
        system_start: Timestamp,
        vault: Pubkey, // Changed AccountId to Pubkey
        voter: Pubkey, // Changed AccountId to Pubkey
        initial_lock_weeks: u64,
        lock_decay_weeks: u64,
        weekly_pct: u64,
        scheduled_weekly_pct: Vec<(u64, u64)>,
        utxos: Vec<UtxoMeta>, // Add UTXO metadata
    ) -> Self {
        assert!(initial_lock_weeks <= MAX_LOCK_WEEKS, "Cannot exceed MAX_LOCK_WEEKS");
        assert!(lock_decay_weeks > 0, "Decay weeks cannot be 0");
        assert!(weekly_pct <= MAX_PCT, "Cannot exceed MAX_PCT");

        // Use Arch SDK to validate the owner account
        let owner_info = AccountInfo::new(&owner);
        assert!(validate_utxo_ownership(&owner_info).is_ok(), "Invalid owner account");

        Self {
            owner,
            system_start,
            vault,
            voter,
            lock_weeks: initial_lock_weeks,
            lock_decay_weeks,
            weekly_pct,
            scheduled_weekly_pct: scheduled_weekly_pct.into_iter().collect(),
            babel_ownable: BabelOwnable::new(owner),
            utxos, // Initialize UTXOs
        }
    }

    pub fn set_weekly_pct_schedule(&mut self, caller: Pubkey, schedule: Vec<(u64, u64)>) {
        self.babel_ownable.only_owner(caller);
        let mut last_week = u64::MAX;
        for &(week, pct) in &schedule {
            assert!(week < last_week, "Must sort by week descending");
            assert!(pct <= MAX_PCT, "Cannot exceed MAX_PCT");
            last_week = week;
        }
        self.scheduled_weekly_pct = schedule.into_iter().collect();


        // Use Arch SDK to validate the caller account
        let caller_info = AccountInfo::new(&caller);
        assert!(validate_utxo_ownership(&caller_info).is_ok(), "Invalid caller account");
    }


        // Validate UTXO ownership
        for utxo in &self.utxos {
            let utxo_info = AccountInfo::new(&utxo.pubkey);
            assert!(validate_utxo_ownership(&utxo_info).is_ok(), "Invalid UTXO ownership");
        }

        // Use Arch SDK to validate the caller account
        let caller_info = AccountInfo::new(&caller);
        assert!(validate_utxo_ownership(&caller_info).is_ok(), "Invalid caller account");

        // Create a state transition transaction
        let mut tx = get_state_transition_tx(&[caller_info]);
        tx.input.push(InputToSign {
            index: 0,
            signer: caller,
        });

        // Set the transaction to sign
        set_transaction_to_sign(&[caller_info], tx);
    }

    pub fn lock(&mut self, caller: Pubkey, weeks: u64) {
        self.babel_ownable.only_owner(caller);
        assert!(weeks <= MAX_LOCK_WEEKS, "Lock duration exceeds maximum allowed weeks");
        self.lock_weeks = weeks;


        // Validate UTXO ownership
        for utxo in &self.utxos {
            let utxo_info = AccountInfo::new(&utxo.pubkey);
            assert!(validate_utxo_ownership(&utxo_info).is_ok(), "Invalid UTXO ownership");
        }

        // Use Arch SDK to validate the caller account
        let caller_info = AccountInfo::new(&caller);
        assert!(validate_utxo_ownership(&caller_info).is_ok(), "Invalid caller account");

        // Create a state transition transaction
        let mut tx = get_state_transition_tx(&[caller_info]);
        tx.input.push(InputToSign {
            index: 0,
            signer: caller,
        });

        // Set the transaction to sign
        set_transaction_to_sign(&[caller_info], tx);

        // Use Arch SDK to validate the caller account
        let caller_info = AccountInfo::new(&caller);
        assert!(validate_utxo_ownership(&caller_info).is_ok(), "Invalid caller account");
    }

    pub fn unlock(&mut self, caller: Pubkey) {
        self.babel_ownable.only_owner(caller);
        self.lock_weeks = 0;


        // Validate UTXO ownership
        for utxo in &self.utxos {
            let utxo_info = AccountInfo::new(&utxo.pubkey);
            assert!(validate_utxo_ownership(&utxo_info).is_ok(), "Invalid UTXO ownership");
        }

        // Use Arch SDK to validate the caller account
        let caller_info = AccountInfo::new(&caller);
        assert!(validate_utxo_ownership(&caller_info).is_ok(), "Invalid caller account");

        // Create a state transition transaction
        let mut tx = get_state_transition_tx(&[caller_info]);
        tx.input.push(InputToSign {
            index: 0,
            signer: caller,
        });

        // Set the transaction to sign
        set_transaction_to_sign(&[caller_info], tx);
    }

    fn create_transaction_bytes(&self, amount: u64) -> Vec<u8> {
        // Simulate creating raw transaction bytes
        vec![amount as u8] // adjust as needed
    }

    fn sign_transaction(&self, transaction: &TransactionToSign) {
        // Simulate signing the transaction
        println!("Signing transaction with inputs: {:?}", transaction.inputs_to_sign);
    }

    // function to demonstrate usage of bitcoin::Transaction
    pub fn process_transaction(&self, tx: Transaction) {
        // Process the Bitcoin transaction using the Arch SDK
        let tx_info = get_bitcoin_tx(&tx);
        // Further processing logic here...
        // Use Arch SDK to validate the caller account
        let caller_info = AccountInfo::new(&caller);
        assert!(validate_utxo_ownership(&caller_info).is_ok(), "Invalid caller account");
    }
}

const MAX_PCT: u64 = 10000;
const MAX_LOCK_WEEKS: u64 = 52;

// Define types for AccountId and Timestamp as per your requirements
type AccountId = Pubkey; // Changed AccountId to Pubkey
type Timestamp = u64;