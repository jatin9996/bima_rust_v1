#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_storage::{
    collections::HashMap as StorageMap,
    traits::{PackedLayout, SpreadLayout},
};

#[ink::contract]
mod fee_receiver {
    use super::*;

    #[ink(storage)]
    pub struct FeeReceiver {
        owner: AccountId,
        tokens: StorageMap<String, Token>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Token {
        balances: StorageMap<AccountId, Balance>,
        allowances: StorageMap<(AccountId, AccountId), Balance>,
    }

    impl Token {
        pub fn new() -> Self {
            Self {
                balances: StorageMap::new(),
                allowances: StorageMap::new(),
            }
        }

        pub fn transfer(&mut self, from: AccountId, to: AccountId, amount: Balance) -> Result<(), ink_env::Error> {
            let from_balance = self.balances.get(&from).copied().unwrap_or(0);
            if from_balance < amount {
                return Err(ink_env::Error::new(ink_env::ErrorCode::Custom(1)));
            }
            self.balances.insert(from, from_balance - amount);
            let to_balance = self.balances.get(&to).copied().unwrap_or(0);
            self.balances.insert(to, to_balance + amount);
            Ok(())
        }

        pub fn approve(&mut self, owner: AccountId, spender: AccountId, amount: Balance) {
            self.allowances.insert((owner, spender), amount);
        }
    }

    impl FeeReceiver {
        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            Self {
                owner,
                tokens: StorageMap::new(),
            }
        }

        #[ink(message)]
        pub fn transfer_token(&mut self, token_id: String, receiver: AccountId, amount: Balance) -> Result<(), ink_env::Error> {
            self.only_owner()?;
            let token = self.tokens.get_mut(&token_id).ok_or(ink_env::Error::new(ink_env::ErrorCode::Custom(2)))?;
            token.transfer(self.owner, receiver, amount)
        }

        #[ink(message)]
        pub fn set_token_approval(&mut self, token_id: String, spender: AccountId, amount: Balance) -> Result<(), ink_env::Error> {
            self.only_owner()?;
            let token = self.tokens.get_mut(&token_id).ok_or(ink_env::Error::new(ink_env::ErrorCode::Custom(2)))?;
            token.approve(self.owner, spender, amount);
            Ok(())
        }

        fn only_owner(&self) -> Result<(), ink_env::Error> {
            if self.env().caller() != self.owner {
                return Err(ink_env::Error::new(ink_env::ErrorCode::Custom(3)));
            }
            Ok(())
        }
    }
}