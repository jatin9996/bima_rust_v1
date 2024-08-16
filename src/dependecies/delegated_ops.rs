#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod delegated_ops {
    use ink_storage::traits::SpreadAllocate;
    use ink_storage::Mapping;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct DelegatedOps {
        is_approved_delegate: Mapping<(AccountId, AccountId), bool>,
    }

    impl DelegatedOps {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|_| {})
        }

        #[ink(message)]
        pub fn set_delegate_approval(&mut self, delegate: AccountId, is_approved: bool) {
            let caller = self.env().caller();
            self.is_approved_delegate.insert((caller, delegate), &is_approved);
        }

        #[ink(message)]
        pub fn is_approved_delegate(&self, owner: AccountId, caller: AccountId) -> bool {
            self.is_approved_delegate.get(&(owner, caller)).unwrap_or(false)
        }

        fn ensure_caller_or_delegated(&self, account: AccountId) {
            let caller = self.env().caller();
            assert!(
                caller == account || self.is_approved_delegate(account, caller),
                "Delegate not approved"
            );
        }
    }
}


