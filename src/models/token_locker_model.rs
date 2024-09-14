use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct AccountData {
    pub locked: u64,
    pub unlocked: u64,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct UtxoInfo {
    pub txid: Vec<u8>,
    pub output_index: u32,
    pub data: Vec<u8>,
}
