pub trait BoostDelegate {
    /// Get the current fee percent charged to use this boost delegate.
    /// 
    /// # Parameters
    /// - `claimant`: Address that will perform the claim.
    /// - `receiver`: Address that will receive the claim.
    /// - `amount`: Amount to be claimed (before applying boost or fee).
    /// - `previous_amount`: Previous amount claimed this week by this contract.
    /// - `total_weekly_emissions`: Total weekly emissions released this week.
    /// 
    /// # Returns
    /// Fee percentage charged for claims that use this contract's delegated boost.
    /// Given as a whole number out of 10000. If a claim would be rejected,
    /// the preferred return value is `u128::MAX`.
    fn get_fee_pct(
        &self,
        claimant: &str,
        receiver: &str,
        amount: u128,
        previous_amount: u128,
        total_weekly_emissions: u128,
    ) -> u128;

    /// Callback function for boost delegators.
    /// 
    /// # Parameters
    /// - `claimant`: Address that performed the claim.
    /// - `receiver`: Address that received the claim.
    /// - `amount`: Amount that was claimed (before applying boost or fee).
    /// - `adjusted_amount`: Actual amount received by `claimant`.
    /// - `fee`: Fee amount paid by `claimant`.
    /// - `previous_amount`: Previous amount claimed this week by this contract.
    /// - `total_weekly_emissions`: Total weekly emissions released this week.
    /// 
    /// # Returns
    /// Boolean indicating whether the callback was successful.
    fn delegated_boost_callback(
        &self,
        claimant: &str,
        receiver: &str,
        amount: u128,
        adjusted_amount: u128,
        fee: u128,
        previous_amount: u128,
        total_weekly_emissions: u128,
    ) -> bool;
}
