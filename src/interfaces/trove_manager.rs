pub struct TroveManager {
    troves: HashMap<String, Trove>, // Troves mapped by user ID
}

struct Trove {
    collateral: u64,
    debt: u64,
}

impl TroveManager {
    pub fn new() -> Self {
        Self {
            troves: HashMap::new(),
        }
    }

    pub fn create_trove(&mut self, user_id: String, collateral: u64, debt: u64) {
        let trove = Trove { collateral, debt };
        self.troves.insert(user_id, trove);
    }

    pub fn adjust_trove(&mut self, user_id: String, collateral_change: i64, debt_change: i64) {
        if let Some(trove) = self.troves.get_mut(&user_id) {
            trove.collateral = ((trove.collateral as i64) + collateral_change) as u64;
            trove.debt = ((trove.debt as i64) + debt_change) as u64;
        }
    }
}