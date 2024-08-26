#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod multitrove_getter {
    use ink_storage::{
        collections::HashMap as StorageMap,
        traits::{PackedLayout, SpreadLayout},
    };

    #[derive(Debug, Clone, PartialEq, Eq, ink_storage::traits::SpreadLayout, ink_storage::traits::PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout))]
    pub struct CombinedTroveData {
        owner: AccountId,
        debt: u128,
        coll: u128,
        stake: u128,
        snapshot_collateral: u128,
        snapshot_debt: u128,
    }

    #[ink(storage)]
    pub struct TroveManager {
        troves: StorageMap<AccountId, (u128, u128, u128)>,
        reward_snapshots: StorageMap<AccountId, (u128, u128)>,
    }

    impl TroveManager {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                troves: StorageMap::new(),
                reward_snapshots: StorageMap::new(),
            }
        }

        #[ink(message)]
        pub fn get_trove_data(&self, owner: AccountId) -> Option<(u128, u128, u128)> {
            self.troves.get(&owner).copied()
        }

        #[ink(message)]
        pub fn get_reward_snapshot(&self, owner: AccountId) -> Option<(u128, u128)> {
            self.reward_snapshots.get(&owner).copied()
        }
    }

    #[ink(storage)]
    pub struct MultiTroveGetter {
        trove_manager: TroveManager,
    }

    impl MultiTroveGetter {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                trove_manager: TroveManager::new() 
            }
        }

        #[ink(message)]
        pub fn get_multiple_sorted_troves(&self, start_idx: u32, count: u32) -> Vec<CombinedTroveData> {
            let mut troves_data = Vec::new();
            let keys: Vec<_> = self.trove_manager.troves.keys().cloned().collect();
            for owner in keys.iter().skip(start_idx as usize).take(count as usize) {
                if let Some((debt, coll, stake)) = self.trove_manager.get_trove_data(*owner) {
                    if let Some((snapshot_collateral, snapshot_debt)) = self.trove_manager.get_reward_snapshot(*owner) {
                        troves_data.push(CombinedTroveData {
                            owner: *owner,
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
}