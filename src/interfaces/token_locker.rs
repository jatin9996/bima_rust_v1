use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct LockData {
    pub amount: u256,
    pub weeks_to_unlock: u256,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct ExtendLockData {
    pub amount: u256,
    pub current_weeks: u256,
    pub new_weeks: u256,
}

// Define a trait for ITokenLocker
pub trait ITokenLocker {
    fn extend_lock(&self, amount: u256, weeks: u256, new_weeks: u256) -> Result<(), String>;
    
    fn extend_many(&self, new_extend_locks: Vec<ExtendLockData>) -> Result<(), String>;
    
    fn freeze(&self) -> Result<(), String>;
    
    fn get_account_weight_write(&self, account: String) -> Result<u256, String>;
    
    fn get_total_weight_write(&self) -> Result<u256, String>;
    
    fn lock(&self, account: String, amount: u256, weeks: u256) -> Result<(), String>;
    
    fn lock_many(&self, account: String, new_locks: Vec<LockData>) -> Result<(), String>;
    
    fn set_penalty_withdrawals_enabled(&self, enabled: bool) -> Result<(), String>;
    
    fn unfreeze(&self, keep_incentives_vote: bool) -> Result<(), String>;
    
    fn withdraw_expired_locks(&self, weeks: u256) -> Result<(), String>;
    
    fn withdraw_with_penalty(&self, amount_to_withdraw: u256) -> Result<(u256, u256), String>;
    
    fn max_lock_weeks(&self) -> Result<u256, String>;
    
    fn babel_core(&self) -> Result<String, String>;
    
    fn get_account_active_locks(&self, account: String, min_weeks: u256) -> Result<(Vec<LockData>, u256), String>;
    
    fn get_account_balances(&self, account: String) -> Result<(u256, u256), String>;
    
    fn get_account_weight(&self, account: String) -> Result<u256, String>;
    
    fn get_account_weight_at(&self, account: String, week: u256) -> Result<u256, String>;
    
    fn get_total_weight(&self) -> Result<u256, String>;
    
    fn get_total_weight_at(&self, week: u256) -> Result<u256, String>;
    
    fn get_week(&self) -> Result<u256, String>;
    
    fn get_withdraw_with_penalty_amounts(&self, account: String, amount_to_withdraw: u256) -> Result<(u256, u256), String>;
    
    fn guardian(&self) -> Result<String, String>;
    
    fn incentive_voter(&self) -> Result<String, String>;
    
    fn lock_to_token_ratio(&self) -> Result<u256, String>;
    
    fn lock_token(&self) -> Result<String, String>;
    
    fn owner(&self) -> Result<String, String>;
    
    fn penalty_withdrawals_enabled(&self) -> Result<bool, String>;
    
    fn babel_core(&self) -> Result<String, String>;
    
    fn total_decay_rate(&self) -> Result<u256, String>;
    
    fn total_updated_week(&self) -> Result<u256, String>;
}
