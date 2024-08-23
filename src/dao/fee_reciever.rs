use std::collections::HashMap;
use crate::dependecies::babel_ownable::{BabelOwnable, IBabelCore}; // Import the BabelOwnable and IBabelCore

struct Token {
    balances: HashMap<Address, u128>,
    allowances: HashMap<(Address, Address), u128>,
}

impl Token {
    fn new() -> Self {
        Token {
            balances: HashMap::new(),
            allowances: HashMap::new(),
        }
    }

    fn transfer(&mut self, from: Address, to: Address, amount: u128) -> Result<(), String> {
        let balance = self.balances.get(&from).cloned().unwrap_or(0);
        if balance < amount {
            return Err("Insufficient balance".to_string());
        }
        self.balances.insert(from, balance - amount);
        let recipient_balance = self.balances.get(&to).cloned().unwrap_or(0);
        self.balances.insert(to, recipient_balance + amount);
        Ok(())
    }

    fn approve(&mut self, owner: Address, spender: Address, amount: u128) {
        self.allowances.insert((owner, spender), amount);
    }
}

type Address = String; // Simplified address type

struct FeeReceiver {
    babel_ownable: BabelOwnable, // Use BabelOwnable for ownership management
    tokens: HashMap<String, Token>, // Token identifier mapped to Token struct
}

impl FeeReceiver {
    fn new(owner: Address) -> Self {
        FeeReceiver {
            babel_ownable: BabelOwnable::new(owner), // Initialize BabelOwnable with the owner
            tokens: HashMap::new(),
        }
    }

    fn transfer_token(&mut self, token_id: &str, receiver: Address, amount: u128) -> Result<(), String> {
        self.babel_ownable.only_owner(); // Use only_owner to check ownership
        match self.tokens.get_mut(token_id) {
            Some(token) => token.transfer(self.babel_ownable.owner(), receiver, amount),
            None => Err("Token not found".to_string()),
        }
    }

    fn set_token_approval(&mut self, token_id: &str, spender: Address, amount: u128) -> Result<(), String> {
        self.babel_ownable.only_owner(); // Use only_owner to check ownership
        match self.tokens.get_mut(token_id) {
            Some(token) => {
                token.approve(self.babel_ownable.owner(), spender, amount);
                Ok(())
            },
            None => Err("Token not found".to_string()),
        }
    }
}