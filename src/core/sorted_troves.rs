#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod sorted_troves {
    use ink_storage::{
        collections::HashMap as StorageMap,
        traits::{PackedLayout, SpreadLayout},
    };

    #[derive(Debug, Clone, PartialEq, Eq, PackedLayout, SpreadLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Node {
        exists: bool,
        next_id: Option<AccountId>,
        prev_id: Option<AccountId>,
    }

    #[ink(storage)]
    pub struct SortedTroves {
        head: Option<AccountId>,
        tail: Option<AccountId>,
        size: u32,
        nodes: StorageMap<AccountId, Node>,
    }

    impl SortedTroves {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                head: None,
                tail: None,
                size: 0,
                nodes: StorageMap::new(),
            }
        }

        #[ink(message)]
        pub fn insert(&mut self, id: AccountId, prev_id: Option<AccountId>, next_id: Option<AccountId>) {
            let node = Node {
                exists: true,
                next_id: next_id.clone(),
                prev_id: prev_id.clone(),
            };

            if let Some(prev_id) = prev_id {
                if let Some(prev_node) = self.nodes.get_mut(&prev_id) {
                    prev_node.next_id = Some(id);
                }
            } else {
                self.head = Some(id);
            }

            if let Some(next_id) = next_id {
                if let Some(next_node) = self.nodes.get_mut(&next_id) {
                    next_node.prev_id = Some(id);
                }
            } else {
                self.tail = Some(id);
            }

            self.nodes.insert(id, node);
            self.size += 1;
        }

        #[ink(message)]
        pub fn remove(&mut self, id: AccountId) {
            if let Some(node) = self.nodes.take(&id) {
                if let Some(prev_id) = node.prev_id {
                    if let Some(prev_node) = self.nodes.get_mut(&prev_id) {
                        prev_node.next_id = node.next_id;
                    }
                } else {
                    self.head = node.next_id;
                }

                if let Some(next_id) = node.next_id {
                    if let Some(next_node) = self.nodes.get_mut(&next_id) {
                        next_node.prev_id = node.prev_id;
                    }
                } else {
                    self.tail = node.prev_id;
                }

                self.size -= 1;
            }
        }

        #[ink(message)]
        pub fn is_empty(&self) -> bool {
            self.size == 0
        }

        #[ink(message)]
        pub fn contains(&self, id: AccountId) -> bool {
            self.nodes.contains_key(&id)
        }

        #[ink(message)]
        pub fn re_insert(&mut self, id: AccountId, new_prev_id: Option<AccountId>, new_next_id: Option<AccountId>) {
            self.remove(id);
            self.insert(id, new_prev_id, new_next_id);
        }
    }
}