use borsh::{BorshDeserialize, BorshSerialize};

pub trait IIncentiveVoting {
    /// Represents a vote with an ID and points.
    #[derive(BorshSerialize, BorshDeserialize)]
    struct Vote {
        id: u256,
        points: u256,
    }

    /// Represents lock data with an amount and weeks to unlock.
    #[derive(BorshSerialize, BorshDeserialize)]
    struct LockData {
        amount: u256,
        weeks_to_unlock: u256,
    }

    /// Clears the registered weight for an account.
    fn clear_registered_weight(&self, account: &str) -> bool;

    /// Clears the vote for an account.
    fn clear_vote(&self, account: &str);

    /// Returns the vote percentage for a receiver.
    fn get_receiver_vote_pct(&self, id: u256, week: u256) -> u256;

    /// Returns the weight for a specific receiver.
    fn get_receiver_weight_write(&self, idx: u256) -> u256;

    /// Returns the total weight.
    fn get_total_weight_write(&self) -> u256;

    /// Registers the weight for an account.
    fn register_account_weight(&self, account: &str, min_weeks: u256);

    /// Registers the weight and votes for an account.
    fn register_account_weight_and_vote(&self, account: &str, min_weeks: u256, votes: &[Vote]);

    /// Registers a new receiver and returns the ID.
    fn register_new_receiver(&self) -> u256;

    /// Sets delegate approval status.
    fn set_delegate_approval(&self, delegate: &str, is_approved: bool);

    /// Unfreezes an account with an option to keep the vote.
    fn unfreeze(&self, account: &str, keep_vote: bool) -> bool;

    /// Submits votes for an account with an option to clear previous votes.
    fn vote(&self, account: &str, votes: &[Vote], clear_previous: bool);

    /// Returns the maximum lock weeks allowed.
    fn max_lock_weeks(&self) -> u256;

    /// Returns the maximum points allowed.
    fn max_points(&self) -> u256;

    /// Returns the current votes for an account.
    fn get_account_current_votes(&self, account: &str) -> Vec<Vote>;

    /// Returns the registered locks for an account.
    fn get_account_registered_locks(&self, account: &str) -> (u256, Vec<LockData>);

    /// Returns the weight for a specific receiver.
    fn get_receiver_weight(&self, idx: u256) -> u256;

    /// Returns the weight for a receiver at a specific week.
    fn get_receiver_weight_at(&self, idx: u256, week: u256) -> u256;

    /// Returns the total weight.
    fn get_total_weight(&self) -> u256;

    /// Returns the total weight at a specific week.
    fn get_total_weight_at(&self, week: u256) -> u256;

    /// Returns the current week.
    fn get_week(&self) -> u256;

    /// Checks if an address is an approved delegate.
    fn is_approved_delegate(&self, owner: &str, caller: &str) -> bool;

    /// Returns the count of receivers.
    fn receiver_count(&self) -> u256;

    /// Returns the decay rate for a receiver.
    fn receiver_decay_rate(&self, idx: u256) -> u32;

    /// Returns the week the receiver was last updated.
    fn receiver_updated_week(&self, idx: u256) -> u16;

    /// Returns the weekly unlocks for a receiver.
    fn receiver_weekly_unlocks(&self, idx: u256, week: u256) -> u32;

    /// Returns the token locker address.
    fn token_locker(&self) -> &str;

    /// Returns the total decay rate.
    fn total_decay_rate(&self) -> u32;

    /// Returns the week the total was last updated.
    fn total_updated_week(&self) -> u16;

    /// Returns the weekly unlocks for the total.
    fn total_weekly_unlocks(&self, week: u256) -> u32;

    /// Returns the vault address.
    fn vault(&self) -> &str;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockIncentiveVoting;

    impl IncentiveVoting for MockIncentiveVoting {
        fn clear_vote(&self, account: &str) {
            // Implementation for testing
        }

        fn get_receiver_vote_pct(&self, id: u256, week: u256) -> u256 {
            // Mock implementation
            100
        }

        fn get_receiver_weight_write(&self, idx: u256) -> u256 {
            // Mock implementation
            200
        }

        fn get_total_weight_write(&self) -> u256 {
            // Mock implementation
            1000
        }

        fn register_account_weight(&self, account: &str, min_weeks: u256) {
            // Implementation for testing
        }

        fn register_account_weight_and_vote(&self, account: &str, min_weeks: u256, votes: &[Vote]) {
            // Implementation for testing
        }

        fn register_new_receiver(&self) -> u256 {
            // Mock implementation
            42
        }
    }

    #[test]
    fn test_get_receiver_vote_pct() {
        let mock = MockIncentiveVoting;
        assert_eq!(mock.get_receiver_vote_pct(1, 1), 100);
    }

    #[test]
    fn test_get_receiver_weight_write() {
        let mock = MockIncentiveVoting;
        assert_eq!(mock.get_receiver_weight_write(1), 200);
    }

    #[test]
    fn test_get_total_weight_write() {
        let mock = MockIncentiveVoting;
        assert_eq!(mock.get_total_weight_write(), 1000);
    }

    #[test]
    fn test_register_new_receiver() {
        let mock = MockIncentiveVoting;
        assert_eq!(mock.register_new_receiver(), 42);
    }
}
