use crate::interfaces::borrower_operations_interface::BorrowerOperationsInterface;
use crate::models::state::{SystemBalances, TroveManagerData};
use crate::helpers::utxo_helpers::{validate_utxo_ownership, update_utxo_state};
use arch_program::{
    account::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    utxo::UtxoMeta,
};

pub struct BorrowerOperations {
    pub min_net_debt: u64,
    pub trove_managers: Vec<TroveManagerData>,
}

impl BorrowerOperationsInterface for BorrowerOperations {
    fn add_liquidity(&mut self, utxo: &UtxoMeta, amount: u64) -> ProgramResult {
        validate_utxo_ownership(utxo)?;
        update_utxo_state(utxo, amount)?;
        Ok(())
    }

    fn remove_liquidity(&mut self, utxo: &UtxoMeta, amount: u64) -> ProgramResult {
        validate_utxo_ownership(utxo)?;
        update_utxo_state(utxo, -amount as i64)?;
        Ok(())
    }
}