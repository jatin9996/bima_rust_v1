use crate::babel_ownable::BabelOwnable;
use crate::constants::BIMA_100_PCT;
use crate::interfaces::{
    ILiquidityGauge, ICurveProxy, IGaugeController, IERC20, IMinter, IFeeDistributor, IVotingEscrow, IAragon,
};
use crate::utils::safe_erc20::SafeERC20;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct CurveProxy {
    babel_core: BabelOwnable,
    crv: Box<dyn IERC20>,
    gauge_controller: Box<dyn IGaugeController>,
    minter: Box<dyn IMinter>,
    voting_escrow: Box<dyn IVotingEscrow>,
    fee_distributor: Box<dyn IFeeDistributor>,
    fee_token: Box<dyn IERC20>,

    crv_fee_pct: u64,
    unlock_time: u64,
    vote_manager: Option<String>,
    deposit_manager: Option<String>,
    per_gauge_approval: HashMap<String, String>,
    execute_permissions: HashMap<String, HashMap<String, HashMap<[u8; 4], bool>>>,
}

impl CurveProxy {
    const WEEK: u64 = 604800;
    const MAX_LOCK_DURATION: u64 = 4 * 365 * 86400;

    pub fn new(
        babel_core: BabelOwnable,
        crv: Box<dyn IERC20>,
        gauge_controller: Box<dyn IGaugeController>,
        minter: Box<dyn IMinter>,
        voting_escrow: Box<dyn IVotingEscrow>,
        fee_distributor: Box<dyn IFeeDistributor>,
    ) -> Self {
        let fee_token = fee_distributor.token();

        crv.approve(voting_escrow.get_address(), u64::MAX);

        Self {
            babel_core,
            crv,
            gauge_controller,
            minter,
            voting_escrow,
            fee_distributor,
            fee_token,
            crv_fee_pct: 0,
            unlock_time: 0,
            vote_manager: None,
            deposit_manager: None,
            per_gauge_approval: HashMap::new(),
            execute_permissions: HashMap::new(),
        }
    }

    fn is_owner_or_vote_manager(&self, caller: &str) -> bool {
        caller == self.vote_manager.as_ref().unwrap_or(&String::new()) || caller == self.babel_core.owner()
    }

    fn is_deposit_manager(&self, caller: &str) -> bool {
        caller == self.deposit_manager.as_ref().unwrap_or(&String::new())
    }

    fn is_approved_for_gauge(&self, caller: &str, gauge: &str) -> bool {
        self.per_gauge_approval.get(caller).map_or(false, |g| g == gauge)
            || caller == self.deposit_manager.as_ref().unwrap_or(&String::new())
    }

    pub fn set_execute_permissions(
        &mut self,
        caller: &str,
        target: &str,
        selectors: Vec<[u8; 4]>,
        permitted: bool,
    ) {
        let execute_permission = self
            .execute_permissions
            .entry(caller.to_string())
            .or_insert_with(HashMap::new)
            .entry(target.to_string())
            .or_insert_with(HashMap::new);

        for selector in selectors {
            execute_permission.insert(selector, permitted);
        }
    }

    pub fn set_crv_fee_pct(&mut self, fee_pct: u64) {
        if fee_pct > BIMA_100_PCT {
            panic!("Invalid setting");
        }
        self.crv_fee_pct = fee_pct;
    }

    pub fn set_vote_manager(&mut self, vote_manager: String) {
        self.vote_manager = Some(vote_manager);
    }

    pub fn set_deposit_manager(&mut self, deposit_manager: String) {
        self.deposit_manager = Some(deposit_manager);
    }

    pub fn set_per_gauge_approval(&mut self, caller: String, gauge: String) {
        self.per_gauge_approval.insert(caller, gauge);
    }

    pub fn claim_fees(&mut self) {
        self.fee_distributor.claim();
        let amount = self.fee_token.balance_of(self.get_address());
        self.fee_token.transfer(self.babel_core.fee_receiver(), amount);
    }

    pub fn lock_crv(&mut self) {
        let max_unlock = Self::current_block_timestamp() / Self::WEEK * Self::WEEK + Self::MAX_LOCK_DURATION;
        let amount = self.crv.balance_of(self.get_address());

        self.update_lock(amount, self.unlock_time, max_unlock);
    }

    pub fn mint_crv(&mut self, gauge: &str, receiver: &str) {
        let initial = self.crv.balance_of(self.get_address());
        self.minter.mint(gauge);
        let mut amount = self.crv.balance_of(self.get_address()) - initial;

        let fee = amount * self.crv_fee_pct / BIMA_100_PCT;
        amount -= fee;

        self.crv.transfer(receiver, amount);

        let max_unlock = Self::current_block_timestamp() / Self::WEEK * Self::WEEK + Self::MAX_LOCK_DURATION;
        if self.unlock_time < max_unlock {
            self.update_lock(initial + fee, self.unlock_time, max_unlock);
        }
    }

    pub fn vote_for_gauge_weights(&mut self, votes: Vec<GaugeWeightVote>) {
        for vote in votes {
            self.gauge_controller.vote_for_gauge_weights(vote.gauge, vote.weight);
        }
    }

    pub fn vote_in_curve_dao(&mut self, aragon: Box<dyn IAragon>, id: u64, support: bool) {
        aragon.vote(id, support, false);
    }

    pub fn approve_gauge_deposit(&mut self, gauge: &str, depositor: &str) {
        self.minter.approve_deposit(gauge, depositor, true);
    }

    pub fn set_gauge_rewards_receiver(&mut self, gauge: &str, receiver: &str) {
        self.minter.set_rewards_receiver(gauge, receiver);
    }

    pub fn withdraw_from_gauge(&mut self, gauge: &str, lp_token: Box<dyn IERC20>, amount: u64, receiver: &str) {
        self.minter.withdraw_from_gauge(gauge, amount);
        lp_token.transfer(receiver, amount);
    }

    pub fn transfer_tokens(&mut self, receiver: &str, balances: Vec<TokenBalance>) {
        for balance in balances {
            balance.token.safe_transfer(receiver, balance.amount);
        }
    }

    pub fn execute(&mut self, target: &str, data: &[u8]) {
        let selector = &data[0..4];
        if !self.is_owner_or_vote_manager(self.babel_core.owner()) {
            if !self
                .execute_permissions
                .get(self.babel_core.owner())
                .and_then(|permissions| permissions.get(target))
                .map_or(false, |selectors| selectors.get(selector).copied().unwrap_or(false))
            {
                panic!("Not permitted");
            }
        }
        self.call_function(target, data);
    }

    fn update_lock(&mut self, amount: u64, unlock: u64, max_unlock: u64) {
        if amount > 0 {
            if unlock == 0 {
                self.voting_escrow.create_lock(amount, max_unlock);
                self.unlock_time = max_unlock;
                return;
            }
            self.voting_escrow.increase_amount(amount);
        }
        if unlock < max_unlock {
            self.voting_escrow.increase_unlock_time(max_unlock);
            self.unlock_time = max_unlock;
        }
    }

    fn get_address(&self) -> &str {
        self.babel_core.get_address()
    }

    fn current_block_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    fn call_function(&self, target: &str, data: &[u8]) {
        // Implementation of calling external contract's function via its address and data.
        // Use cross-call functionality or an external library to handle this.
    }
}

pub struct GaugeWeightVote {
    pub gauge: String,
    pub weight: u64,
}

pub struct TokenBalance {
    pub token: Box<dyn IERC20>,
    pub amount: u64,
}