pub trait IGaugeController {
    // Votes for gauge weights.
    fn vote_for_gauge_weights(&self, gauge: &str, weight: u128);

    // Returns the type of a gauge.
    fn gauge_types(&self, gauge: &str) -> i128;
}
