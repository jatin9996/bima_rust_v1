#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use crate::utxo::utxo::UTXO;
use ink_storage::collections::HashMap as StorageMap;
use crate::core::utxo_module::UtxoContract;
use crate::dao::token_locker::TokenLocker;

#[ink::contract]
mod transaction {
    #[ink(storage)]
    pub struct Transaction {
        inputs: Vec<UTXO>,
        outputs: Vec<UTXO>,
    }

    impl Transaction {
        #[ink(constructor)]
        pub fn new(inputs: Vec<UTXO>, outputs: Vec<UTXO>) -> Self {
            Self { inputs, outputs }
        }

        #[ink(message)]
        pub fn validate_transaction(&self) -> bool {
            // Check if the sum of inputs equals the sum of outputs
            let input_sum: u128 = self.inputs.iter().map(|utxo| utxo.value).sum();
            let output_sum: u128 = self.outputs.iter().map(|utxo| utxo.value).sum();

            input_sum == output_sum
        }

        // Broadcasts the transaction to the network
        #[ink(message)]
        pub fn broadcast_transaction(&self) {
            // Convert transaction data to bytes
            let transaction_data = self.encode_transaction();
            
            // Assuming `NetworkContract` is a contract that can broadcast transactions
            // and it has been instantiated and available as `network_contract`
            let network_contract: NetworkContract = self.get_network_contract();

            // Call the broadcast method on the network contract
            network_contract.broadcast(transaction_data);
        }

        // Checks if the specified account is the owner of the UTXO
        #[ink(message)]
        pub fn check_utxo_ownership(&self, account_id: AccountId, utxo: &UTXO) -> bool {
            // Check if the account_id matches the owner field in the UTXO
            utxo.owner == account_id
        }

        // Locks a UTXO by interacting with the DAO's token locker
        #[ink(message)]
        pub fn lock_utxo(&mut self, utxo: &UTXO, locker: AccountId, period: u64) {
            // Assuming there's a method in TokenLocker to lock tokens based on UTXO
            let token_locker = TokenLocker::new();
            token_locker.lock_tokens_based_on_utxo(utxo, locker, period);
        }

        // Helper method to get an instance of the network contract
        fn get_network_contract(&self) -> NetworkContract {
            //  for obtaining a network contract instance
            // This might involve fetching a contract address from a registry or configuration
            // and then instantiating the contract proxy with that address
            NetworkContract::new()
        }

        // Helper method to encode the transaction into bytes
        fn encode_transaction(&self) -> Vec<u8> {
            // Encoding the transaction to bytes (using SCALE codec as an example)
            let mut encoded = Vec::new();
            for input in &self.inputs {
                input.encode_to(&mut encoded);
            }
            for output in &self.outputs {
                output.encode_to(&mut encoded);
            }
            encoded
        }
    }
}
