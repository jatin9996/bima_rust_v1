use borsh::{BorshSerialize, BorshDeserialize};
use bitcoin::hashes::sha256;
use bitcoin::secp256k1::PublicKey;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct AllocationSplit {
    recipient: PublicKey,
    points: u32,
    number_of_weeks: u8,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct AllocationState {
    points: u32,
    number_of_weeks: u8,
    claimed: u64,  // Using u64 to represent satoshis
    preclaimed: u64,
}
