pub trait AggregatorV3Interface {
    fn decimals(&self) -> u8;

    fn description(&self) -> String;

    fn version(&self) -> u128;

    // getRoundData and latestRoundData should both raise "No data present"
    // if they do not have data to report, instead of returning unset values
    // which could be misinterpreted as actual reported values.
    
    fn get_round_data(
        &self,
        round_id: u128,
    ) -> (u128, i128, u128, u128, u128);

    fn latest_round_data(
        &self,
    ) -> (u128, i128, u128, u128, u128);
}
