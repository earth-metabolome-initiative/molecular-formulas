//! Benchmarks for contains and contains_count methods.

use std::str::FromStr;

use criterion::{Criterion, criterion_group, criterion_main};
use molecular_formulas::prelude::*;

fn criterion_benchmark(c: &mut Criterion) {
    // Small formulas (2-3 elements)
    let water: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();
    let ammonia: ChemicalFormula = ChemicalFormula::from_str("NH3").unwrap();

    // Medium formulas (3-6 elements)
    let glucose: ChemicalFormula = ChemicalFormula::from_str("C6H12O6").unwrap();
    let ch2o: ChemicalFormula = ChemicalFormula::from_str("CH2O").unwrap();
    let hydrate: ChemicalFormula = ChemicalFormula::from_str("CuSO4.5H2O").unwrap();

    // Large formula (many elements)
    let complex: ChemicalFormula =
        ChemicalFormula::from_str("C₃₉₀H₄₀₄B₂Br₂ClCs₂F₁₁K₂MnN₂₆Na₂O₁₀₀OsPdS₃W₂").unwrap();

    // Isotope formulas
    let labeled: ChemicalFormula = ChemicalFormula::from_str("[13C]6H12O6").unwrap();
    let c13: ChemicalFormula = ChemicalFormula::from_str("[13C]").unwrap();

    // InChI formulas
    let ethanol: InChIFormula = InChIFormula::from_str("C2H6O").unwrap();
    let ch3: InChIFormula = InChIFormula::from_str("CH3").unwrap();

    let mut group = c.benchmark_group("contains");

    group.bench_function("small: water contains ammonia", |b| {
        b.iter(|| water.contains(&ammonia));
    });

    group.bench_function("medium: glucose contains water", |b| {
        b.iter(|| glucose.contains(&water));
    });

    group.bench_function("medium: hydrate contains water", |b| {
        b.iter(|| hydrate.contains(&water));
    });

    group.bench_function("large: complex contains ch2o", |b| {
        b.iter(|| complex.contains(&ch2o));
    });

    group.bench_function("isotope: labeled contains c13", |b| {
        b.iter(|| labeled.contains(&c13));
    });

    group.bench_function("inchi: ethanol contains ch3", |b| {
        b.iter(|| ethanol.contains(&ch3));
    });

    group.finish();

    let mut group = c.benchmark_group("contains_count");

    group.bench_function("medium: glucose contains_count water", |b| {
        b.iter(|| glucose.contains_count(&water));
    });

    group.bench_function("medium: glucose contains_count ch2o", |b| {
        b.iter(|| glucose.contains_count(&ch2o));
    });

    group.bench_function("large: complex contains_count ch2o", |b| {
        b.iter(|| complex.contains_count(&ch2o));
    });

    group.bench_function("isotope: labeled contains_count c13", |b| {
        b.iter(|| labeled.contains_count(&c13));
    });

    group.bench_function("inchi: ethanol contains_count ch3", |b| {
        b.iter(|| ethanol.contains_count(&ch3));
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
