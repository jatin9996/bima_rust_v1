use borsh::{BorshSerialize, BorshDeserialize};
use serde::{Serialize, Deserialize};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
pub struct Trove {
    debt: u64,
    coll: u64,
    stake: u64,
    status: Status,
    array_index: u32,
    active_interest_index: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Status {
    NonExistent,
    Active,
    ClosedByOwner,
    ClosedByLiquidation,
    ClosedByRedemption,
}
