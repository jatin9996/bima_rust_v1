use borsh::{BorshSerialize, BorshDeserialize};

pub fn verify_utxo_authority(utxo: &UTXO, pubkey: &Pubkey) -> bool {
    // Verify UTXO authority
}

pub fn create_new_utxo(data: &[u8], value: u64) -> UTXO {
    // Create a new UTXO
}
