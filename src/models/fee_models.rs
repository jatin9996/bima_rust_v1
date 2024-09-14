use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Token {
    pub balances: Vec<u64>,
    pub allowances: Vec<u64>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct UtxoInfo {
    pub txid: String,
    pub output_index: u32,
    pub value: u64,
}
