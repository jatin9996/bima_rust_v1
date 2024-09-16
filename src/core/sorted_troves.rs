use std::collections::HashMap;
use borsh::{BorshDeserialize, BorshSerialize};
use crate::interfaces::trove_manager::TroveManager;

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Node {
    exists: bool,
    next_id: Option<u32>, // Assuming AccountId can be represented as u32 for simplicity
    prev_id: Option<u32>,
    nicr: u256, // Add NICR field
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct SortedTroves {
    head: Option<u32>,
    tail: Option<u32>,
    size: u32,
    nodes: HashMap<u32, Node>,
    trove_manager: Option<TroveManager>, // Add TroveManager field
}

impl SortedTroves {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
            nodes: HashMap::new(),
            trove_manager: None,
        }
    }

    pub fn set_addresses(&mut self, trove_manager: TroveManager) {
        assert!(self.trove_manager.is_none(), "Already set");
        self.trove_manager = Some(trove_manager);
    }

    fn require_caller_is_trove_manager(&self) {
        // Assuming we have a way to get the caller's address
        let caller = get_caller_address();
        assert!(self.trove_manager.as_ref().map_or(false, |tm| tm.address == caller), "Caller is not the TroveManager");
    }

    pub fn insert(&mut self, id: u32, nicr: u256, prev_id: Option<u32>, next_id: Option<u32>) {
        self.require_caller_is_trove_manager();

        let node = Node {
            exists: true,
            next_id,
            prev_id,
            nicr,
        };

        // Insert logic based on NICR
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

    pub fn remove(&mut self, id: u32) {
        self.require_caller_is_trove_manager();

        if let Some(node) = self.nodes.remove(&id) {
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

    pub fn re_insert(&mut self, id: u32, new_nicr: u256, new_prev_id: Option<u32>, new_next_id: Option<u32>) {
        self.remove(id);
        self.insert(id, new_nicr, new_prev_id, new_next_id);
    }

    // Additional methods for NICR-based sorting
    fn valid_insert_position(&self, nicr: u256, prev_id: Option<u32>, next_id: Option<u32>) -> bool {
        if let Some(prev_id) = prev_id {
            if let Some(prev_node) = self.nodes.get(&prev_id) {
                if prev_node.nicr > nicr {
                    return false;
                }
            } else {
                return false; // prev_id does not exist
            }
        }

        if let Some(next_id) = next_id {
            if let Some(next_node) = self.nodes.get(&next_id) {
                if next_node.nicr < nicr {
                    return false;
                }
            } else {
                return false; // next_id does not exist
            }
        }

        true
    }
    // Additional methods for NICR-based sorting
    fn valid_insert_position(&self, nicr: u256, prev_id: Option<u32>, next_id: Option<u32>) -> bool {
        if let Some(prev_id) = prev_id {
            if let Some(prev_node) = self.nodes.get(&prev_id) {
                if prev_node.nicr > nicr {
                    return false;
                }
            } else {
                return false; // prev_id does not exist
            }
        }

        if let Some(next_id) = next_id {
            if let Some(next_node) = self.nodes.get(&next_id) {
                if next_node.nicr < nicr {
                    return false;
                }
            } else {
                return false; // next_id does not exist
            }
        }

        true
    }

    // Serialization method
    pub fn serialize(&self) -> Vec<u8> {
        self.try_to_vec().expect("Serialization failed")
    }

    // Deserialization method
    pub fn deserialize(data: &[u8]) -> Self {
        Self::try_from_slice(data).expect("Deserialization failed")
    }
}