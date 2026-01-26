//! Benchmark for ChemicalFormula parsing.

use core::hint::black_box;
use std::str::FromStr;

use criterion::{Criterion, criterion_group, criterion_main};
use molecular_formulas::prelude::*;

/// Benchmark parsing a complex ChemicalFormula with unicode subscripts and
/// charge.
fn criterion_benchmark(c: &mut Criterion) {
    let formula = "C₃₉₀H₄₀₄B₂Br₂ClCs₂F₁₁K₂MnN₂₆Na₂O₁₀₀OsPdS₃W₂³⁻";
    c.bench_function("chemical formula complex", |b| {
        b.iter(|| {
            let _: ChemicalFormula = ChemicalFormula::from_str(black_box(formula)).unwrap();
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
