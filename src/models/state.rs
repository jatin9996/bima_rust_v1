use borsh::{BorshSerialize, BorshDeserialize};
use std::collections::VecDeque;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct EmissionState {
    owner: Vec<u8>,  // Public key of the owner
    system_start: u64,
    vault: Vec<u8>,
    voter: Vec<u8>,
    lock_weeks: u64,
    lock_decay_weeks: u64,
    weekly_pct: u64,
    scheduled_weekly_pct: VecDeque<(u64, u64)>,
}
