extern crate cash_float;
extern crate rand;

use self::rand::prelude::*;
use self::cash_float::BigDecimal;


pub fn random_float_string(length: usize) -> String {
    let mut rng = thread_rng();
    // Don't want 0 as first digit
    let first_d: i32 = rng.gen_range(1, 10);
    let mut digits = format!("{}", first_d);

    // Create random digits
    for _i in 0..(length - 1) {
        let d: i32 = rng.gen_range(0, 10);
        digits.push_str(&d.to_string());
    }

    // Add decimal point in random position
    let point_i = rng.gen_range(1, digits.len() - 1);
    digits.insert(point_i, '.');

    if rng.gen() {
        digits = format!("-{}", digits);
    }
    digits
}

pub fn add(mut a: BigDecimal, b: BigDecimal, m: usize) -> (BigDecimal, BigDecimal) {
    for _i in 0..m {
        a += &b;
        a -= &b;
    }
    (a, b)
}

pub fn sub(mut a: BigDecimal, b: BigDecimal, m: usize) -> (BigDecimal, BigDecimal) {
    for _i in 0..m {
        a -= &b;
        a += &b;
    }
    (a, b)
}

pub fn mul(mut a: BigDecimal, b: BigDecimal, m: usize) -> (BigDecimal, BigDecimal) {
    for _i in 0..m {
        a *= &b;
        a = &a / &b;
    }
    (a, b)
}

pub fn div(mut a: BigDecimal, b: BigDecimal, m: usize) -> (BigDecimal, BigDecimal) {
    for _i in 0..m {
        a = &a / &b;
        a *= &b;
    }
    (a, b)
}
