use crate::models::{LockData, ExtendLockData};
use arch_program::pubkey::Pubkey;

pub trait ITokenLocker {
    fn extend_lock(&self, amount: u64, weeks: u32, new_weeks: u32) -> bool;
    fn lock(&self, account: Pubkey, amount: u64, weeks: u32) -> bool;
    // Additional methods as per the Solidity interface
}
