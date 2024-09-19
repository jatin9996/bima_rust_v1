use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct SortedTrove {
    id: String,
    nicr: u256,
    prev_id: String,
    next_id: String,
}

pub trait ISortedTroves {
    /// Event triggered when a node is added.
    fn node_added(&self, id: &str, nicr: u256);

    /// Event triggered when a node is removed.
    fn node_removed(&self, id: &str);

    /// Inserts a new node into the sorted list.
    fn insert(&self, id: &str, nicr: u256, prev_id: &str, next_id: &str);

    /// Re-inserts an existing node into the sorted list with a new NICR value.
    fn re_insert(&self, id: &str, new_nicr: u256, prev_id: &str, next_id: &str);

    /// Removes a node from the sorted list.
    fn remove(&self, id: &str);

    /// Sets the address of the trove manager.
    fn set_addresses(&self, trove_manager_address: &str);

    /// Checks if a node exists in the sorted list.
    fn contains(&self, id: &str) -> bool;

    /// Returns the current state of the sorted list.
    fn data(&self) -> (String, String, u256); // head, tail, size

    /// Finds the position to insert a node with a given NICR value.
    fn find_insert_position(&self, nicr: u256, prev_id: &str, next_id: &str) -> (String, String);

    /// Returns the address of the first node.
    fn get_first(&self) -> String;

    /// Returns the address of the last node.
    fn get_last(&self) -> String;

    /// Returns the address of the next node after the specified node.
    fn get_next(&self, id: &str) -> String;

    /// Returns the address of the previous node before the specified node.
    fn get_prev(&self, id: &str) -> String;

    /// Returns the size of the sorted list.
    fn get_size(&self) -> u256;

    /// Checks if the sorted list is empty.
    fn is_empty(&self) -> bool;

    /// Returns the address of the trove manager.
    fn trove_manager(&self) -> String;

    /// Checks if the position is valid for insertion.
    fn valid_insert_position(&self, nicr: u256, prev_id: &str, next_id: &str) -> bool;
}
