struct BorrowerOperationsState {
    min_net_debt: u64,
    // Additional state variables can be added here
}

impl BorrowerOperationsState {
    pub fn new(min_net_debt: u64) -> Self {
        Self { min_net_debt }
    }

    pub fn adjust_trove(&mut self, coll_change: u64, is_coll_increase: bool) {
        // Implement the logic to adjust the trove
        
        if is_coll_increase {
            println!("Increasing collateral by {}", coll_change);
        } else {
            println!("Decreasing collateral by {}", coll_change);
        }
    }
}

fn main() {
    let mut borrower_ops = BorrowerOperationsState::new(1000);
    borrower_ops.adjust_trove(500, true); 
}