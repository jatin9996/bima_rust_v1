use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct StabilityPool {
    // Define the fields that you want to serialize/deserialize
    pub depositor: String,
    pub collateral: Vec<u256>,
    // Add other fields as necessary
}

pub trait IStabilityPool {
    /// Event triggered when collateral gains are withdrawn.
    fn collateral_gain_withdrawn(&self, depositor: &str, collateral: Vec<u256>);

    /// Event triggered when collateral is overwritten.
    fn collateral_overwritten(&self, old_collateral: &str, new_collateral: &str);

    /// Event triggered when a deposit snapshot is updated.
    fn deposit_snapshot_updated(&self, depositor: &str, p: u256, g: u256);

    /// Event triggered when the epoch is updated.
    fn epoch_updated(&self, current_epoch: u128);

    /// Event triggered when G is updated.
    fn g_updated(&self, g: u256, epoch: u128, scale: u128);

    /// Event triggered when P is updated.
    fn p_updated(&self, p: u256);

    /// Event triggered when a reward is claimed.
    fn reward_claimed(&self, account: &str, recipient: &str, claimed: u256);

    /// Event triggered when S is updated.
    fn s_updated(&self, idx: u256, s: u256, epoch: u128, scale: u128);

    /// Event triggered when the scale is updated.
    fn scale_updated(&self, current_scale: u128);

    /// Event triggered when the stability pool debt balance is updated.
    fn stability_pool_debt_balance_updated(&self, new_balance: u256);

    /// Event triggered when a user's deposit is changed.
    fn user_deposit_changed(&self, depositor: &str, new_deposit: u256);

    /// Claims collateral gains.
    fn claim_collateral_gains(&self, recipient: &str, collateral_indexes: Vec<u256>) -> ();

    /// Claims rewards.
    fn claim_reward(&self, recipient: &str) -> u256;

    /// Enables a collateral token.
    fn enable_collateral(&self, collateral: &str) -> ();

    /// Offsets debt with collateral.
    fn offset(&self, collateral: &str, debt_to_offset: u256, coll_to_add: u256) -> ();

    /// Provides amount to the stability pool.
    fn provide_to_sp(&self, amount: u256) -> ();

    /// Starts the collateral sunset process.
    fn start_collateral_sunset(&self, collateral: &str) -> ();

    /// Claims rewards from the vault.
    fn vault_claim_reward(&self, claimant: &str, address: &str) -> u256;

    /// Withdraws amount from the stability pool.
    fn withdraw_from_sp(&self, amount: u256) -> ();

    /// Returns the decimal precision.
    fn decimal_precision(&self) -> u256;

    /// Returns the value of P.
    fn p(&self) -> u256;

    /// Returns the address of Babel Core.
    fn babel_core(&self) -> &str;

    /// Returns the scale factor.
    fn scale_factor(&self) -> u256;

    /// Returns the sunset duration.
    fn sunset_duration(&self) -> u128;

    /// Returns account deposits.
    fn account_deposits(&self, account: &str) -> (u128, u128); // (amount, timestamp)

    /// Returns claimable rewards.
    fn claimable_reward(&self, depositor: &str) -> u256;

    /// Returns collateral gains by depositor.
    fn collateral_gains_by_depositor(&self, depositor: &str, index: u256) -> u80;

    /// Returns collateral tokens.
    fn collateral_tokens(&self, index: u256) -> &str;

    /// Returns the current epoch.
    fn current_epoch(&self) -> u128;

    /// Returns the current scale.
    fn current_scale(&self) -> u128;

    /// Returns the debt token address.
    fn debt_token(&self) -> &str;

    /// Returns deposit snapshots.
    fn deposit_snapshots(&self, account: &str) -> (u256, u256, u128, u128); // (P, G, scale, epoch)

    /// Returns deposit sums.
    fn deposit_sums(&self, account: &str, index: u256) -> u256;

    /// Returns emission ID.
    fn emission_id(&self) -> u256;

    /// Returns G value for a given epoch and scale.
    fn epoch_to_scale_to_g(&self, epoch: u128, scale: u128) -> u256;

    /// Returns Sums for a given epoch, scale, and index.
    fn epoch_to_scale_to_sums(&self, epoch: u128, scale: u128, index: u256) -> u256;

    /// Returns the factory address.
    fn factory(&self) -> &str;

    /// Returns compounded debt deposit for a depositor.
    fn get_compounded_debt_deposit(&self, depositor: &str) -> u256;

    /// Returns depositor collateral gains.
    fn get_depositor_collateral_gain(&self, depositor: &str) -> Vec<u256>;

    /// Returns total debt token deposits.
    fn get_total_debt_token_deposits(&self) -> u256;

    /// Returns the current week.
    fn get_week(&self) -> u256;

    /// Returns the guardian address.
    fn guardian(&self) -> &str;

    /// Returns the index by collateral.
    fn index_by_collateral(&self, collateral: &str) -> u256;

    /// Returns the last collateral error offset.
    fn last_collateral_error_offset(&self) -> u256;

    /// Returns the last debt loss error offset.
    fn last_debt_loss_error_offset(&self) -> u256;

    /// Returns the last Babel error.
    fn last_babel_error(&self) -> u256;

    /// Returns the last update timestamp.
    fn last_update(&self) -> u32;

    /// Returns the liquidation manager address.
    fn liquidation_manager(&self) -> &str;

    /// Returns the owner address.
    fn owner(&self) -> &str;

    /// Returns the period finish timestamp.
    fn period_finish(&self) -> u32;

    /// Returns the reward rate.
    fn reward_rate(&self) -> u128;

    /// Returns the vault address.
    fn vault(&self) -> &str;
}
