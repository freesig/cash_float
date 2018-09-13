#![feature(test)]

extern crate cash_float;
extern crate test;

use test::Bencher;

mod support;

const ACCURACY: usize = 15;

#[bench]
fn bench_math(b: &mut Bencher) {
    let n = 1000;
    let mut digits = Vec::<f64>::with_capacity(n);
    for _i in 0..n {
        digits.push(support::random_float());
    }

    b.iter(|| {
        for f in &digits {
            let cf = cash_float::new(*f);
            test::black_box(cf);
        }
    });
}
