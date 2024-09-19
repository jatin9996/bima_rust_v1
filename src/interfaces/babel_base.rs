use borsh::{BorshDeserialize, BorshSerialize};

// Define the IBabelBase trait to 
pub trait IBabelBase {
    fn decimal_precision(&self) -> u128;

    fn ccr(&self) -> u128;

    fn debt_gas_compensation(&self) -> u128;

    fn percent_divisor(&self) -> u128;
}

//  struct implementing IBabelBase
#[derive(BorshSerialize, BorshDeserialize)]
pub struct BabelBaseImpl {
    pub decimal_precision: u128,
    pub ccr: u128,
    pub debt_gas_compensation: u128,
    pub percent_divisor: u128,
}

impl IBabelBase for BabelBaseImpl {
    fn decimal_precision(&self) -> u128 {
        self.decimal_precision
    }

    fn ccr(&self) -> u128 {
        self.ccr
    }

    fn debt_gas_compensation(&self) -> u128 {
        self.debt_gas_compensation
    }

    fn percent_divisor(&self) -> u128 {
        self.percent_divisor
    }
}
