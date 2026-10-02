//! Submodule testing the computation of the overall molecular charge.

use std::str::FromStr;

use molecular_formulas::prelude::*;

#[test]
fn test_charge() {
    let formula: ChemicalFormula = ChemicalFormula::from_str("C6H12O6").unwrap();
    assert!(formula.charge().abs() < f64::EPSILON);

    let formula: ChemicalFormula = ChemicalFormula::from_str("[Co(NH3)6]+3(Cl−)3").unwrap();
    assert!(formula.charge().abs() < f64::EPSILON);

    let formula: ChemicalFormula = ChemicalFormula::from_str("H3O+").unwrap();
    assert!((formula.charge() - 1.0).abs() < f64::EPSILON);

    let formula: ChemicalFormula = ChemicalFormula::from_str("NO2-").unwrap();
    assert!((formula.charge() - -1.0).abs() < f64::EPSILON);

    let formula: ChemicalFormula = ChemicalFormula::from_str("Ca²⁺").unwrap();
    assert!((formula.charge() - 2.0).abs() < f64::EPSILON);
}
