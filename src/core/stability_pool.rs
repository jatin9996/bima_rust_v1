use borsh::{BorshSerialize, BorshDeserialize};
use crate::models::params::{DepositParams, WithdrawParams, InterestParams};
use crate::utils::utxo_utils::{validate_utxo_ownership, create_new_utxo};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct StabilityPool {
    pub total_stablecoins: u64,
    pub owner: Pubkey,
}

impl StabilityPool {
    pub fn deposit(params: DepositParams) -> Result<(), String> {
        validate_utxo_ownership(params.utxo_id, params.caller_pubkey)?;
        // Logic to add funds to the pool
        create_new_utxo(params.amount, params.caller_pubkey)?;
        Ok(())
    }

    pub fn withdraw(params: WithdrawParams) -> Result<(), String> {
        validate_utxo_ownership(params.utxo_id, params.caller_pubkey)?;
        // Logic to remove funds from the pool
        create_new_utxo(params.amount, params.caller_pubkey)?;
        Ok(())
    }

    pub fn calculate_interest(params: InterestParams) -> u64 {
        // Interest calculation logic
        params.collateral_amount * 5 / 100
    }
}
