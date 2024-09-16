use borsh::{BorshDeserialize, BorshSerialize};

pub trait IFactory {
    /// Represents parameters used for deployment.
    #[derive(BorshSerialize, BorshDeserialize)]
    struct DeploymentParams {
        minute_decay_factor: u128, // half life of 12 hours
        redemption_fee_floor: u128, // 0.5%
        max_redemption_fee: u128, // 100%
        borrowing_fee_floor: u128, // 0.5%
        max_borrowing_fee: u128, // 5%
        interest_rate_in_bps: u128, // 1%
        max_debt: u128,
        mcr: u128, // 120%
    }

    /// Emitted when a new deployment is made.
    fn new_deployment(
        collateral: &str,
        price_feed: &str,
        trove_manager: &str,
        sorted_troves: &str
    );

    /// Deploys a new instance with the specified parameters.
    fn deploy_new_instance(
        collateral: &str,
        price_feed: &str,
        custom_trove_manager_impl: &str,
        custom_sorted_troves_impl: &str,
        params: DeploymentParams
    );

    /// Sets the implementations for trove manager and sorted troves.
    fn set_implementations(
        trove_manager_impl: &str,
        sorted_troves_impl: &str
    );

    /// Returns the address of the Babel core.
    fn babel_core(&self) -> &str;

    /// Returns the address of the borrower operations contract.
    fn borrower_operations(&self) -> &str;

    /// Returns the address of the debt token contract.
    fn debt_token(&self) -> &str;

    /// Returns the address of the guardian.
    fn guardian(&self) -> &str;

    /// Returns the address of the liquidation manager.
    fn liquidation_manager(&self) -> &str;

    /// Returns the address of the owner.
    fn owner(&self) -> &str;

    /// Returns the address of the sorted troves implementation.
    fn sorted_troves_impl(&self) -> &str;

    /// Returns the address of the stability pool.
    fn stability_pool(&self) -> &str;

    /// Returns the count of trove managers.
    fn trove_manager_count(&self) -> u128;

    /// Returns the address of the trove manager implementation.
    fn trove_manager_impl(&self) -> &str;

    /// Returns the address of a specific trove manager by index.
    fn trove_managers(&self, index: u128) -> &str;
}
