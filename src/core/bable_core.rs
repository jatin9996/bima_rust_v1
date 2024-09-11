#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use crate::utxo::UTXO;
use ink_storage::collections::HashMap as StorageMap;
use crate::utxo_module::UtxoContract;

#[ink::contract]
mod bable_core {
    use ink_storage::collections::HashMap;
    use ink_prelude::string::String;
    use ink_env::block_timestamp;

    const OWNERSHIP_TRANSFER_DELAY: u64 = 86400 * 3; // 3 days

    #[ink(storage)]
    pub struct BabelCore {
        utxos: StorageMap<(Vec<u8>, u32), UTXO>,
        fee_receiver: String,
        price_feed: String,
        owner: String,
        pending_owner: Option<String>,
        ownership_transfer_deadline: Option<u64>,
        guardian: String,
        paused: bool,
        start_time: u64,
    }

    impl BabelCore {
        #[ink(constructor)]
        pub fn new(owner: String, guardian: String, price_feed: String, fee_receiver: String) -> Self {
            let start_time = Self::env().block_timestamp() / 1000; // Convert to seconds
            Self {
                utxos: StorageMap::default(),
                fee_receiver,
                price_feed,
                owner,
                pending_owner: None,
                ownership_transfer_deadline: None,
                guardian,
                paused: false,
                start_time: start_time - (start_time % (7 * 86400)), // Rounded down to the nearest week
            }
        }

        #[ink(message)]
        pub fn set_fee_receiver(&mut self, new_fee_receiver: String) {
            self.fee_receiver = new_fee_receiver;
        }

        #[ink(message)]
        pub fn set_price_feed(&mut self, new_price_feed: String) {
            self.price_feed = new_price_feed;
        }

        #[ink(message)]
        pub fn set_guardian(&mut self, new_guardian: String) {
            self.guardian = new_guardian;
        }

        #[ink(message)]
        pub fn set_paused(&mut self, new_paused: bool) {
            if new_paused && self.guardian != self.owner {
                panic!("Unauthorized");
            }
            self.paused = new_paused;
        }

        #[ink(message)]
        pub fn commit_transfer_ownership(&mut self, new_owner: String) {
            self.pending_owner = Some(new_owner);
            self.ownership_transfer_deadline = Some(Self::env().block_timestamp() / 1000 + OWNERSHIP_TRANSFER_DELAY);
        }

        #[ink(message)]
        pub fn accept_transfer_ownership(&mut self) {
            if let Some(pending_owner) = &self.pending_owner {
                if Self::env().block_timestamp() / 1000 >= self.ownership_transfer_deadline.unwrap() {
                    self.owner = pending_owner.clone();
                    self.pending_owner = None;
                    self.ownership_transfer_deadline = None;
                } else {
                    panic!("Deadline not passed");
                }
            } else {
                panic!("No pending owner");
            }
        }

        #[ink(message)]
        pub fn revoke_transfer_ownership(&mut self) {
            self.pending_owner = None;
            self.ownership_transfer_deadline = None;
        }

        //  method adjustment for transferring using UTXOs
        #[ink(message)]
        pub fn transfer_utxo(&mut self, input_utxos: Vec<(Vec<u8>, u32)>, output_utxos: Vec<UTXO>) {
            // Validate that all input UTXOs are unspent and collect their total value
            let mut input_value = 0;
            for (txid, vout) in input_utxos.iter() {
                let utxo = self.utxos.get(&(*txid, *vout)).expect("UTXO not found");
                input_value += utxo.value;
                // Mark UTXO as spent by removing it
                self.utxos.take(&(*txid, *vout));
            }

            // Validate and create new UTXOs
            let mut output_value = 0;
            for utxo in output_utxos.iter() {
                output_value += utxo.value;
                let txid = utxo.txid.clone(); // Assume txid is generated elsewhere
                let vout = utxo.vout;
                self.utxos.insert((txid, vout), utxo.clone());
            }

            // Ensure no value is created or destroyed
            if input_value != output_value {
                panic!("Input and output values do not match");
            }
        }

        // Use UTXO methods for transactions
        pub fn handle_utxo_transaction(&mut self, input_utxos: Vec<(Vec<u8>, u32)>, output_utxos: Vec<UTXO>) {
            let mut utxo_contract = UtxoContract::new();
            utxo_contract.transfer_utxo(input_utxos, output_utxos);
        }
    }
}