extern crate cash_float;

mod support;

use std::str::FromStr;

use support::{random_float_string, add, sub, div, mul};

const ACCURACY: usize = 15;

#[test]
fn take_my_floats() {
    let n = 1000;
    let m = 100;

    for _i in 0..n {
        let digits_a = random_float_string(ACCURACY);
        let a: f64 = f64::from_str(&digits_a[..]).unwrap();
        let cfa = cash_float::new(a);

        let digits_b = random_float_string(ACCURACY);
        let b: f64 = f64::from_str(&digits_b[..]).unwrap();
        let cfb = cash_float::new(b);

        let (cfa, cfb) = add(cfa, cfb, m);
        assert_eq!(f64::from_str(&digits_a).unwrap(), cash_float::to_f64(cfa.clone()));
        
        let (cfa, cfb) = sub(cfa, cfb, m);
        assert_eq!(f64::from_str(&digits_a).unwrap(), cash_float::to_f64(cfa.clone()));
        
        let (cfa, cfb) = mul(cfa, cfb, m);
        assert_eq!(f64::from_str(&digits_a).unwrap(), cash_float::to_f64(cfa.clone()));
        
        let (cfa, _) = div(cfa, cfb, m);
        assert_eq!(f64::from_str(&digits_a).unwrap(), cash_float::to_f64(cfa.clone()));
    }
}

#[test]
fn how_accurate() {
    let n = 1000;

    for _i in 0..n {
        let digits = random_float_string(15);

        let a: f64 = f64::from_str(&digits[..]).unwrap();
        let cf = cash_float::new(a);
        let cf = cash_float::new(cash_float::to_f64(cf));
        let cf = cash_float::new(cash_float::to_f64(cf));
        let cf = cash_float::new(cash_float::to_f64(cf));
        let cf = cash_float::new(cash_float::to_f64(cf));
        let cf = cash_float::new(cash_float::to_f64(cf));
        assert_eq!(digits, format!("{}", cf));
        assert_eq!(f64::from_str(&digits).unwrap(), cash_float::to_f64(cf));

    }
}

#[test]
fn boundaries() {
    let a: f64 = 999_999_999_999_999.0;
    let a_d: String = "999999999999999".to_string();
    let b: f64 = -999_999_999_999_999.0;
    let b_d: String = "-999999999999999".to_string();
    let c: f64 = 0.99_999_999_999_999;
    let c_d: String = "0.99999999999999".to_string();
    let d: f64 = -0.99_999_999_999_999;
    let d_d: String = "-0.99999999999999".to_string();
    let cfa = cash_float::new(a);
    assert_eq!(a_d, format!("{}", cfa));
    let cfb = cash_float::new(b);
    assert_eq!(b_d, format!("{}", cfb));
    let cfc = cash_float::new(c);
    assert_eq!(c_d, format!("{}", cfc));
    let cfd = cash_float::new(d);
    assert_eq!(d_d, format!("{}", cfd));
}

#[test]
#[should_panic(expected = "float is out of range for safe cash")]
fn past_boundary_1() {
    let a: f64 = 9_999_999_999_999_999.0;
    cash_float::new(a);
}

#[test]
#[should_panic(expected = "float is out of range for safe cash")]
fn past_boundary_2() {
    let a: f64 = -9_999_999_999_999_999.0;
    cash_float::new(a);
}
