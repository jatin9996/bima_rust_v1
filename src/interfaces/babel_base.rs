// Define the IBabelBase trait to 
pub trait IBabelBase {
    fn decimal_precision(&self) -> u128;

    fn ccr(&self) -> u128;

    fn debt_gas_compensation(&self) -> u128;

    fn percent_divisor(&self) -> u128;
}
