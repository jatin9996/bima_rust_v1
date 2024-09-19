use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct GaugeController {
    // Add fields here
}

pub trait IGaugeController {
    // Votes for gauge weights.
    fn vote_for_gauge_weights(&self, gauge: &str, weight: u128);

    // Returns the type of a gauge.
    fn gauge_types(&self, gauge: &str) -> i128;
}

// Implement the trait for the struct
impl IGaugeController for GaugeController {
    fn vote_for_gauge_weights(&self, gauge: &str, weight: u128) {
        // Implementation here
    }

    fn gauge_types(&self, gauge: &str) -> i128 {
        // Implementation here
    }
}
