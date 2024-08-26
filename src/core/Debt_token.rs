#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod debt_token {
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
    };

    #[ink(storage)]
    pub struct DebtToken {
        name: ink_prelude::string::String,
        symbol: ink_prelude::string::String,
        total_supply: Balance,
        balances: StorageHashMap<AccountId, Balance>,
        allowances: StorageHashMap<(AccountId, AccountId), Balance>,
    }

    pub type AccountId = ink_env::AccountId;
    pub type Balance = u128;

    impl DebtToken {
        #[ink(constructor)]
        pub fn new(name: ink_prelude::string::String, symbol: ink_prelude::string::String) -> Self {
            Self {
                name,
                symbol,
                total_supply: 0,
                balances: StorageHashMap::new(),
                allowances: StorageHashMap::new(),
            }
        }

        #[ink(message)]
        pub fn mint(&mut self, account: AccountId, amount: Balance) {
            let balance = self.balances.entry(account).or_insert(0);
            *balance += amount;
            self.total_supply += amount;
        }

        #[ink(message)]
        pub fn burn(&mut self, account: AccountId, amount: Balance) {
            let balance = self.balances.entry(account).or_default();
            if *balance < amount {
                ink_env::panic("Insufficient balance");
            }
            *balance -= amount;
            self.total_supply -= amount;
        }

        #[ink(message)]
        pub fn approve(&mut self, owner: AccountId, spender: AccountId, amount: Balance) {
            self.allowances.insert((owner, spender), amount);
        }

        #[ink(message)]
        pub fn transfer(&mut self, from: AccountId, to: AccountId, amount: Balance) {
            let from_balance = self.balances.entry(from).or_default();
            if *from_balance < amount {
                ink_env::panic("Insufficient balance");
            }
            *from_balance -= amount;

            let to_balance = self.balances.entry(to).or_insert(0);
            *to_balance += amount;
        }
    }
}