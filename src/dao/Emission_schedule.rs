use std::collections::VecDeque;
use crate::babel_ownable::BabelOwnable;
use crate::system_start::SystemStart;
use crate::interfaces::vault::IBabelVault;
use crate::interfaces::incentive_voting::IIncentiveVoting;

const MAX_PCT: u64 = 10000;
const MAX_LOCK_WEEKS: u64 = 52;

struct EmissionSchedule {
    owner: BabelOwnable, // Changed from String to BabelOwnable
    system_start: SystemStart, // Added SystemStart for time management
    vault: Box<dyn IBabelVault>, // Added vault interface
    voter: Box<dyn IIncentiveVoting>, // Added voter interface
    lock_weeks: u64,
    lock_decay_weeks: u64,
    weekly_pct: u64,
    scheduled_weekly_pct: VecDeque<(u64, u64)>, // Using VecDeque for efficient pops from the end
}

impl EmissionSchedule {
    pub fn new(
        owner: BabelOwnable, 
        system_start: SystemStart, 
        vault: Box<dyn IBabelVault>, 
        voter: Box<dyn IIncentiveVoting>, 
        initial_lock_weeks: u64, 
        lock_decay_weeks: u64, 
        weekly_pct: u64, 
        scheduled_weekly_pct: Vec<(u64, u64)>
    ) -> Self {
        assert!(initial_lock_weeks <= MAX_LOCK_WEEKS, "Cannot exceed MAX_LOCK_WEEKS");
        assert!(lock_decay_weeks > 0, "Decay weeks cannot be 0");
        assert!(weekly_pct <= MAX_PCT, "Cannot exceed MAX_PCT");

        EmissionSchedule {
            owner,
            system_start,
            vault,
            voter,
            lock_weeks: initial_lock_weeks,
            lock_decay_weeks,
            weekly_pct,
            scheduled_weekly_pct: scheduled_weekly_pct.into_iter().collect(),
        }
    }

    pub fn set_weekly_pct_schedule(&mut self, schedule: Vec<(u64, u64)>) {
        self.owner.only_owner(); // Ownership check
        // Ensure the schedule is sorted and valid
        let mut last_week = u64::MAX;
        for &(week, pct) in &schedule {
            assert!(week < last_week, "Must sort by week descending");
            assert!(pct <= MAX_PCT, "Cannot exceed MAX_PCT");
            last_week = week;
        }

        self.scheduled_weekly_pct = schedule.into_iter().collect();
    }

    pub fn get_weekly_pct(&self, current_week: u64) -> u64 {
        let week = self.system_start.get_week(); // Use SystemStart to get the current week
        // Iterate through the scheduled percentages to find the applicable percentage for the given week
        for &(week, pct) in self.scheduled_weekly_pct.iter().rev() {
            if current_week >= week {
                return pct;
            }
        }
        // If no specific entry is found for the week, use the default weekly_pct
        self.weekly_pct
    }

    pub fn lock(&mut self, weeks: u64) {
        self.owner.only_owner(); // Ownership check
        // Ensure that the number of weeks to lock does not exceed the maximum allowed
        assert!(weeks <= MAX_LOCK_WEEKS, "Lock duration exceeds maximum allowed weeks");
        // Update the lock_weeks to the specified number of weeks
        self.lock_weeks = weeks;
    }

    pub fn unlock(&mut self) {
        self.owner.only_owner(); // Ownership check
        // Reset the lock_weeks to zero to fully unlock
        self.lock_weeks = 0;
    }
}