pub struct GaugeWeightVote {
    pub gauge: String,
    pub weight: u128,
}

pub struct TokenBalance {
    pub token: String,
    pub amount: u128,
}

pub trait CurveProxy {
    // Events
    fn emit_crv_fee_pct_set(&self, fee_pct: u64);

    // Functions
    fn approve_gauge_deposit(&mut self, gauge: &str, depositor: &str) -> bool;

    fn claim_fees(&mut self) -> u128;

    fn execute(&mut self, target: &str, data: &[u8]) -> Vec<u8>;

    fn lock_crv(&mut self) -> bool;

    fn mint_crv(&mut self, gauge: &str, receiver: &str) -> u128;

    fn set_crv_fee_pct(&mut self, fee_pct: u64) -> bool;

    fn set_deposit_manager(&mut self, deposit_manager: &str) -> bool;

    fn set_execute_permissions(
        &mut self,
        caller: &str,
        target: &str,
        selectors: &[u32],  
        permitted: bool,
    ) -> bool;

    fn set_gauge_rewards_receiver(&mut self, gauge: &str, receiver: &str) -> bool;

    fn set_per_gauge_approval(&mut self, caller: &str, gauge: &str) -> bool;

    fn set_vote_manager(&mut self, vote_manager: &str) -> bool;

    fn transfer_tokens(&mut self, receiver: &str, balances: &[TokenBalance]) -> bool;

    fn vote_for_gauge_weights(&mut self, votes: &[GaugeWeightVote]) -> bool;

    fn vote_in_curve_dao(&mut self, aragon: &str, id: u128, support: bool) -> bool;

    fn withdraw_from_gauge(
        &mut self,
        gauge: &str,
        lp_token: &str,
        amount: u128,
        receiver: &str,
    ) -> bool;

    // Getter functions
    fn crv(&self) -> &str;

    fn babel_core(&self) -> &str;

    fn crv_fee_pct(&self) -> u64;

    fn deposit_manager(&self) -> &str;

    fn fee_distributor(&self) -> &str;

    fn fee_token(&self) -> &str;

    fn gauge_controller(&self) -> &str;

    fn guardian(&self) -> &str;

    fn minter(&self) -> &str;

    fn owner(&self) -> &str;

    fn per_gauge_approval(&self, caller: &str) -> &str;

    fn unlock_time(&self) -> u64;

    fn vote_manager(&self) -> &str;

    fn voting_escrow(&self) -> &str;
}
