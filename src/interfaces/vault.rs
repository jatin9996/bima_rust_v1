use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitialAllowance {
    pub receiver: String, 
    pub amount: u256,
}

// Define the trait for IBabelVault
pub trait IBabelVault {
    // Method to allocate new emissions
    fn allocate_new_emissions(&self, id: u256) -> Result<u256, String>;
    
    // Method to batch claim rewards
    fn batch_claim_rewards(
        &self,
        receiver: String,
        boost_delegate: String,
        reward_contracts: Vec<String>,
        max_fee_pct: u256
    ) -> Result<bool, String>;
    
    // Method to increase unallocated supply
    fn increase_unallocated_supply(&self, amount: u256) -> Result<bool, String>;
    
    // Method to register a new receiver
    fn register_receiver(&self, receiver: String, count: u256) -> Result<bool, String>;
    
    // Method to set the boost calculator address
    fn set_boost_calculator(&self, boost_calculator: String) -> Result<bool, String>;
    
    // Method to set boost delegation parameters
    fn set_boost_delegation_params(&self, is_enabled: bool, fee_pct: u256, callback: String) -> Result<bool, String>;
    
    // Method to set emission schedule address
    fn set_emission_schedule(&self, emission_schedule: String) -> Result<bool, String>;
    
    // Method to set initial parameters
    fn set_initial_parameters(
        &self,
        emission_schedule: String,
        boost_calculator: String,
        total_supply: u256,
        initial_lock_weeks: u64,
        fixed_initial_amounts: Vec<u128>,
        initial_allowances: Vec<InitialAllowance>
    ) -> Result<(), String>;
    
    // Method to set receiver's active status
    fn set_receiver_is_active(&self, id: u256, is_active: bool) -> Result<bool, String>;
    
    // Method to transfer allocated tokens
    fn transfer_allocated_tokens(&self, claimant: String, receiver: String, amount: u256) -> Result<bool, String>;
    
    // Method to transfer tokens
    fn transfer_tokens(&self, token: String, receiver: String, amount: u256) -> Result<bool, String>;
    
    // View methods
    fn babel_core(&self) -> Result<String, String>;
    
    fn allocated(&self, account: String) -> Result<u256, String>;
    
    fn boost_calculator(&self) -> Result<String, String>;
    
    fn boost_delegation(&self, account: String) -> Result<(bool, u16, String), String>;
    
    fn claimable_reward_after_boost(
        &self,
        account: String,
        receiver: String,
        boost_delegate: String,
        reward_contract: String
    ) -> Result<(u256, u256), String>;
  
    fn emission_schedule(&self) -> Result<String, String>;
    
    fn get_claimable_with_boost(&self, claimant: String) -> Result<(u256, u256), String>;
    
    fn get_week(&self) -> Result<u256, String>;
    
    fn guardian(&self) -> Result<String, String>;
    
    fn id_to_receiver(&self, id: u256) -> Result<(String, bool), String>;
    
    fn lock_weeks(&self) -> Result<u64, String>;
    
    fn locker(&self) -> Result<String, String>;
    
    fn owner(&self) -> Result<String, String>;
    
    fn claimable_boost_delegation_fees(&self, claimant: String) -> Result<u256, String>;
    
    fn babel_token(&self) -> Result<String, String>;
    
    fn receiver_updated_week(&self, id: u256) -> Result<u16, String>;
    
    fn total_update_week(&self) -> Result<u64, String>;
    
    fn unallocated_total(&self) -> Result<u128, String>;
    
    fn voter(&self) -> Result<String, String>;
    
    fn weekly_emissions(&self, id: u256) -> Result<u128, String>;
}

// Define u256 as an 
// In a real application, you'd need to use a specific library for handling large integers or define your own
pub type u256 = [u8; 32];
