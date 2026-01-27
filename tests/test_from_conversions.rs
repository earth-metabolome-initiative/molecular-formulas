//! Tests for From conversions for various formula types.

use std::convert::TryFrom;

use elements_rs::{Element, Isotope};
use molecular_formulas::{ChemicalFormula, InChIFormula, MineralFormula, ResidualFormula};

#[test]
fn test_residual_formula_from_element() {
    let element = Element::C;
    let formula: ResidualFormula = ResidualFormula::from(element);
    assert_eq!(formula.to_string(), "C");
}

#[test]
fn test_residual_formula_from_isotope() {
    let isotope = Isotope::try_from((Element::C, 13u16)).expect("Could not create isotope");
    let formula: ResidualFormula = ResidualFormula::from(isotope);
    assert_eq!(formula.to_string(), "[¹³C]");
}

#[test]
fn test_chemical_formula_from_element() {
    let element = Element::O;
    let formula: ChemicalFormula = ChemicalFormula::from(element);
    assert_eq!(formula.to_string(), "O");
}

#[test]
fn test_chemical_formula_from_isotope() {
    let isotope = Isotope::try_from((Element::O, 18u16)).expect("Could not create isotope");
    let formula: ChemicalFormula = ChemicalFormula::from(isotope);
    // ChemicalFormula display uses [¹⁸O] style if it delegates to the tree which
    // handles isotopes
    assert_eq!(formula.to_string(), "[¹⁸O]");
}

#[test]
fn test_mineral_formula_from_element() {
    let element = Element::Si;
    let formula: MineralFormula = MineralFormula::from(element);
    assert_eq!(formula.to_string(), "Si");
}

#[test]
fn test_mineral_formula_from_isotope() {
    let isotope = Isotope::try_from((Element::Si, 29u16)).expect("Could not create isotope");
    let formula: MineralFormula = MineralFormula::from(isotope);
    assert_eq!(formula.to_string(), "[²⁹Si]");
}

#[test]
fn test_inchi_formula_from_element() {
    let element = Element::N;
    let formula: InChIFormula = InChIFormula::from(element);
    assert_eq!(formula.to_string(), "N");
}
