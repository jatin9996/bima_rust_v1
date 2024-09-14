use borsh::{BorshSerialize, BorshDeserialize};
use crate::models::utxo::StateUTXO;
use crate::utils::serialization::{serialize, deserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct TokenContract {
    pub vault_pubkey: Vec<u8>,
    pub locker_pubkey: Vec<u8>,
}

impl TokenContract {
    pub fn new(vault_pubkey: Vec<u8>, locker_pubkey: Vec<u8>) -> Self {
        Self {
            vault_pubkey,
            locker_pubkey,
        }
    }

    pub fn mint(&self, amount: u128) -> StateUTXO {
        // Logic to mint new tokens and return a new State UTXO
    }

    pub fn transfer(&self, from_utxo: StateUTXO, to_pubkey: Vec<u8>, amount: u128) -> (StateUTXO, StateUTXO) {
        // Logic to transfer tokens between UTXOs
    }
}
