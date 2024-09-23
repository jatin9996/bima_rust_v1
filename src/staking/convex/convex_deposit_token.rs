use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct IERC20 {
    pub balance: HashMap<String, u128>, // To track token balances of users
}

impl IERC20 {
    pub fn transfer_from(&mut self, from: &str, to: &str, amount: u128) -> bool {
        let from_balance = self.balance.entry(from.to_string()).or_insert(0);
        let to_balance = self.balance.entry(to.to_string()).or_insert(0);

        if *from_balance >= amount {
            *from_balance -= amount;
            *to_balance += amount;
            true
        } else {
            false
        }
    }

    pub fn transfer(&mut self, to: &str, amount: u128) -> bool {
        let to_balance = self.balance.entry(to.to_string()).or_insert(0);
        *to_balance += amount;
        true
    }

    pub fn balance_of(&self, account: &str) -> u128 {
        *self.balance.get(account).unwrap_or(&0)
    }
}

// Booster interface
pub trait IBooster {
    fn deposit(&mut self, pid: u64, amount: u128, stake: bool) -> bool;
    fn pool_info(&self, pid: u64) -> (String, String, String, String, String, bool);
}

// Reward pool interface
pub trait IBaseRewardPool {
    fn withdraw_and_unwrap(&mut self, amount: u128, claim: bool) -> bool;
    fn get_reward(&mut self, account: &str, claim_extras: bool) -> bool;
}

// Convex stash interface
pub trait IConvexStash {
    fn token_info(&self, token: &str) -> (String, String);
}

// ConvexDepositToken structure
pub struct ConvexDepositToken {
    pub babel: IERC20,
    pub crv: IERC20,
    pub cvx: IERC20,

    pub booster: Box<dyn IBooster>,
    pub curve_proxy: Box<dyn ICurveProxy>,
    pub vault: Box<dyn IBabelVault>,

    pub lp_token: IERC20,
    pub deposit_pid: u64,
    pub crv_rewards: Box<dyn IBaseRewardPool>,
    pub cvx_rewards: Box<dyn IBaseRewardPool>,

    pub emission_id: u64,
    pub symbol: String,
    pub name: String,
    pub total_supply: u128,

    pub balance_of: HashMap<String, u128>,
    pub allowance: HashMap<(String, String), u128>, // (owner, spender) -> allowance

    pub reward_integral: [u128; 3],
    pub reward_rate: [u128; 3],
    pub last_crv_balance: u128,
    pub last_cvx_balance: u128,
    pub last_update: u64,
    pub period_finish: u64,

    pub reward_integral_for: HashMap<String, [u128; 3]>,
    pub stored_pending_reward: HashMap<String, [u128; 3]>,
}

impl ConvexDepositToken {
    pub fn new(
        babel: IERC20,
        crv: IERC20,
        cvx: IERC20,
        booster: Box<dyn IBooster>,
        curve_proxy: Box<dyn ICurveProxy>,
        vault: Box<dyn IBabelVault>,
    ) -> Self {
        Self {
            babel,
            crv,
            cvx,
            booster,
            curve_proxy,
            vault,
            lp_token: IERC20 { balance: HashMap::new() },
            deposit_pid: 0,
            crv_rewards: Box::new(DefaultRewardPool::new()), // Default reward pool implementation
            cvx_rewards: Box::new(DefaultRewardPool::new()),

            emission_id: 0,
            symbol: String::new(),
            name: String::new(),
            total_supply: 0,

            balance_of: HashMap::new(),
            allowance: HashMap::new(),

            reward_integral: [0; 3],
            reward_rate: [0; 3],
            last_crv_balance: 0,
            last_cvx_balance: 0,
            last_update: 0,
            period_finish: 0,

            reward_integral_for: HashMap::new(),
            stored_pending_reward: HashMap::new(),
        }
    }

    pub fn initialize(&mut self, pid: u64) {
        assert_eq!(self.lp_token.balance.len(), 0, "Already initialized");

        let (lp_token, _, _, crv_rewards, stash, _) = self.booster.pool_info(pid);

        self.deposit_pid = pid;
        self.lp_token = IERC20 { balance: HashMap::new() };
        self.crv_rewards = Box::new(DefaultRewardPool::new());

        let (_, rewards) = self.curve_proxy.token_info(&stash);
        assert_ne!(rewards, "", "Pool has no CVX rewards");
        self.cvx_rewards = Box::new(DefaultRewardPool::new());

        // Setup token approval and metadata
        self.symbol = format!("babel-{}", lp_token);
        self.name = format!("Babel {} Convex Deposit", lp_token);

        self.period_finish = Self::current_timestamp() - 1;
    }

