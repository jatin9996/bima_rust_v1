#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod trovemanager_getter {
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::SpreadAllocate,
    };

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct TroveManagerGetters {
        trove_managers: StorageHashMap<u32, String>,
        trove_to_collateral: StorageHashMap<String, String>,
        trove_status: StorageHashMap<(String, String), i32>,
    }

    impl TroveManagerGetters {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.trove_to_collateral.insert("trove_manager_1".to_string(), "ETH".to_string());
                contract.trove_to_collateral.insert("trove_manager_2".to_string(), "BTC".to_string());
                contract.trove_to_collateral.insert("trove_manager_3".to_string(), "DAI".to_string());
                contract.trove_status.insert(("trove_manager_1".to_string(), "account_1".to_string()), 1);
                contract.trove_status.insert(("trove_manager_2".to_string(), "account_2".to_string()), 0);
                contract.trove_status.insert(("trove_manager_3".to_string(), "account_3".to_string()), 1);
            })
        }

        #[ink(message)]
        pub fn get_collateral_token(&self, trove_manager: String) -> String {
            self.trove_to_collateral.get(&trove_manager).cloned().unwrap_or_else(|| "Unknown".to_string())
        }

        #[ink(message)]
        pub fn get_trove_status(&self, trove_manager: String, account: String) -> i32 {
            *self.trove_status.get(&(trove_manager, account)).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn get_active_trove_managers_for_account(&self, account: String) -> Vec<String> {
            let mut active_managers = Vec::new();
            for (key, value) in self.trove_status.iter() {
                if key.1 == account && *value > 0 {
                    active_managers.push(key.0.clone());
                }
            }
            active_managers
        }
    }
}