pub use bigdecimal::{BigDecimal, ToPrimitive};
use std::str::FromStr;

const ACCURACY: usize = 15;

/// Creates a BigDecimal safley
/// Must be between:
/// 999_999_999_999_999.0
/// -999_999_999_999_999.0
/// Only safe for 15 digits total
/// Anything longer then
/// 0.99_999_999_999_999
/// or
/// -0.99_999_999_999_999
/// will be lost
pub fn new(x: f64) -> BigDecimal {
    out_of_bounds(x);
    let d = x as i64;
    let neg = if d < 0 { 1 } else { 0 };
    let len = d.to_string().len() - neg;
    let x_string = format!("{:.*}", ACCURACY - len, x);
    BigDecimal::from_str(&x_string).unwrap()
}

fn out_of_bounds(x: f64) {
    let a: f64 = 999_999_999_999_999.0;
    let b: f64 = -999_999_999_999_999.0;
    if x > a || x < b { panic!("float is out of range for safe cash"); }
}

pub fn to_f64(bd: BigDecimal) -> f64 {
    f64::from_str(&format!("{}", bd)).unwrap()
}