    pub fn deposit(&mut self, receiver: &str, amount: u128) -> bool {
        assert!(amount > 0, "Cannot deposit zero");
        self.lp_token.transfer_from(receiver, &self.symbol, amount);

        self.booster.deposit(self.deposit_pid, amount, true);

        let balance = self.balance_of.entry(receiver.to_string()).or_insert(0);
        *balance += amount;
        self.total_supply += amount;

        self.update_integrals(receiver, *balance, self.total_supply);
        if Self::current_timestamp() / 604800 >= self.period_finish / 604800 {
            self.fetch_rewards();
        }

        true
    }

    pub fn withdraw(&mut self, receiver: &str, amount: u128) -> bool {
        assert!(amount > 0, "Cannot withdraw zero");
        let balance = self.balance_of.entry(receiver.to_string()).or_insert(0);
        *balance -= amount;
        self.total_supply -= amount;

        self.crv_rewards.withdraw_and_unwrap(amount, false);
        self.lp_token.transfer(receiver, amount);

        self.update_integrals(receiver, *balance, self.total_supply);
        if Self::current_timestamp() / 604800 >= self.period_finish / 604800 {
            self.fetch_rewards();
        }

        true
    }

    fn claim_reward(&mut self, claimant: &str, receiver: &str) -> [u128; 3] {
        self.update_integrals(claimant, self.balance_of[claimant], self.total_supply);
        let pending_reward = self.stored_pending_reward.get(claimant).cloned().unwrap_or([0; 3]);
        self.stored_pending_reward.insert(claimant.to_string(), [0; 3]);
        self.last_crv_balance -= pending_reward[1];
        self.last_cvx_balance -= pending_reward[2];

        self.crv.transfer(receiver, pending_reward[1]);
        self.cvx.transfer(receiver, pending_reward[2]);

        pending_reward
    }

    pub fn claimable_reward(&self, account: &str) -> [u128; 3] {
        let duration = self.period_finish - self.last_update;
        let balance = self.balance_of[account];
        let mut amounts = [0; 3];

        for i in 0..3 {
            let integral = self.reward_integral[i];
            let integral_for = self.reward_integral_for[account][i];
            amounts[i] = (balance * (integral - integral_for)) / 1e18 as u128;
        }

        amounts
    }

    fn update_integrals(&mut self, account: &str, balance: u128, supply: u128) {
        let current_time = Self::current_timestamp();
        let duration = current_time - self.last_update;
        self.last_update = current_time;

        for i in 0..3 {
            if duration > 0 && supply > 0 {
                self.reward_integral[i] += (duration as u128 * self.reward_rate[i] * 1e18 as u128) / supply;
            }

            let integral_for = self.reward_integral_for.get(account).cloned().unwrap_or([0; 3]);
            if self.reward_integral[i] > integral_for[i] {
                let integral_diff = self.reward_integral[i] - integral_for[i];
                let stored_reward = self.stored_pending_reward.entry(account.to_string()).or_insert([0; 3]);
                stored_reward[i] += (balance * integral_diff) / 1e18 as u128;
                let reward_integral = self.reward_integral_for.entry(account.to_string()).or_insert([0; 3]);
                reward_integral[i] = self.reward_integral[i];
            }
        }
    }

    fn fetch_rewards(&mut self) {
        assert!(Self::current_timestamp() / 604800 >= self.period_finish / 604800, "Can only fetch once per week");

        self.update_integrals("", 0, self.total_supply);
        // Fetch rewards logic...
    }

    fn current_timestamp() -> u64 {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        since_the_epoch.as_secs()
    }
}

pub struct DefaultRewardPool {}

impl DefaultRewardPool {
    pub fn new() -> Self {
        Self {}
    }
}

impl IBaseRewardPool for DefaultRewardPool {
    fn withdraw_and_unwrap(&mut self, _amount: u128, _claim: bool) -> bool {
        true
    }

    fn get_reward(&mut self, _account: &str, _claim_extras: bool) -> bool {
        true
    }
}

pub trait ICurveProxy {
    fn token_info(&self, token: &str) -> (String, String);
}

pub trait IBabelVault {}