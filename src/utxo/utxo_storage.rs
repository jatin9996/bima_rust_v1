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
