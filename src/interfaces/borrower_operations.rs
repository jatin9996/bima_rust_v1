pub trait BorrowerOperations {   
    /// Structure representing balances, including collaterals, debts, and prices.
    struct Balances {
        collaterals: Vec<u128>,
        debts: Vec<u128>,
        prices: Vec<u128>,
    }

    /// Event emitted when a borrowing fee is paid.
    fn emit_borrowing_fee_paid(borrower: &str, amount: u128);

    /// Event emitted when collateral is configured.
    fn emit_collateral_configured(trove_manager: &str, collateral_token: &str);

    /// Event emitted when a new trove is created.
    fn emit_trove_created(borrower: &str, array_index: u128);

    /// Event emitted when a trove manager is removed.
    fn emit_trove_manager_removed(trove_manager: &str);

    /// Event emitted when a trove is updated.
    fn emit_trove_updated(borrower: &str, debt: u128, coll: u128, stake: u128, operation: u8);

    /// Add collateral to a trove.
    ///
    /// # Parameters
    /// - `trove_manager`: Address of the trove manager.
    /// - `account`: Address of the borrower.
    /// - `collateral_amount`: Amount of collateral to add.
    /// - `upper_hint`: Address for the upper hint.
    /// - `lower_hint`: Address for the lower hint.
    fn add_coll(
        &self,
        trove_manager: &str,
        account: &str,
        collateral_amount: u128,
        upper_hint: &str,
        lower_hint: &str
    );

    /// Adjust a trove.
    ///
    /// # Parameters
    /// - `trove_manager`: Address of the trove manager.
    /// - `account`: Address of the borrower.
    /// - `max_fee_percentage`: Maximum fee percentage.
    /// - `coll_deposit`: Collateral deposit amount.
    /// - `coll_withdrawal`: Collateral withdrawal amount.
    /// - `debt_change`: Change in debt amount.
    /// - `is_debt_increase`: Whether the debt is increasing.
    /// - `upper_hint`: Address for the upper hint.
    /// - `lower_hint`: Address for the lower hint.
    fn adjust_trove(
        &self,
        trove_manager: &str,
        account: &str,
        max_fee_percentage: u128,
        coll_deposit: u128,
        coll_withdrawal: u128,
        debt_change: u128,
        is_debt_increase: bool,
        upper_hint: &str,
        lower_hint: &str
    );

    /// Close a trove.
    ///
    /// # Parameters
    /// - `trove_manager`: Address of the trove manager.
    /// - `account`: Address of the borrower.
    fn close_trove(
        &self,
        trove_manager: &str,
        account: &str
    );

    /// Configure collateral for a trove.
    ///
    /// # Parameters
    /// - `trove_manager`: Address of the trove manager.
    /// - `collateral_token`: Address of the collateral token.
    fn configure_collateral(
        &self,
        trove_manager: &str,
        collateral_token: &str
    );

    /// Fetch balances.
    ///
    /// # Returns
    /// A `Balances` structure containing the current balances.
    fn fetch_balances(&self) -> Balances;

    /// Get global system balances.
    ///
    /// # Returns
    /// A tuple containing total priced collateral and total debt.
    fn get_global_system_balances(&self) -> (u128, u128);

    /// Get the total collateral ratio.
    ///
    /// # Returns
    /// Global total collateral ratio.
    fn get_tcr(&self) -> u128;

    /// Open a new trove.
    ///
    /// # Parameters
    /// - `trove_manager`: Address of the trove manager.
    /// - `account`: Address of the borrower.
    /// - `max_fee_percentage`: Maximum fee percentage.
    /// - `collateral_amount`: Amount of collateral.
    /// - `debt_amount`: Amount of debt.
    /// - `upper_hint`: Address for the upper hint.
    /// - `lower_hint`: Address for the lower hint.
    fn open_trove(
        &self,
        trove_manager: &str,
        account: &str,
        max_fee_percentage: u128,
        collateral_amount: u128,
        debt_amount: u128,
        upper_hint: &str,
        lower_hint: &str
    );

    /// Remove a trove manager.
    ///
    /// # Parameters
    /// - `trove_manager`: Address of the trove manager.
    fn remove_trove_manager(
        &self,
        trove_manager: &str
    );

    /// Repay debt for a trove.
    ///
    /// # Parameters
    /// - `trove_manager`: Address of the trove manager.
    /// - `account`: Address of the borrower.
    /// - `debt_amount`: Amount of debt to repay.
    /// - `upper_hint`: Address for the upper hint.
    /// - `lower_hint`: Address for the lower hint.
    fn repay_debt(
        &self,
        trove_manager: &str,
        account: &str,
        debt_amount: u128,
        upper_hint: &str,
        lower_hint: &str
    );

    /// Set approval for a delegate.
    ///
    /// # Parameters
    /// - `delegate`: Address of the delegate.
    /// - `is_approved`: Whether the delegate is approved.
    fn set_delegate_approval(
        &self,
        delegate: &str,
        is_approved: bool
    );

    /// Set the minimum net debt.
    ///
    /// # Parameters
    /// - `min_net_debt`: Minimum net debt amount.
    fn set_min_net_debt(
        &self,
        min_net_debt: u128
    );

    /// Withdraw collateral from a trove.
    ///
    /// # Parameters
    /// - `trove_manager`: Address of the trove manager.
    /// - `account`: Address of the borrower.
    /// - `coll_withdrawal`: Amount of collateral to withdraw.
    /// - `upper_hint`: Address for the upper hint.
    /// - `lower_hint`: Address for the lower hint.
    fn withdraw_coll(
        &self,
        trove_manager: &str,
        account: &str,
        coll_withdrawal: u128,
        upper_hint: &str,
        lower_hint: &str
    );

    /// Withdraw debt from a trove.
    ///
    /// # Parameters
    /// - `trove_manager`: Address of the trove manager.
    /// - `account`: Address of the borrower.
    /// - `max_fee_percentage`: Maximum fee percentage.
    /// - `debt_amount`: Amount of debt to withdraw.
    /// - `upper_hint`: Address for the upper hint.
    /// - `lower_hint`: Address for the lower hint.
    fn withdraw_debt(
        &self,
        trove_manager: &str,
        account: &str,
        max_fee_percentage: u128,
        debt_amount: u128,
        upper_hint: &str,
        lower_hint: &str
    );

    /// Check if recovery mode is active based on the total collateral ratio.
    ///
    /// # Parameters
    /// - `tcr`: Total collateral ratio.
    ///
    /// # Returns
    /// Whether recovery mode is active.
    fn check_recovery_mode(&self, tcr: u128) -> bool;

    /// Get the collateral coverage ratio.
    ///
    /// # Returns
    /// Collateral coverage ratio.
    fn ccr(&self) -> u128;

    /// Get the debt gas compensation.
    ///
    /// # Returns
    /// Debt gas compensation amount.
    fn debt_gas_compensation(&self) -> u128;

    /// Get the decimal precision.
    ///
    /// # Returns
    /// Decimal precision amount.
    fn decimal_precision(&self) -> u128;

    /// Get the percent divisor.
    ///
    /// # Returns
    /// Percent divisor amount.
    fn percent_divisor(&self) -> u128;

    /// Get the Babel core address.
    ///
    /// # Returns
    /// Babel core address.
    fn babel_core(&self) -> &str;

    /// Get the 100 percent value.
    ///
    /// # Returns
    /// 100 percent value.
    fn hundred_pct(&self) -> u128;

    /// Get the debt token address.
    ///
    /// # Returns
    /// Debt token address.
    fn debt_token(&self) -> &str;

    /// Get the factory address.
    ///
    /// # Returns
    /// Factory address.
    fn factory(&self) -> &str;

    /// Get the composite debt amount.
    ///
    /// # Parameters
    /// - `debt`: Debt amount.
    ///
    /// # Returns
    /// Composite debt amount.
    fn get_composite_debt(&self, debt: u128) -> u128;

    /// Get the guardian address.
    ///
    /// # Returns
    /// Guardian address.
    fn guardian(&self) -> &str;

    /// Check if a delegate is approved.
    ///
    /// # Parameters
    /// - `owner`: Address of the owner.
    /// - `caller`: Address of the caller.
    ///
    /// # Returns
    /// Whether the delegate is approved.
    fn is_approved_delegate(&self, owner: &str, caller: &str) -> bool;

    /// Get the minimum net debt.
    ///
    /// # Returns
    /// Minimum net debt amount.
    fn min_net_debt(&self) -> u128;

    /// Get the owner address.
    ///
    /// # Returns
    /// Owner address.
    fn owner(&self) -> &str;

    /// Get trove managers data.
    ///
    /// # Parameters
    /// - `address`: Address of the trove manager.
    ///
    /// # Returns
    /// Tuple containing collateral token address and index.
    fn trove_managers_data(&self, address: &str) -> (String, u16);
}
