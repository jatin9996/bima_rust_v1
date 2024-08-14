#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod babel_ownable {
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct BabelOwnable {
        babel_core: AccountId,
    }

    impl BabelOwnable {
        #[ink(constructor)]
        pub fn new(babel_core: AccountId) -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.babel_core = babel_core;
            })
        }

        #[ink(message)]
        pub fn owner(&self) -> AccountId {
            self.babel_core
        }

        #[ink(message)]
        pub fn guardian(&self) -> AccountId {
            // Assuming you have a method to get the guardian from the BabelCore contract
            // Here you would call that method and return the guardian's AccountId
            unimplemented!()
        }

        #[ink(message)]
        pub fn only_owner(&self) {
            let caller = self.env().caller();
            assert_eq!(caller, self.babel_core, "Only owner");
        }
    }

    #[ink::trait_definition]
    pub trait IBabelCore {
        #[ink(message)]
        fn owner(&self) -> AccountId;

        #[ink(message)]
        fn guardian(&self) -> AccountId;
    }
}

