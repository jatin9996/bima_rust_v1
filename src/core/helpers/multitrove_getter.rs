use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CombinedTroveData {
    owner: String, // Assuming AccountId is a String for simplicity
    debt: u128,
    coll: u128,
    stake: u128,
    snapshot_collateral: u128,
    snapshot_debt: u128,
}

pub struct TroveManager {
    troves: HashMap<String, (u128, u128, u128)>,
    reward_snapshots: HashMap<String, (u128, u128)>,
}

impl TroveManager {
    pub fn new() -> Self {
        Self {
            troves: HashMap::new(),
            reward_snapshots: HashMap::new(),
        }
    }

    pub fn get_trove_data(&self, owner: &String) -> Option<(u128, u128, u128)> {
        self.troves.get(owner).copied()
    }

    pub fn get_reward_snapshot(&self, owner: &String) -> Option<(u128, u128)> {
        self.reward_snapshots.get(owner).copied()
    }
}

pub struct MultiTroveGetter {
    trove_manager: TroveManager,
}

impl MultiTroveGetter {
    pub fn new() -> Self {
        Self { 
            trove_manager: TroveManager::new() 
        }
    }

    pub fn get_multiple_sorted_troves(&self, start_idx: usize, count: usize) -> Vec<CombinedTroveData> {
        let mut troves_data = Vec::new();
        let keys: Vec<_> = self.trove_manager.troves.keys().cloned().collect();
        for owner in keys.iter().skip(start_idx).take(count) {
            if let Some((debt, coll, stake)) = self.trove_manager.get_trove_data(owner) {
                if let Some((snapshot_collateral, snapshot_debt)) = self.trove_manager.get_reward_snapshot(owner) {
                    troves_data.push(CombinedTroveData {
                        owner: owner.clone(),
                        debt,
                        coll,
                        stake,
                        snapshot_collateral,
                        snapshot_debt,
                    });
                }
            }
        }
        troves_data
    }
}