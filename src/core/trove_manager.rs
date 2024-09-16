use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use borsh::{BorshDeserialize, BorshSerialize};

pub struct TroveManager {
    troves: HashMap<AccountId, Trove>,
    total_stakes: Balance,
    total_active_collateral: Balance,
    total_active_debt: Balance,
    base_rate: Balance,
    last_fee_operation_time: u64,
    owner: AccountId,
    reward_integral: Balance,
    reward_rate: Balance,
    last_update: u64,
    period_finish: u64,
    reward_integral_for: HashMap<AccountId, Balance>,
    stored_pending_reward: HashMap<AccountId, Balance>,
    surplus_balances: HashMap<AccountId, Balance>,
    paused: bool,
    sunsetting: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Trove {
    debt: Balance,
    coll: Balance,
    stake: Balance,
    status: Status,
    array_index: u32,
    active_interest_index: Balance,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum Status {
    NonExistent,
    Active,
    ClosedByOwner,
    ClosedByLiquidation,
    ClosedByRedemption,
}

impl TroveManager {
    pub fn new(owner: AccountId) -> Self {
        Self {
            troves: HashMap::new(),
            total_stakes: 0,
            total_active_collateral: 0,
            total_active_debt: 0,
            base_rate: 0,
            last_fee_operation_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            owner,
            reward_integral: 0,
            reward_rate: 0,
            last_update: 0,
            period_finish: 0,
            reward_integral_for: HashMap::new(),
            stored_pending_reward: HashMap::new(),
            surplus_balances: HashMap::new(),
            paused: false,
            sunsetting: false,
        }
    }

    pub fn set_paused(&mut self, paused: bool) -> Result<(), String> {
        let caller = self.get_caller();
        if self.owner == caller {
            self.paused = paused;
            Ok(())
        } else {
            Err("Unauthorized: caller is not the owner".to_string())
        }
    }

    pub fn adjust_base_rate(&mut self, adjustment: Balance) -> Result<(), String> {
        let caller = self.get_caller();
        if self.owner == caller {
            self.base_rate = self.base_rate.saturating_add(adjustment);
            Ok(())
        } else {
            Err("Unauthorized: caller is not the owner".to_string())
        }
    }

    pub fn add_collateral(&mut self, borrower: AccountId, amount: Balance) {
        let trove = self.troves.get_mut(&borrower).unwrap();
        trove.coll += amount;
        self.total_active_collateral += amount;
    }

    pub fn claim_collateral(&mut self, receiver: AccountId) -> Result<Balance, String> {
        let claimable_coll = self.surplus_balances.get(&receiver).cloned().unwrap_or(0);
        if claimable_coll > 0 {
            self.surplus_balances.insert(receiver, 0);
            Ok(claimable_coll)
        } else {
            Err("No collateral available to claim".to_string())
        }
    }

    pub fn claim_reward(&mut self, account: AccountId) -> Result<Balance, String> {
        let amount = self.apply_pending_rewards(account)?;
        if amount > 0 {
            self.stored_pending_reward.insert(account, 0);
            Ok(amount)
        } else {
            Err("No rewards available to claim".to_string())
        }
    }

    pub fn apply_pending_rewards(&mut self, account: AccountId) -> Result<Balance, String> {
        let reward_integral = self.reward_integral;
        let reward_integral_for = self.reward_integral_for.get(&account).cloned().unwrap_or(0);
        if reward_integral > reward_integral_for {
            let pending_reward = self.stored_pending_reward.get(&account).cloned().unwrap_or(0);
            let new_reward = pending_reward + (self.troves.get(&account).unwrap().stake * (reward_integral - reward_integral_for)) / 1e18 as Balance;
            self.stored_pending_reward.insert(account, new_reward);
            self.reward_integral_for.insert(account, reward_integral);
            Ok(new_reward)
        } else {
            Ok(0)
        }
    }

    fn get_caller(&self) -> AccountId {
        self.owner 
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.try_to_vec().expect("Serialization should not fail")
    }

    pub fn deserialize(data: &[u8]) -> Self {
        Self::try_from_slice(data).expect("Deserialization should not fail")
    }
}

type AccountId = u32; // type definition
type Balance = u64; // type definition
type Timestamp = u64; // type definition