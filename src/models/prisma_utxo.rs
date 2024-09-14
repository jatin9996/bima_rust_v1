use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct StateUTXO {
    pub owner_pubkey: Vec<u8>,
    pub data: Vec<u8>,
}

impl StateUTXO {
    pub fn new(owner_pubkey: Vec<u8>, data: Vec<u8>) -> Self {
        Self {
            owner_pubkey,
            data,
        }
    }
}
