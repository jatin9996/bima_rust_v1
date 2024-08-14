use std::collections::HashMap;

#[derive(Debug, Clone)]
struct BabelToken {
    total_supply: u128,
    balances: HashMap<String, u128>,
}

impl BabelToken {
    fn new(total_supply: u128) -> Self {
        let mut balances = HashMap::new();
        balances.insert("vault".to_string(), total_supply);
        BabelToken { total_supply, balances }
    }

    fn transfer(&mut self, from: &str, to: &str, amount: u128) {
        let from_balance = self.balances.get(from).cloned().unwrap_or(0);
        let to_balance = self.balances.get(to).cloned().unwrap_or(0);
        self.balances.insert(from.to_string(), from_balance - amount);
        self.balances.insert(to.to_string(), to_balance + amount);
    }
}

struct Vault {
    babel_token: BabelToken,
    unallocated_total: u128,
    weekly_emissions: HashMap<u64, u128>,
    allocated: HashMap<String, u128>,
}

impl Vault {
    fn new(babel_token: BabelToken) -> Self {
        Vault {
            babel_token,
            unallocated_total: babel_token.total_supply,
            weekly_emissions: HashMap::new(),
            allocated: HashMap::new(),
        }
    }

    fn set_weekly_emission(&mut self, week: u64, amount: u128) {
        self.weekly_emissions.insert(week, amount);
        self.unallocated_total -= amount;
    }

    fn transfer_tokens(&mut self, receiver: &str, amount: u128) {
        self.babel_token.transfer("vault", receiver, amount);
        self.unallocated_total -= amount;
    }

    fn increase_unallocated_supply(&mut self, amount: u128) {
        self.unallocated_total += amount;
        self.babel_token.balances.insert("vault".to_string(), self.unallocated_total);
    }
}

fn main() {
    let mut babel_token = BabelToken::new(1_000_000);
    let mut vault = Vault::new(babel_token);

    vault.set_weekly_emission(1, 10000);
    vault.transfer_tokens("user1", 5000);
    vault.increase_unallocated_supply(2000);

    println!("{:?}", vault);
}
