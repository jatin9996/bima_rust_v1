#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use crate::utxo::utxo::UTXO;
use ink_storage::collections::HashMap as StorageMap;

#[ink::contract]
mod utxo_storage {
    #[ink(storage)]
    pub struct UtxoStorage {
        utxos: StorageMap<(Vec<u8>, u32), UTXO>,
    }

    impl UtxoStorage {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                utxos: StorageMap::default(),
            }
        }

        #[ink(message)]
        pub fn store_utxo(&mut self, txid: Vec<u8>, vout: u32, utxo: UTXO) {
            self.utxos.insert((txid, vout), utxo);
        }

        #[ink(message)]
        pub fn remove_utxo(&mut self, txid: Vec<u8>, vout: u32) {
            self.utxos.take(&(txid, vout));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_interim_admin() {
        let babel_core = "babel_core".to_string();
        let owner = "owner".to_string();
        let admin = InterimAdmin::new(babel_core.clone(), owner.clone());

        assert_eq!(admin.babel_core, babel_core);
        assert_eq!(admin.owner, owner);
    }

    #[test]
    fn test_set_guardian() {
        let mut admin = InterimAdmin::new("babel_core".to_string(), "owner".to_string());
        admin.set_guardian("owner", "guardian".to_string());

        assert_eq!(admin.guardian, Some("guardian".to_string()));
    }

    #[test]
    fn test_create_new_proposal() {
        let mut admin = InterimAdmin::new("babel_core".to_string(), "owner".to_string());
        let payload = vec![Action { target: "target".to_string(), data: vec![1, 2, 3] }];

        admin.create_new_proposal("owner", payload); // Implement the logic for this method...

        // Assert the proposal was created successfully...
    }

    // Add more tests for other methods...
}