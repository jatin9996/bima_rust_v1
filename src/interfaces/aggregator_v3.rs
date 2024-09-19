use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct RoundData {
    pub round_id: u128,
    pub answer: i128,
    pub started_at: u128,
    pub updated_at: u128,
    pub answered_in_round: u128,
}

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
    ) -> RoundData;

    fn latest_round_data(
        &self,
    ) -> RoundData;
}
