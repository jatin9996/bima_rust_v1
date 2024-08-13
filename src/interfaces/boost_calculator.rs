pub trait BoostCalculator {
    fn get_boosted_amount_write(
        &mut self,
        account: &str,
        amount: u128,
        previous_amount: u128,
        total_weekly_emissions: u128,
    ) -> u128;

    fn max_boost_grace_weeks(&self) -> u128;

    fn get_boosted_amount(
        &self,
        account: &str,
        amount: u128,
        previous_amount: u128,
        total_weekly_emissions: u128,
    ) -> u128;

    fn get_claimable_with_boost(
        &self,
        claimant: &str,
        previous_amount: u128,
        total_weekly_emissions: u128,
    ) -> (u128, u128);

    fn get_week(&self) -> u128;

    fn locker(&self) -> &str;
}
