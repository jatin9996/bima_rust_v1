#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod system_start {
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct SystemStart {
        start_time: u64,
    }

    impl SystemStart {
        #[ink(constructor)]
        pub fn new(babel_core: AccountId) -> Self {
            let babel_core_instance: BabelCore = ink_env::call::FromAccountId::from_account_id(babel_core);
            let start_time = babel_core_instance.start_time();

            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.start_time = start_time;
            })
        }

        #[ink(message)]
        pub fn get_week(&self) -> u64 {
            let current_time = self.env().block_timestamp();
            (current_time - self.start_time) / (7 * 24 * 60 * 60 * 1000)
        }
    }

    #[ink::trait_definition]
    pub trait IBabelCore {
        #[ink(message)]
        fn start_time(&self) -> u64;
    }

    // Assuming there's a BabelCore contract which implements IBabelCore
    #[ink(storage)]
    pub struct BabelCore {
        start_time: u64,
    }

    impl BabelCore {
        #[ink(constructor)]
        pub fn new(start_time: u64) -> Self {
            Self { start_time }
        }

        #[ink(message)]
        pub fn start_time(&self) -> u64 {
            self.start_time
        }
    }
}
/*
the entry point of the code i lib.rs

#![cfg_attr(not(feature = "std"), no_std)]

mod babel_ownable;
mod delegated_ops;
mod system_start;

*/