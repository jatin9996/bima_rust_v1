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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utxo_creation() {
        let txid = vec![1, 2, 3, 4];
        let vout = 0;
        let value = 100;
        let script = vec![5, 6, 7, 8];

        let utxo = UTXO::new(txid.clone(), vout, value, script.clone());

        assert_eq!(utxo.txid, txid);
        assert_eq!(utxo.vout, vout);
        assert_eq!(utxo.value, value);
        assert_eq!(utxo.script, script);
    }
}
