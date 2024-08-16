pub trait EmissionSchedule {
    /// Event emitted when lock parameters are set.
    fn emit_lock_parameters_set(lock_weeks: u128, lock_decay_weeks: u128);

    /// Event emitted when the weekly percentage schedule is set.
    fn emit_weekly_pct_schedule_set(schedule: &[(u64, u64)]);

    /// Get the weekly emissions for a receiver.
    ///
    /// # Parameters
    /// - `id`: Identifier of the receiver.
    /// - `week`: Week number for the emission calculation.
    /// - `total_weekly_emissions`: Total emissions available for the week.
    ///
    /// # Returns
    /// Amount of emissions for the receiver.
    fn get_receiver_weekly_emissions(
        &self,
        id: u128,
        week: u128,
        total_weekly_emissions: u128
    ) -> u128;

    /// Get the total weekly emissions.
    ///
    /// # Parameters
    /// - `week`: Week number for the emission calculation.
    /// - `unallocated_total`: Total emissions that are not allocated.
    ///
    /// # Returns
    /// Tuple containing the total amount of emissions and the amount to be locked.
    fn get_total_weekly_emissions(
        &self,
        week: u128,
        unallocated_total: u128
    ) -> (u128, u128);

    /// Set the lock parameters.
    ///
    /// # Parameters
    /// - `lock_weeks`: Number of weeks for the lock period.
    /// - `lock_decay_weeks`: Number of weeks over which the lock decays.
    ///
    /// # Returns
    /// Whether the operation was successful.
    fn set_lock_parameters(
        &self,
        lock_weeks: u64,
        lock_decay_weeks: u64
    ) -> bool;

    /// Set the weekly percentage schedule.
    ///
    /// # Parameters
    /// - `schedule`: Array of tuples where each tuple contains a week and a percentage.
    ///
    /// # Returns
    /// Whether the operation was successful.
    fn set_weekly_pct_schedule(
        &self,
        schedule: &[(u64, u64)]
    ) -> bool;

    /// Get the maximum number of lock weeks.
    ///
    /// # Returns
    /// Maximum number of lock weeks.
    fn max_lock_weeks(&self) -> u128;

    /// Get the Babel core address.
    ///
    /// # Returns
    /// Babel core address.
    fn babel_core(&self) -> &str;

    /// Get the current week number.
    ///
    /// # Returns
    /// Current week number.
    fn get_week(&self) -> u128;

    /// Get the weekly percentage schedule.
    ///
    /// # Returns
    /// Array of tuples where each tuple contains a week and a percentage.
    fn get_weekly_pct_schedule(&self) -> Vec<(u64, u64)>;

    /// Get the guardian address.
    ///
    /// # Returns
    /// Guardian address.
    fn guardian(&self) -> &str;

    /// Get the number of lock decay weeks.
    ///
    /// # Returns
    /// Number of lock decay weeks.
    fn lock_decay_weeks(&self) -> u64;

    /// Get the number of lock weeks.
    ///
    /// # Returns
    /// Number of lock weeks.
    fn lock_weeks(&self) -> u64;

    /// Get the owner address.
    ///
    /// # Returns
    /// Owner address.
    fn owner(&self) -> &str;

    /// Get the vault address.
    ///
    /// # Returns
    /// Vault address.
    fn vault(&self) -> &str;

    /// Get the voter address.
    ///
    /// # Returns
    /// Voter address.
    fn voter(&self) -> &str;

    /// Get the weekly percentage.
    ///
    /// # Returns
    /// Weekly percentage.
    fn weekly_pct(&self) -> u64;
}
