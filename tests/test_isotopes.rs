//! Tests for isotopic notation in molecular formulas.
use elements_rs::isotopes::HydrogenIsotope;
use molecular_formulas::prelude::*;

#[test]
/// Test standard isotope notation like ¹³C
fn test_standard_isotopes() {
    // ¹³C
    let formula: ChemicalFormula = "¹³CH4".parse().unwrap();
    let mass = formula.isotopologue_mass();
    // 13.00335 + 4 * 1.007825
    assert!((mass - 17.03465).abs() < 1e-3);

    // We check that the same formula can also be parsed by the 
    // ResidualFormula parser.
    let residual_formula: ResidualFormula = "¹³CH4".parse().unwrap();
    assert_eq!(formula.to_string(), residual_formula.to_string());
}

#[test]
/// Test bracket isotope notation like [13C]
fn test_bracket_isotopes() {
    // [13C]H4
    let formula: ChemicalFormula = "[13C]H4".parse().unwrap();
    let mass = formula.isotopologue_mass();
    // 13.00335 + 4 * 1.007825
    assert!((mass - 17.03465).abs() < 1e-3, "Mass was {mass} but expected ~17.03465");

    // We check that the same formula can also be parsed by the 
    // ResidualFormula parser.
    let residual_formula: ResidualFormula = "[13C]H4".parse().unwrap();
    assert_eq!(formula.to_string(), residual_formula.to_string());
}

#[test]
/// Test round bracket isotope notation like (13C)
fn test_round_bracket_isotopes() {
    // (13C)H4
    let formula: ChemicalFormula = "(13C)H4".parse().unwrap();
    let mass = formula.isotopologue_mass();
    // 13.00335 + 4 * 1.007825
    assert!((mass - 17.03465).abs() < 1e-3, "Mass was {mass} but expected ~17.03465");

    // We check that the same formula can also be parsed by the 
    // ResidualFormula parser.
    let residual_formula: ResidualFormula = "(13C)H4".parse().unwrap();
    assert_eq!(formula.to_string(), residual_formula.to_string());
}

#[test]
/// Test deuterium notation D
fn test_deuterium() {
    // D2O
    let formula: ChemicalFormula = "D2O".parse().unwrap();
    // D mass approx 2.014
    // 2 * 2.014 + 15.9949 = ~20.023
    let mass = formula.isotopologue_mass();
    assert!((mass - 20.023).abs() < 1e-3);

    assert!(formula.contains_isotope(HydrogenIsotope::D.into()));

    // We check that the same formula can also be parsed by the 
    // ResidualFormula parser.
    let residual_formula: ResidualFormula = "D2O".parse().unwrap();
    assert_eq!(formula.to_string(), residual_formula.to_string());
}

#[test]
/// Test tritium notation T
fn test_tritium() {
    // T2
    let formula: ChemicalFormula = "T2".parse().unwrap();
    // T mass approx 3.016
    let mass = formula.isotopologue_mass();
    assert!((mass - 6.032).abs() < 1e-3);

    // We check that the same formula can also be parsed by the 
    // ResidualFormula parser.
    let residual_formula: ResidualFormula = "T2".parse().unwrap();
    assert_eq!(formula.to_string(), residual_formula.to_string());
}

#[test]
/// Test other bracket isotope notation like [18O]ù
fn test_other_bracket_isotopes() {
    // [18O]
    let formula: ChemicalFormula = "H2[18O]".parse().unwrap();
    // 2*1.008 + 17.999 = ~20.015
    let mass = formula.isotopologue_mass();
    assert!((mass - 20.015).abs() < 1e-3);

    // We check that the same formula can also be parsed by the 
    // ResidualFormula parser.
    let residual_formula: ResidualFormula = "H2[18O]".parse().unwrap();
    assert_eq!(formula.to_string(), residual_formula.to_string());
}

#[test]
/// Test for the `C[13]` notation
fn test_c13_notation() {
    // C[13]H4
    let formula: ChemicalFormula = "C[13]H4".parse().unwrap();
    let mass = formula.isotopologue_mass();
    // 13.00335 + 4 * 1.007825
    assert!((mass - 17.03465).abs() < 1e-3);

    // We check that the same formula can also be parsed by the 
    // ResidualFormula parser.
    let residual_formula: ResidualFormula = "C[13]H4".parse().unwrap();
    assert_eq!(formula.to_string(), residual_formula.to_string());
}
