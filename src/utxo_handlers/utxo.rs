#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod utxo {
    #[ink(storage)]
    pub struct UTXO {
        txid: Vec<u8>,
        vout: u32,
        value: u64,
        script: Vec<u8>,  // Script to define spending conditions
    }

    impl UTXO {
        #[ink(constructor)]
        pub fn new(txid: Vec<u8>, vout: u32, value: u64, script: Vec<u8>) -> Self {
            Self { txid, vout, value, script }
        }

        // Additional methods related to UTXO can be added here
    }
}
