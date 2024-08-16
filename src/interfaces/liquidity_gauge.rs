pub trait ILiquidityGauge {
    /// Deposits `amount` of liquidity into the gauge for the specified `receiver`.
    fn deposit(&self, amount: u256, receiver: &str);

    /// Withdraws `value` of liquidity from the gauge.
    fn withdraw(&self, value: u256);

    /// Returns the address of the LP token associated with the gauge.
    fn lp_token(&self) -> &str;

    /// Sets the approval status for a `depositor` to allow or disallow deposits.
    fn set_approve_deposit(&self, depositor: &str, can_deposit: bool);

    /// Sets the address of the rewards receiver.
    fn set_rewards_receiver(&self, receiver: &str);
}
