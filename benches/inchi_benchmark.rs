//! Benchmark for InChIFormula parsing.

use core::hint::black_box;
use std::str::FromStr;

use criterion::{Criterion, criterion_group, criterion_main};
use molecular_formulas::prelude::*;

/// Benchmark parsing a large InChI formula representing a mixture.
fn criterion_benchmark(c: &mut Criterion) {
    let mixture = "C5H9NO.C5H11N.C5H10O2S.C5H10OS.C5H10O.C5H10S.C4H6N2O2.C4H8N2O.C4H10N2.C4H9NO2S.C4H7NO2.C4H9NOS.C4H9NO.C4H7NO.C4H9NS.C4H9N.3C4H7N.C4H8O3S.2C4H8O2S.2C4H6O2S.C4H8OS.2C4H6OS.C4H8O.2C4H6O.C4H8S.2C4H6S.2C3H6N2O.C3H8N2.3C3H6N2.C3H8N2.3C3H6N2.C3H7NO2S.3C3H5NO2S.C3H5NO2.C3H7NOS.4C3H5NOS.C3H7NO.3C3H5NO.C3H7NS.3C3H5NS.C3H7N.C3H6O3S.C3H4O3S.C3H6O2S.C3H4O2S.C3H6O2S.C3H6O2.C3H4O2.C3H6OS.C3H4OS.C3H6OS.C3H6O.C3H6S.C2H5N.C2H4O";
    c.bench_function("inchi large mixture", |b| {
        b.iter(|| {
            let _: InChIFormula = InChIFormula::from_str(black_box(mixture)).unwrap();
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
