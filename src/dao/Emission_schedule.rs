#![cfg_attr(not(feature = "std"), no_std)]

use borsh::{BorshDeserialize, BorshSerialize};
use ink_prelude::collections::VecDeque;

#[derive(BorshSerialize, BorshDeserialize)]
struct BabelOwnable {
    owner: AccountId,
}

impl BabelOwnable {
    fn new(owner: AccountId) -> Self {
        Self { owner }
    }

    fn only_owner(&self, caller: AccountId) {
        assert_eq!(caller, self.owner, "Only owner can call this function");
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
struct EmissionSchedule {
    owner: AccountId,
    system_start: Timestamp,
    vault: AccountId,
    voter: AccountId,
    lock_weeks: u64,
    lock_decay_weeks: u64,
    weekly_pct: u64,
    scheduled_weekly_pct: VecDeque<(u64, u64)>,
    babel_ownable: BabelOwnable,
}

impl EmissionSchedule {
    pub fn new(
        owner: AccountId,
        system_start: Timestamp,
        vault: AccountId,
        voter: AccountId,
        initial_lock_weeks: u64,
        lock_decay_weeks: u64,
        weekly_pct: u64,
        scheduled_weekly_pct: Vec<(u64, u64)>
    ) -> Self {
        assert!(initial_lock_weeks <= MAX_LOCK_WEEKS, "Cannot exceed MAX_LOCK_WEEKS");
        assert!(lock_decay_weeks > 0, "Decay weeks cannot be 0");
        assert!(weekly_pct <= MAX_PCT, "Cannot exceed MAX_PCT");

        Self {
            owner,
            system_start,
            vault,
            voter,
            lock_weeks: initial_lock_weeks,
            lock_decay_weeks,
            weekly_pct,
            scheduled_weekly_pct: scheduled_weekly_pct.into_iter().collect(),
            babel_ownable: BabelOwnable::new(owner),
        }
    }

    pub fn set_weekly_pct_schedule(&mut self, caller: AccountId, schedule: Vec<(u64, u64)>) {
        self.babel_ownable.only_owner(caller);
        let mut last_week = u64::MAX;
        for &(week, pct) in &schedule {
            assert!(week < last_week, "Must sort by week descending");
            assert!(pct <= MAX_PCT, "Cannot exceed MAX_PCT");
            last_week = week;
        }
        self.scheduled_weekly_pct = schedule.into_iter().collect();
    }

    pub fn get_weekly_pct(&self, current_week: u64) -> u64 {
        for &(week, pct) in self.scheduled_weekly_pct.iter().rev() {
            if current_week >= week {
                return pct;
            }
        }
        self.weekly_pct
    }

    pub fn lock(&mut self, caller: AccountId, weeks: u64) {
        self.babel_ownable.only_owner(caller);
        assert!(weeks <= MAX_LOCK_WEEKS, "Lock duration exceeds maximum allowed weeks");
        self.lock_weeks = weeks;
    }

    pub fn unlock(&mut self, caller: AccountId) {
        self.babel_ownable.only_owner(caller);
        self.lock_weeks = 0;
    }
}

const MAX_PCT: u64 = 10000;
const MAX_LOCK_WEEKS: u64 = 52;

// Define types for AccountId and Timestamp as per your requirements
type AccountId = u64;
type Timestamp = u64;