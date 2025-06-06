use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use borsh::{BorshDeserialize, BorshSerialize};
use borsh::maybestd::io::{Error, ErrorKind};
use bitcoin::{self, Transaction}; // Import bitcoin crate and Transaction struct

// Arch SDK imports
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

// Arch SDK imports
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
    bitcoin::{self, Transaction},
};

// Assuming a simplified interface for AggregatorV3Interface
trait AggregatorV3Interface {
    fn latest_round_data(&self) -> (u64, i128, u32, u32, u32);
}

#[derive(Default, Clone, BorshSerialize, BorshDeserialize)]
pub struct OracleRecord {
    chainlink_oracle: Box<dyn AggregatorV3Interface>,
    decimals: u8,
    heartbeat: u32,
    share_price_signature: [u8; 4],
    share_price_decimals: u8,
    is_feed_working: bool,
    is_eth_indexed: bool,
}

#[derive(Default, Clone, BorshSerialize, BorshDeserialize)]
pub struct PriceRecord {
    scaled_price: u128,
    timestamp: u32,
    last_updated: u32,
    round_id: u64,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct PriceFeed {
    oracle_records: HashMap<String, OracleRecord>,
    price_records: HashMap<String, PriceRecord>,
    owner: Pubkey, // Changed to Pubkey for compatibility with Arch SDK
}

impl PriceFeed {
    pub fn new(owner: Pubkey) -> Self {
        Self {
            oracle_records: HashMap::new(),
            price_records: HashMap::new(),
            owner,
        }
    }

    pub fn set_oracle(
        &mut self,
        token: String,
        chainlink_oracle: Box<dyn AggregatorV3Interface>,
        heartbeat: u32,
        share_price_signature: [u8; 4],
        share_price_decimals: u8,
        is_eth_indexed: bool,
    ) {
        let record = OracleRecord {
            chainlink_oracle,
            decimals: 18,
            heartbeat,
            share_price_signature,
            share_price_decimals,
            is_feed_working: true,
            is_eth_indexed,
        };
        self.oracle_records.insert(token, record);
    }

    pub fn fetch_price(&mut self, token: &str) -> Option<u128> {
        let current_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32;
        let price_record = self.price_records.get(token)?;

        if price_record.last_updated == current_timestamp {
            // Return cached price if it was updated in the current block
            return Some(price_record.scaled_price);
        }

        let oracle_record = self.oracle_records.get(token)?;
        let (round_id, answer, timestamp, _, _) = oracle_record.chainlink_oracle.latest_round_data();

        if self.is_price_stale(timestamp, oracle_record.heartbeat) {
            return None;
        }

        let mut scaled_price = (answer as u128) * 10u128.pow(18 - oracle_record.decimals as u32);

        if oracle_record.share_price_signature != [0; 4] {
            // Simulate fetching share price (this would be a call to another contract in Solidity)
            let share_price = 1u128; // Placeholder for actual share price fetching logic
            scaled_price = (scaled_price * share_price) / 10u128.pow(oracle_record.share_price_decimals as u32);
        }

        if oracle_record.is_eth_indexed {
            // Convert ETH price to USD
            let eth_price = self.fetch_price("ETH")?;
            scaled_price = (scaled_price * eth_price) / 1_000_000_000_000_000_000u128;
        }

        // Validate UTXO ownership before proceeding
        let utxo_meta = UtxoMeta {
            txid: fees_tx.txid(),
            vout: 0,
            amount: 0, // Placeholder, replace with actual amount
            script_pubkey: get_account_script_pubkey(account.key),
        };
        validate_utxo_ownership(&utxo_meta)?;

        self.store_price(token, scaled_price, timestamp, round_id);
        Some(scaled_price)
    }

    fn is_price_stale(&self, price_timestamp: u32, heartbeat: u32) -> bool {
        let current_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32;
        current_timestamp - price_timestamp > heartbeat + 3600 // RESPONSE_TIMEOUT_BUFFER equivalent
    }

    fn store_price(&mut self, token: &str, price: u128, timestamp: u32, round_id: u64) {
        // Create a state transition transaction using Arch SDK
        let mut tx = get_state_transition_tx(&[]);

        // Add necessary inputs to the transaction
        let inputs_to_sign = vec![InputToSign {
            index: 0,
            signer: self.owner,
        }];

        let tx_to_sign = TransactionToSign {
            tx_bytes: &bitcoin::consensus::serialize(&tx),
            inputs_to_sign: &inputs_to_sign,
        };

        // Add UTXO metadata to the transaction
        let utxo_meta = UtxoMeta {
            txid: tx.input[0].previous_output.txid,
            vout: tx.input[0].previous_output.vout,
            amount: 0, // Placeholder, replace with actual amount
            script_pubkey: get_account_script_pubkey(&self.owner),
        };

        // Ensure the transaction is signed by the owner
        self.only_owner();

        // Set the transaction to sign using Arch SDK
        set_transaction_to_sign(&[], tx_to_sign);

        // Log the transaction details
        msg!("Transaction to sign: {:?}", tx_to_sign);

        // Store the price record
        self.price_records.insert(
            token.to_string(),
            PriceRecord {
                scaled_price: price,
                timestamp,
                last_updated: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32,
                round_id,
            },
        );
    }

    fn create_transaction_bytes(&self, token: &str, price: u128, timestamp: u32, round_id: u64) -> Vec<u8> {
        // Create a simple transaction structure
        let mut tx = Transaction {
            version: 1,
            lock_time: 0,
            input: vec![],
            output: vec![],
        };

        // Add a  input (in a real scenario, this would be a valid UTXO)
        tx.input.push(bitcoin::TxIn {
            previous_output: bitcoin::OutPoint::null(),
            script_sig: bitcoin::Script::new(),
            sequence: 0xFFFFFFFF,
            witness: vec![],
        });

        // Add an output with the price data encoded in the script
        let script_data = format!("{}:{}:{}:{}", token, price, timestamp, round_id);
        tx.output.push(bitcoin::TxOut {
            value: 0, // No value transfer, just data
            script_pubkey: bitcoin::Script::new_p2pkh(&bitcoin::Address::p2pkh(
                &bitcoin::PublicKey::from_slice(&[0u8; 33]).unwrap(),
                bitcoin::Network::Bitcoin,
            ).script_pubkey().into_bytes()),
        });

        // Serialize the transaction to bytes
        let mut tx_bytes = vec![];
        tx.consensus_encode(&mut tx_bytes).unwrap();
        tx_bytes
    }

    fn only_owner(&self) {

        assert_eq!(self.owner, Pubkey::new_unique(), "Only owner can call this function");
    }
}