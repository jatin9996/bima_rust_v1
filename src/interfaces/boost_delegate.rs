use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct BoostDelegateData {
    claimant: String,
    receiver: String,
    amount: u128,
    previous_amount: u128,
    total_weekly_emissions: u128,
}

pub trait BoostDelegate {
    /// Get the current fee percent charged to use this boost delegate.
    /// 
    /// # Parameters
    /// - `data`: BoostDelegateData containing all necessary parameters.
    /// 
    /// # Returns
    /// Fee percentage charged for claims that use this contract's delegated boost.
    /// Given as a whole number out of 10000. If a claim would be rejected,
    /// the preferred return value is `u128::MAX`.
    fn get_fee_pct(&self, data: &BoostDelegateData) -> u128;

    /// Callback function for boost delegators.
    /// 
    /// # Parameters
    /// - `data`: BoostDelegateData containing all necessary parameters.
    /// - `adjusted_amount`: Actual amount received by `claimant`.
    /// - `fee`: Fee amount paid by `claimant`.
    /// 
    /// # Returns
    /// Boolean indicating whether the callback was successful.
    fn delegated_boost_callback(
        &self,
        data: &BoostDelegateData,
        adjusted_amount: u128,
        fee: u128,
    ) -> bool;
}
