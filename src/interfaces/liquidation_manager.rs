pub trait ILiquidationManager {
    /// Represents the event when a liquidation occurs.
    fn liquidation_event(
        &self,
        liquidated_debt: u256,
        liquidated_coll: u256,
        coll_gas_compensation: u256,
        debt_gas_compensation: u256,
    );

    /// Represents the event when a trove is liquidated.
    fn trove_liquidated(
        &self,
        borrower: &str,
        debt: u256,
        coll: u256,
        operation: u8,
    );

    /// Represents the event when a trove is updated.
    fn trove_updated(
        &self,
        borrower: &str,
        debt: u256,
        coll: u256,
        stake: u256,
        operation: u8,
    );

    /// Batch liquidates multiple troves.
    fn batch_liquidate_troves(&self, trove_manager: &str, trove_array: &[&str]);

    /// Enables a trove manager.
    fn enable_trove_manager(&self, trove_manager: &str);

    /// Liquidates a specific trove.
    fn liquidate(&self, trove_manager: &str, borrower: &str);

    /// Liquidates multiple troves.
    fn liquidate_troves(&self, trove_manager: &str, max_troves_to_liquidate: u256, max_icr: u256);

    /// Returns the collateral coverage ratio.
    fn ccr(&self) -> u256;

    /// Returns the debt gas compensation.
    fn debt_gas_compensation(&self) -> u256;

    /// Returns the decimal precision used in the contract.
    fn decimal_precision(&self) -> u256;

    /// Returns the percent divisor used in calculations.
    fn percent_divisor(&self) -> u256;

    /// Returns the address of the borrower operations contract.
    fn borrower_operations(&self) -> &str;

    /// Returns the address of the factory contract.
    fn factory(&self) -> &str;

    /// Returns the address of the stability pool contract.
    fn stability_pool(&self) -> &str;
}
