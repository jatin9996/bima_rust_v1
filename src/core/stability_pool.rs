use std::collections::HashMap;
use crate::dependencies::babel_ownable::BabelOwnable;
use crate::dependencies::system_start::SystemStart;
use crate::dependencies::babel_math::BabelMath;
use crate::interfaces::stability_pool::IStabilityPool;

pub struct StabilityPool {
    deposits: HashMap<AccountId, Balance>,
    total_stablecoins: Balance,
    owner: AccountId,
    collaterals: HashMap<CollateralId, CollateralData>,
    debt_token: DebtToken,
    reward_rate: Balance,
    last_update: u64,
    period_finish: u64,
}

impl StabilityPool {
    pub fn new(owner_id: AccountId, debt_token: DebtToken) -> Self {
        StabilityPool {
            deposits: HashMap::new(),
            total_stablecoins: 0,
            owner: owner_id,
            collaterals: HashMap::new(),
            debt_token,
            reward_rate: 0,
            last_update: 0,
            period_finish: 0,
        }
    }

    pub fn deposit(&mut self, caller: AccountId, amount: Balance) {
        self.only_owner(&caller);
        let current_deposit = self.deposits.entry(caller).or_insert(0);
        *current_deposit += amount;
        self.total_stablecoins += amount;
    }

    pub fn withdraw(&mut self, caller: AccountId, amount: Balance) -> bool {
        self.only_owner(&caller);
        if let Some(current_deposit) = self.deposits.get_mut(&caller) {
            if *current_deposit >= amount {
                *current_deposit -= amount;
                self.total_stablecoins -= amount;
                return true;
            }
        }
        false
    }

    fn only_owner(&self, caller: &AccountId) {
        assert_eq!(&self.owner, caller, "Only owner can call this function");
    }

    pub fn enable_collateral(&mut self, caller: AccountId, collateral_id: CollateralId) {
        self.only_owner(&caller);
        if !self.collaterals.contains_key(&collateral_id) {
            self.collaterals.insert(collateral_id, CollateralData::new());
        }
    }

    pub fn start_collateral_sunset(&mut self, caller: AccountId, collateral_id: CollateralId) {
        self.only_owner(&caller);
        if let Some(collateral) = self.collaterals.get_mut(&collateral_id) {
            collateral.start_sunset();
        }
    }

    pub fn offset(&mut self, caller: AccountId, collateral_id: CollateralId, debt_to_offset: Balance, coll_to_add: Balance) {
        self.only_owner(&caller);
        if let Some(collateral) = self.collaterals.get_mut(&collateral_id) {
            collateral.offset(debt_to_offset, coll_to_add);
        }
    }

    pub fn claim_reward(&mut self, caller: AccountId) -> Balance {
        self.only_owner(&caller);
        let reward = self.calculate_reward(caller);
        self.debt_token.transfer(caller, reward);
        reward
    }

    fn calculate_reward(&self, caller: AccountId) -> Balance {
        // Reward calculation based on deposits and a fixed reward rate
        if let Some(deposit) = self.deposits.get(&caller) {
            let reward = (*deposit as f64 * self.reward_rate as f64 / 100.0) as Balance;
            reward
        } else {
            0
        }
    }

    pub fn calculate_interest(&self, coll: Balance, debt: Balance) -> Balance {
        let interest_rate: Balance = 5; // Interest rate of 5%
        let rate_decimal: Balance = interest_rate / 100; // Convert to decimal
        coll * rate_decimal * debt // Calculate simple interest
    }
}

// Types for AccountId, Balance, CollateralId, DebtToken, and CollateralData would need to be defined or imported
type AccountId = u64; // Example type
type Balance = u64; // Example type
type CollateralId = u64; // Example type

struct DebtToken;

impl DebtToken {
    fn transfer(&self, to: AccountId, amount: Balance) {
        // Assuming we have a global or static mutable HashMap to track balances
        let mut balances: HashMap<AccountId, Balance> = HashMap::new();

        // Check if the sender has enough balance
        let sender_balance = balances.entry(self.owner).or_insert(0);
        if *sender_balance < amount {
            panic!("Insufficient balance");
        }

        // Deduct the amount from the sender's balance
        *sender_balance -= amount;

        // Add the amount to the recipient's balance
        let recipient_balance = balances.entry(to).or_insert(0);
        *recipient_balance += amount;
    }
}

struct CollateralData {
    amount: Balance,
    is_sunset: bool,
}

impl CollateralData {
    fn new() -> Self {
        CollateralData {
            amount: 0,
            is_sunset: false,
        }
    }

    fn start_sunset(&mut self) {
        self.is_sunset = true;
    }

    fn offset(&mut self, debt_to_offset: Balance, coll_to_add: Balance) {
        if !self.is_sunset {
            self.amount += coll_to_add;
            // Implement logic to handle debt_to_offset
            if self.amount >= debt_to_offset {
                self.amount -= debt_to_offset;
            } else {
                // Handle the case where the collateral is insufficient to offset the debt
                panic!("Insufficient collateral to offset the debt");
            }
        }
    }
}