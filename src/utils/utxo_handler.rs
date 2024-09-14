use borsh::{BorshSerialize, BorshDeserialize};
use crate::models::{Utxo, AuthorityMessage};
use arch_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct UtxoHandler;

impl UtxoHandler {
    pub fn create_utxo(&self, data: Vec<u8>, authority: Pubkey) -> Utxo {
        // Logic to create a new UTXO
    }

    pub fn update_utxo(&self, utxo_id: String, new_data: Vec<u8>) {
        // Logic to update an existing UTXO
    }

    pub fn delete_utxo(&self, utxo_id: String) {
        // Logic to delete an UTXO
    }
}
