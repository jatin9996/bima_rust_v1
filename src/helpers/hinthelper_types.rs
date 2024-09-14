use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Trove {
    owner: Vec<u8>,
    debt: u128,
    collateral: u128,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct UTXOAuthority {
    pub key: Vec<u8>,
}
