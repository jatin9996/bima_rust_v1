#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_storage::{
    collections::Vec as InkVec,
    traits::{PackedLayout, SpreadLayout},
};

#[ink::contract]
mod factory {
    use super::*;
    use ink_storage::{
        collections::Vec as InkVec,
        traits::{PackedLayout, SpreadLayout},
    };

    #[ink(storage)]
    pub struct Factory {
        babel_ownable: BabelOwnable,
        debt_token: DebtToken,
        stability_pool: Box<dyn IStabilityPool>,
        liquidation_manager: Box<dyn ILiquidationManager>,
        borrower_operations: Box<dyn BorrowerOperations>,
        sorted_troves_impl: String,
        trove_manager_impl: String,
        trove_managers: InkVec<String>,
    }

    impl Factory {
        #[ink(constructor)]
        pub fn new(
            babel_core: AccountId,
            debt_token: DebtToken,
            stability_pool: Box<dyn IStabilityPool>,
            borrower_operations: Box<dyn BorrowerOperations>,
            sorted_troves_impl: String,
            trove_manager_impl: String,
            liquidation_manager: Box<dyn ILiquidationManager>,
        ) -> Self {
            Self {
                babel_ownable: BabelOwnable::new(babel_core),
                debt_token,
                stability_pool,
                liquidation_manager,
                borrower_operations,
                sorted_troves_impl,
                trove_manager_impl,
                trove_managers: InkVec::new(),
            }
        }

        #[ink(message)]
        pub fn deploy_new_instance(&mut self, collateral: String, price_feed: String, params: DeploymentParams) {
            let trove_manager = self.clone_contract(&self.trove_manager_impl);
            self.trove_managers.push(trove_manager);

            let sorted_troves = self.clone_contract(&self.sorted_troves_impl);

            // Assuming the TroveManager and SortedTroves have methods to set up their state
            let tm = TroveManager::new(); // You would need to modify this according to actual implementation
            tm.set_addresses(price_feed, sorted_troves, collateral);

            ink_env::debug_println!("Deployed new TroveManager and SortedTroves for collateral: {}", collateral);
        }

        fn clone_contract(&mut self, implementation: &String) -> String {
            let new_id = format!("{}_instance_{}", implementation, self.trove_managers.len() + 1);
            new_id
        }
    }
}