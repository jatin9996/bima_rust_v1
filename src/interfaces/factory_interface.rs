pub trait IFactory {
    fn deploy_new_instance(&self, params: DeploymentParams) -> Result<(), ProgramError>;
    fn set_implementations(&self, trove_manager_impl: &str, sorted_troves_impl: &str);
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct DeploymentParams {
    pub minute_decay_factor: u128,
    pub redemption_fee_floor: u128,
    pub max_redemption_fee: u128,
    pub borrowing_fee_floor: u128,
    pub max_borrowing_fee: u128,
    pub interest_rate_in_bps: u128,
    pub max_debt: u128,
    pub mcr: u128,
}
