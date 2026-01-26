//! Tests for molecular formulas with common complex groups like Me, Et, Ph, Bn,
//! Cy, Cp, etc.
use std::str::FromStr;

use molecular_formulas::prelude::*;

#[test]
/// Test expansion of Me2O to C2H6O
fn test_me2_expansion() {
    // Me2O -> (CH3)2O -> C2H6O
    let formula: ChemicalFormula = ChemicalFormula::from_str("Me2O").unwrap();

    // Check element counts
    let c_count = formula.count_of_element(elements_rs::Element::C);
    let h_count = formula.count_of_element(elements_rs::Element::H);
    let o_count = formula.count_of_element(elements_rs::Element::O);

    assert_eq!(c_count, Some(2), "Carbon count should be 2 for {formula}");
    assert_eq!(h_count, Some(6), "Hydrogen count should be 6 for {formula}");
    assert_eq!(o_count, Some(1), "Oxygen count should be 1 for {formula}");
}

#[test]
/// Test expansion of Et2O to C4H10O
fn test_ethyl_compound() {
    // Et2O -> (C2H5)2O -> C4H10O
    let formula: ChemicalFormula = ChemicalFormula::from_str("Et2O").unwrap();
    assert_eq!(formula.count_of_element(elements_rs::Element::C), Some(4));
    assert_eq!(formula.count_of_element(elements_rs::Element::H), Some(10));
    assert_eq!(formula.count_of_element(elements_rs::Element::O), Some(1));
}

#[test]
/// Test expansion of phenyl compounds like PhOH and PhCOOH
fn test_phenyl_compounds() {
    // PhOH -> C6H5OH -> C6H6O
    let phenol: ChemicalFormula = ChemicalFormula::from_str("PhOH").unwrap();
    assert_eq!(phenol.count_of_element(elements_rs::Element::C), Some(6));
    assert_eq!(phenol.count_of_element(elements_rs::Element::H), Some(6));
    assert_eq!(phenol.count_of_element(elements_rs::Element::O), Some(1));

    // PhCOOH -> C6H5COOH -> C7H6O2
    let benzoic_acid: ChemicalFormula = ChemicalFormula::from_str("PhCOOH").unwrap();
    assert_eq!(benzoic_acid.count_of_element(elements_rs::Element::C), Some(7));
    assert_eq!(benzoic_acid.count_of_element(elements_rs::Element::H), Some(6));
    assert_eq!(benzoic_acid.count_of_element(elements_rs::Element::O), Some(2));
}

#[test]
/// Test expansion of benzyl compounds like BnBr
fn test_benzyl_compounds() {
    // BnBr -> C7H7Br
    let benzyl_bromide: ChemicalFormula = ChemicalFormula::from_str("BnBr").unwrap();
    assert_eq!(benzyl_bromide.count_of_element(elements_rs::Element::C), Some(7));
    assert_eq!(benzyl_bromide.count_of_element(elements_rs::Element::H), Some(7));
    assert_eq!(benzyl_bromide.count_of_element(elements_rs::Element::Br), Some(1));
}

#[test]
/// Test expansion of cyclohexyl compounds like CyOH
fn test_cyclohexyl() {
    // CyOH -> C6H11OH -> C6H12O
    let cyclohexanol: ChemicalFormula = ChemicalFormula::from_str("CyOH").unwrap();
    assert_eq!(cyclohexanol.count_of_element(elements_rs::Element::C), Some(6));
    assert_eq!(cyclohexanol.count_of_element(elements_rs::Element::H), Some(12));
    assert_eq!(cyclohexanol.count_of_element(elements_rs::Element::O), Some(1));
}

#[test]
/// Test expansion of cyclopentadienyl compounds like Cp2Fe
fn test_cyclopentadienyl() {
    // Cp2Fe -> (C5H5)2Fe -> C10H10Fe (Ferrocene)
    let ferrocene: ChemicalFormula = ChemicalFormula::from_str("Cp2Fe").unwrap();
    assert_eq!(ferrocene.count_of_element(elements_rs::Element::C), Some(10));
    assert_eq!(ferrocene.count_of_element(elements_rs::Element::H), Some(10));
    assert_eq!(ferrocene.count_of_element(elements_rs::Element::Fe), Some(1));

    // Check charge: 2 * Cp(-1) + Fe(0) = -2
    assert!((ferrocene.charge() - -2.0).abs() < f64::EPSILON);
}
