use arch_program::{
    account::AccountInfo,
    pubkey::Pubkey,
    utxo::UtxoMeta,
    program_error::ProgramError,
};
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct DeploymentParams {
    pub owner: Pubkey,
    // Additional deployment parameters
}

pub trait IFactory {
    fn deploy_new_instance(&self, params: DeploymentParams) -> Result<(), ProgramError>;
    fn manage_utxos(&self, utxo_meta: &UtxoMeta) -> Result<(), ProgramError>;
}
