//! Integration tests for `MineralFormula`.

use std::str::FromStr;

use molecular_formulas::prelude::*;

#[test]
fn test_no_prefix() {
    let formula = MineralFormula::<u32, i32>::from_str("SiO2").unwrap();
    assert_eq!(formula.to_string(), "SiO₂");

    // Check inner formula
    let subformulas: Vec<_> = formula.counted_mixtures().collect();
    assert_eq!(subformulas.len(), 1);
}

#[test]
fn test_equality_with_without_prefix() {
    let with_prefix = MineralFormula::<u32, i32>::from_str("α-SiO2").unwrap();
    let without_prefix = MineralFormula::<u32, i32>::from_str("SiO2").unwrap();

    assert_ne!(with_prefix, without_prefix);
}

#[test]
fn test_equality_different_prefixes() {
    let alpha = MineralFormula::<u32, i32>::from_str("α-Fe2O3").unwrap();
    let gamma = MineralFormula::<u32, i32>::from_str("γ-Fe2O3").unwrap();

    assert_ne!(alpha, gamma);
}

#[test]
fn test_ocr_resilience_separators() {
    // BaselineMinus::matches includes: hyphen, en dash, em dash, etc.
    let cases = [
        "α-SiO2",        // Hyphen (Canonical)
        "α\u{2212}SiO2", // Minus sign
        "α\u{2013}SiO2", // En dash
        "α\u{2014}SiO2", // Em dash
    ];

    for (i, case) in cases.iter().enumerate() {
        let parsed = MineralFormula::<u32, i32>::from_str(case)
            .unwrap_or_else(|_| panic!("Should parse case {i}: {case}"));
        // Should Canonicalize on display to hyphen
        assert_eq!(parsed.to_string(), "α-SiO₂");
    }
}

#[test]
fn test_complex_structures() {
    // Kaolinite: Al2(Si2O5)(OH)4
    let kaolinite_str = "Al2(Si2O5)(OH)4";
    let kaolinite = MineralFormula::<u32, i32>::from_str(kaolinite_str).unwrap();
    assert_eq!(kaolinite.to_string(), "Al₂(Si₂O₅)(OH)₄");
}

#[test]
fn test_different_prefixes() {
    // Testing a few other prefixes just to be sure
    let cases = [("β-SiO2", "β-SiO₂"), ("γ-Al2O3", "γ-Al₂O₃")];

    for (input, expected) in cases {
        let f = MineralFormula::<u32, i32>::from_str(input).unwrap();
        assert_eq!(f.to_string(), expected);
    }
}

#[test]
fn test_into_counted_mixtures() {
    let formula = MineralFormula::<u32, i32>::from_str("α-SiO2.H2O").unwrap();

    // consume the formula
    let mixtures: Vec<_> = formula.into_counted_mixtures().collect();

    assert_eq!(mixtures.len(), 2);

    // First mixture: SiO2 (count 1)
    let (count1, tree1) = &mixtures[0];
    assert_eq!(*count1, 1);
    assert_eq!(tree1.to_string(), "SiO₂");

    // Second mixture: H2O (count 1)
    let (count2, tree2) = &mixtures[1];
    assert_eq!(*count2, 1);
    assert_eq!(tree2.to_string(), "H₂O");
}

#[test]
fn test_into_counted_mixtures_with_counts() {
    let formula = MineralFormula::<u32, i32>::from_str("2Fe2O3.3H2O").unwrap();

    let mixtures: Vec<_> = formula.into_counted_mixtures().collect();

    assert_eq!(mixtures.len(), 2);

    // 2Fe2O3
    assert_eq!(mixtures[0].0, 2);
    assert_eq!(mixtures[0].1.to_string(), "Fe₂O₃");

    // 3H2O
    assert_eq!(mixtures[1].0, 3);
    assert_eq!(mixtures[1].1.to_string(), "H₂O");
}
