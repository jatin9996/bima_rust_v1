use std::collections::HashMap;

struct Token {
    balances: HashMap<AccountId, Balance>,
    allowances: HashMap<(AccountId, AccountId), Balance>,
}

impl Token {
    fn new() -> Self {
        Self {
            balances: HashMap::new(),
            allowances: HashMap::new(),
        }
    }

    fn transfer(&mut self, from: AccountId, to: AccountId, amount: Balance) -> Result<(), String> {
        let from_balance = *self.balances.get(&from).unwrap_or(&0);
        if from_balance < amount {
            return Err("Insufficient balance".to_string());
        }
        self.balances.insert(from, from_balance - amount);
        let to_balance = *self.balances.get(&to).unwrap_or(&0);
        self.balances.insert(to, to_balance + amount);
        Ok(())
    }

    fn approve(&mut self, owner: AccountId, spender: AccountId, amount: Balance) {
        self.allowances.insert((owner, spender), amount);
    }
}

struct FeeReceiver {
    owner: AccountId,
    tokens: HashMap<String, Token>,
}

impl FeeReceiver {
    fn new(owner: AccountId) -> Self {
        Self {
            owner,
            tokens: HashMap::new(),
        }
    }

    fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<(), String> {
        self.only_owner()?;
        self.owner = new_owner;
        Ok(())
    }

    fn transfer_token(&mut self, token_id: String, receiver: AccountId, amount: Balance) -> Result<(), String> {
        self.only_owner()?;
        let token = self.tokens.get_mut(&token_id).ok_or("Token not found".to_string())?;
        let from_balance = *token.balances.get(&self.owner).unwrap_or(&0);
        if from_balance < amount {
            return Err("Insufficient balance".to_string());
        }
        // Ensure no underflow occurs
        let to_balance = *token.balances.get(&receiver).unwrap_or(&0);
        token.balances.insert(self.owner, from_balance - amount);
        token.balances.insert(receiver, to_balance + amount);
        Ok(())
    }

    fn set_token_approval(&mut self, token_id: String, spender: AccountId, amount: Balance) -> Result<(), String> {
        self.only_owner()?;
        let token = self.tokens.get_mut(&token_id).ok_or("Token not found".to_string())?;
        token.allowances.insert((self.owner, spender), amount);
        Ok(())
    }

    fn only_owner(&self) -> Result<(), String> {
        if self.env_caller() != self.owner {
            return Err("Caller is not owner".to_string());
        }
        Ok(())
    }

    fn env_caller(&self) -> AccountId {
        // Mock implementation, replace with actual environment caller retrieval
        self.owner
    }
}

type AccountId = String;
type Balance = u32;