use borsh::{BorshSerialize, BorshDeserialize};
use crate::interfaces2::liquidation_manager_interface::LiquidationManagerInterface;
use crate::models::{LiquidationParams, LiquidationResult};
use sdk::utxo::UtxoMeta;
use sdk::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct LiquidationManager {
    enabled_trove_managers: Vec<Pubkey>,
}

impl LiquidationManager {
    pub fn new() -> Self {
        Self {
            enabled_trove_managers: Vec::new(),
        }
    }

    pub fn enable_trove_manager(&mut self, trove_manager: Pubkey) {
        self.enabled_trove_managers.push(trove_manager);
    }
}

impl LiquidationManagerInterface for LiquidationManager {
    pub fn liquidate(&self, params: LiquidationParams) -> LiquidationResult {
        if !self.enabled_trove_managers.contains(&params.trove_manager) {
            return None;
        }

        // Example liquidation logic
        LiquidationResult {
            borrower: params.borrower,
            liquidated_debt: 1000,
            liquidated_coll: 500,
        }
    }
}
