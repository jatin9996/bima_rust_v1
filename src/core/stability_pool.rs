use std::collections::HashMap;
use crate::dependecies::babel_ownable::BabelOwnable;
use crate::dependecies::system_start::SystemStart;
use crate::dependecies::babel_math::BabelMath;
use crate::interfaces::debt_token::DebtToken;
use crate::interfaces::vault::IBabelVault;
use num_bigint::BigUint;

pub struct StabilityPool {
    deposits: HashMap<String, u64>, // User deposits mapped by user ID
    total_stablecoins: u64,         // Total stablecoins held in the pool
    owner: BabelOwnable,            // Ownership management
    system_start: SystemStart,      // System start time for time-based calculations
    debt_token: DebtToken,          // Debt token management
}

impl StabilityPool {
    pub fn new(owner_account: AccountId, start_time: u64) -> Self {
        Self {
            deposits: HashMap::new(),
            total_stablecoins: 0,
            owner: BabelOwnable::new(owner_account),
            system_start: SystemStart::new(owner_account),
            debt_token: DebtToken::new(),
        }
    }

    pub fn deposit(&mut self, user_id: String, amount: u64) {
        self.owner.only_owner(); // Ensure only the owner can call this
        let current_deposit = self.deposits.entry(user_id).or_insert(0);
        *current_deposit += amount;
        self.total_stablecoins += amount;
        self.debt_token.issue(amount); // Issue debt tokens corresponding to the deposit
    }

    pub fn withdraw(&mut self, user_id: String, amount: u64) -> bool {
        self.owner.only_owner(); // Ensure only the owner can call this
        if let Some(current_deposit) = self.deposits.get_mut(&user_id) {
            if *current_deposit >= amount {
                *current_deposit -= amount;
                self.total_stablecoins -= amount;
                self.debt_token.burn(amount); // Burn debt tokens corresponding to the withdrawal
                return true;
            }
        }
        false
    }

    // Example of using BabelMath for a calculation
    pub fn calculate_interest(&self, coll: BigUint, debt: BigUint) -> BigUint {
        BabelMath::compute_cr(coll, debt, BigUint::from(1u32)) // Simplified example
    }
}