#![cfg_attr(not(feature = "std"), no_std)]

use std::collections::HashMap;
use secp256k1::{Secp256k1, Message, PublicKey, Signature};

pub struct DebtToken {
    name: String,
    symbol: String,
    total_supply: Balance,
    balances: HashMap<AccountId, Balance>,
    allowances: HashMap<(AccountId, AccountId), Balance>,
    collateral: HashMap<AccountId, Balance>,
    debt: HashMap<AccountId, Balance>,
}

pub type AccountId = String; // Simplified for core Rust
pub type Balance = u128;

impl DebtToken {
    pub fn new(name: String, symbol: String) -> Self {
        Self {
            name,
            symbol,
            total_supply: 0,
            balances: HashMap::new(),
            allowances: HashMap::new(),
            collateral: HashMap::new(),
            debt: HashMap::new(),
        }
    }

    pub fn mint(&mut self, account: AccountId, amount: Balance) {
        let balance = self.balances.entry(account).or_insert(0);
        *balance += amount;
        self.total_supply += amount;
    }

    pub fn burn(&mut self, account: AccountId, amount: Balance) {
        let balance = self.balances.entry(account).or_default();
        if *balance < amount {
            panic!("Insufficient balance");
        }
        *balance -= amount;
        self.total_supply -= amount;
    }

    pub fn approve(&mut self, owner: AccountId, spender: AccountId, amount: Balance) {
        self.allowances.insert((owner, spender), amount);
    }

    pub fn transfer(&mut self, from: AccountId, to: AccountId, amount: Balance) {
        let from_balance = self.balances.entry(from).or_default();
        if *from_balance < amount {
            panic!("Insufficient balance");
        }
        *from_balance -= amount;

        let to_balance = self.balances.entry(to).or_insert(0);
        *to_balance += amount;
    }

    pub fn flash_loan(&mut self, amount: Balance, callback: impl FnOnce(Balance) -> bool) {
        let initial_supply = self.total_supply;
        self.total_supply += amount; // Temporarily mint the amount

        if callback(amount) {
            self.total_supply = initial_supply; // Revert the minted amount
        } else {
            panic!("Flash loan failed to return the tokens");
        }
    }

    pub fn add_collateral(&mut self, user: AccountId, amount: Balance) {
        let collateral_balance = self.collateral.entry(user).or_insert(0);
        *collateral_balance += amount;
    }

    pub fn issue_debt(&mut self, user: AccountId, amount: Balance) {
        let debt_balance = self.debt.entry(user.clone()).or_insert(0);
        let collateral_balance = *self.collateral.get(&user).unwrap_or(&0);

        if collateral_balance >= amount * 2 { // Ensure 200% collateralization
            *debt_balance += amount;
            self.total_supply += amount; // Mint debt tokens
            let user_balance = self.balances.entry(user).or_insert(0);
            *user_balance += amount;
        } else {
            panic!("Not enough collateral");
        }
    }

    pub fn verify_signature(&self, message: &[u8], sig: &[u8], pub_key: &[u8]) -> bool {
        let secp = Secp256k1::new();
        let message = Message::from_slice(message).expect("32 bytes");
        let sig = Signature::from_der(sig).expect("Signature in DER format");
        let pub_key = PublicKey::from_slice(pub_key).expect("Public key");

        secp.verify(&message, &sig, &pub_key).is_ok()
    }
}