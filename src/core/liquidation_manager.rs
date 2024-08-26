#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod liquidation_manager {
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::SpreadAllocate,
    };

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct LiquidationManager {
        stability_pool: AccountId,
        sorted_troves: AccountId,
        borrower_operations: AccountId,
        factory: AccountId,
        enabled_trove_managers: StorageHashMap<AccountId, bool>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Liquidation {
        borrower: AccountId,
        liquidated_debt: Balance,
        liquidated_coll: Balance,
    }

    impl LiquidationManager {
        #[ink(constructor)]
        pub fn new(factory: AccountId) -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.factory = factory;
                contract.enabled_trove_managers = StorageHashMap::new();
            })
        }

        #[ink(message)]
        pub fn enable_trove_manager(&mut self, trove_manager: AccountId) {
            assert_eq!(self.env().caller(), self.factory, "Only factory can enable a trove manager");
            self.enabled_trove_managers.insert(trove_manager, true);
        }

        #[ink(message)]
        pub fn liquidate(&mut self, trove_manager: AccountId, borrower: AccountId) -> Option<Liquidation> {
            let is_enabled = *self.enabled_trove_managers.get(&trove_manager).unwrap_or(&false);
            if !is_enabled {
                return None;
            }

            let liquidation = Liquidation {
                borrower,
                liquidated_debt: 1000,  // Example values
                liquidated_coll: 500,
            };

            Some(liquidation)
        }
    }
}