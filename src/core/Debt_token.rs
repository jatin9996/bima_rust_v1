use std::collections::HashMap;
use crate::interfaces::babel_core::BabelCore; // Import the BabelCore trait

type AccountId = String;
type Balance = u128;

#[derive(Debug)]
struct DebtToken<'a, T: BabelCore> {
    name: String,
    symbol: String,
    total_supply: Balance,
    balances: HashMap<AccountId, Balance>,
    allowances: HashMap<(AccountId, AccountId), Balance>,
    babel_core: &'a T, // Reference to an implementation of BabelCore
}

impl<'a, T: BabelCore> DebtToken<'a, T> {
    fn new(name: String, symbol: String, babel_core: &'a T) -> Self {
        Self {
            name,
            symbol,
            total_supply: 0,
            balances: HashMap::new(),
            allowances: HashMap::new(),
            babel_core,
        }
    }

    fn mint(&mut self, account: AccountId, amount: Balance) {
        let balance = self.balances.entry(account.clone()).or_insert(0);
        *balance += amount;
        self.total_supply += amount;
        println!("Minted {} tokens for {}", amount, account);
        self.babel_core.emit_fee_receiver_set("FeeReceiver"); // Example of using BabelCore trait
    }

    fn burn(&mut self, account: AccountId, amount: Balance) {
        let balance = self.balances.entry(account.clone()).or_default();
        if *balance < amount {
            panic!("Insufficient balance");
        }
        *balance -= amount;
        self.total_supply -= amount;
        println!("Burned {} tokens from {}", amount, account);
        self.babel_core.emit_guardian_set("Guardian"); // Example of using BabelCore trait
    }

    fn approve(&mut self, owner: AccountId, spender: AccountId, amount: Balance) {
        self.allowances.insert((owner, spender), amount);
        println!("Approval granted to spend {} tokens", amount);
    }

    fn transfer(&mut self, from: AccountId, to: AccountId, amount: Balance) {
        let from_balance = self.balances.entry(from.clone()).or_default();
        if *from_balance < amount {
            panic!("Insufficient balance");
        }
        *from_balance -= amount;

        let to_balance = self.balances.entry(to.clone()).or_insert(0);
        *to_balance += amount;

        println!("Transferred {} tokens from {} to {}", amount, from, to);
        self.babel_core.emit_new_owner_accepted(&from, &to); // Example of using BabelCore trait
    }
}

fn main() {
    // Assuming there's an implementation of BabelCore available
    let babel_core = /* get your BabelCore implementation here */;
    let mut token = DebtToken::new("DebtToken".to_string(), "DT".to_string(), &babel_core);
    token.mint("Alice".to_string(), 1000);
    token.burn("Alice".to_string(), 200);
    token.approve("Alice".to_string(), "Bob".to_string(), 300);
    token.transfer("Alice".to_string(), "Bob".to_string(), 300);
}