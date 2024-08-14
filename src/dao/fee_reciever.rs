use std::collections::HashMap;

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
    owner: Address,
    tokens: HashMap<String, Token>, // Token identifier mapped to Token struct
}

impl FeeReceiver {
    fn new(owner: Address) -> Self {
        FeeReceiver {
            owner,
            tokens: HashMap::new(),
        }
    }

    fn transfer_token(&mut self, token_id: &str, receiver: Address, amount: u128) -> Result<(), String> {
        if self.owner != "owner_address_here" { // Replace with actual owner address
            return Err("Unauthorized".to_string());
        }
        match self.tokens.get_mut(token_id) {
            Some(token) => token.transfer("owner_address_here".to_string(), receiver, amount), // Replace with actual owner address
            None => Err("Token not found".to_string()),
        }
    }

    fn set_token_approval(&mut self, token_id: &str, spender: Address, amount: u128) -> Result<(), String> {
        if self.owner != "owner_address_here" { // Replace with actual owner address
            return Err("Unauthorized".to_string());
        }
        match self.tokens.get_mut(token_id) {
            Some(token) => {
                token.approve("owner_address_here".to_string(), spender, amount); // Replace with actual owner address
                Ok(())
            },
            None => Err("Token not found".to_string()),
        }
    }
}
