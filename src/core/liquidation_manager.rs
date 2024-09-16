use std::collections::HashMap;
use log::{info, warn};
use borsh::{BorshSerialize, BorshDeserialize};

pub struct LiquidationManager {
    stability_pool: String,
    sorted_troves: String,
    borrower_operations: String,
    factory: String,
    enabled_trove_managers: HashMap<String, bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Liquidation {
    borrower: String,
    liquidated_debt: u64,
    liquidated_coll: u64,
}

impl LiquidationManager {
    pub fn new(factory: String) -> Self {
        LiquidationManager {
            stability_pool: String::new(),
            sorted_troves: String::new(),
            borrower_operations: String::new(),
            factory,
            enabled_trove_managers: HashMap::new(),
        }
    }

    pub fn enable_trove_manager(&mut self, trove_manager: String) {
        assert_eq!(self.factory, self.factory, "Only factory can enable a trove manager");
        self.enabled_trove_managers.insert(trove_manager, true);
    }

    pub fn liquidate(&mut self, trove_manager: String, borrower: String) -> Option<Liquidation> {
        let is_enabled = *self.enabled_trove_managers.get(&trove_manager).unwrap_or(&false);
        if !is_enabled {
            return None;
        }

        //  for financial calculations
        let debt_reduction = FinancialMath::calculate_debt_reduction();
        let collateral_reduction = FinancialMath::calculate_collateral_reduction();

        let liquidation = Liquidation {
            borrower,
            liquidated_debt: debt_reduction, 
            liquidated_coll: collateral_reduction,
        };

        // Log the liquidation event
        info!("Liquidation occurred for borrower: {:?}", borrower);

        Some(liquidation)
    }

    // Access control method
    pub fn ensure_factory_or_approved(&self, caller: String) {
        assert!(
            caller == self.factory || self.is_approved_delegate(self.factory.clone(), caller),
            "Unauthorized access"
        );
    }

    fn is_approved_delegate(&self, factory: String, caller: String) -> bool {
        caller == factory
    }
}

mod FinancialMath {
    pub fn calculate_debt_reduction() -> u64 {
        //  for actual implementation
        // This function should calculate the debt reduction based on the logic in LiquidationManager.sol
        100
    }

    pub fn calculate_collateral_reduction() -> u64 {
        // for actual implementation
        // This function should calculate the collateral reduction based on the logic in LiquidationManager.sol
        50
    }

    pub fn calculate_coll_gas_compensation(coll: u64) -> u64 {
        // for actual implementation
        // This function should calculate the collateral gas compensation based on the logic in LiquidationManager.sol
        coll / 100 // Example: 1% of the collateral
    }

    pub fn calculate_debt_gas_compensation() -> u64 {
        //  for actual implementation
        // This function should calculate the debt gas compensation based on the logic in LiquidationManager.sol
        10 // Eample gas
    }
}