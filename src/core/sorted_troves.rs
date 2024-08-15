use std::collections::HashMap;

struct Node {
    exists: bool,
    next_id: Option<String>, // Using String to represent addresses
    prev_id: Option<String>,
}

struct SortedTroves {
    trove_manager: String, 
    data: Data,
}

struct Data {
    head: Option<String>,
    tail: Option<String>,
    size: usize,
    nodes: HashMap<String, Node>,
}

impl SortedTroves {
    pub fn new(trove_manager: String) -> Self {
        SortedTroves {
            trove_manager,
            data: Data {
                head: None,
                tail: None,
                size: 0,
                nodes: HashMap::new(),
            },
        }
    }

    pub fn insert(&mut self, id: String, prev_id: Option<String>, next_id: Option<String>) {
        let node = Node {
            exists: true,
            next_id: next_id.clone(),
            prev_id: prev_id.clone(),
        };

        if let Some(prev_id) = prev_id {
            if let Some(prev_node) = self.data.nodes.get_mut(&prev_id) {
                prev_node.next_id = Some(id.clone());
            }
        } else {
            self.data.head = Some(id.clone());
        }

        if let Some(next_id) = next_id {
            if let Some(next_node) = self.data.nodes.get_mut(&next_id) {
                next_node.prev_id = Some(id.clone());
            }
        } else {
            self.data.tail = Some(id.clone());
        }

        self.data.nodes.insert(id, node);
        self.data.size += 1;
    }

    pub fn remove(&mut self, id: String) {
        if let Some(node) = self.data.nodes.remove(&id) {
            if let Some(prev_id) = node.prev_id {
                if let Some(prev_node) = self.data.nodes.get_mut(&prev_id) {
                    prev_node.next_id = node.next_id.clone();
                }
            } else {
                self.data.head = node.next_id.clone();
            }

            if let Some(next_id) = node.next_id {
                if let Some(next_node) = self.data.nodes.get_mut(&next_id) {
                    next_node.prev_id = node.prev_id.clone();
                }
            } else {
                self.data.tail = node.prev_id.clone();
            }

            self.data.size -= 1;
        }
    }

    // Checks if the list is empty
    pub fn is_empty(&self) -> bool {
        self.data.size == 0
    }

    // Checks if a trove with the given id exists
    pub fn contains(&self, id: &String) -> bool {
        self.data.nodes.contains_key(id)
    }

    // Re-inserts a trove with updated data, typically used after modifications that may affect sorting
    pub fn re_insert(&mut self, id: String, new_prev_id: Option<String>, new_next_id: Option<String>) {
        // Remove the node and then insert it again with updated links
        self.remove(id.clone());
        self.insert(id, new_prev_id, new_next_id);
    }
}

fn main() {
    let mut troves = SortedTroves::new("trove_manager_address".to_string());
    troves.insert("node1".to_string(), None, None);
    troves.insert("node2".to_string(), Some("node1".to_string()), None);
    troves.remove("node1".to_string());
}