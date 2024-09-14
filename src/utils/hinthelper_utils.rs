pub fn compute_nominal_cr(coll: u128, debt: u128) -> u128 {
    coll / debt
}

pub fn compute_cr(coll: u128, debt: u128, price: u128) -> u128 {
    (coll * price) / debt
}
