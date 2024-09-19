use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct DebtToken {
    total_supply: u64,
}

impl DebtToken {
    pub fn new() -> Self {
        Self {
            total_supply: 0,
        }
    }

    pub fn issue(&mut self, amount: u64) {
        self.total_supply += amount;
    }

    pub fn burn(&mut self, amount: u64) {
        if self.total_supply >= amount {
            self.total_supply -= amount;
        }
    }
}