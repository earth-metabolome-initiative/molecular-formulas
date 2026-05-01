//! Tests for element vector encodings.

use core::str::FromStr;

use elements_rs::Element;
use molecular_formulas::{
    errors::{CountError, NumericError},
    prelude::*,
};

fn element_index(element: Element) -> usize {
    usize::from(element) - 1
}

#[test]
fn test_element_one_hot_vector() {
    let formula: ChemicalFormula = ChemicalFormula::from_str("2H2O.NaCl").unwrap();
    let vector = formula.element_one_hot_vector();

    assert_eq!(vector.len(), usize::from(Element::Og));
    assert!(vector[element_index(Element::H)]);
    assert!(vector[element_index(Element::O)]);
    assert!(vector[element_index(Element::Na)]);
    assert!(vector[element_index(Element::Cl)]);
    assert!(!vector[element_index(Element::C)]);
}

#[test]
fn test_element_count_vector() {
    let formula: ChemicalFormula = ChemicalFormula::from_str("2H2O.NaCl").unwrap();
    let vector = formula.element_count_vector::<u32>().unwrap();

    assert_eq!(vector.len(), usize::from(Element::Og));
    assert_eq!(vector[element_index(Element::H)], 4);
    assert_eq!(vector[element_index(Element::O)], 2);
    assert_eq!(vector[element_index(Element::Na)], 1);
    assert_eq!(vector[element_index(Element::Cl)], 1);
    assert_eq!(vector[element_index(Element::C)], 0);
}

#[test]
fn test_element_vectors_normalize_isotopes() {
    let formula: ChemicalFormula = ChemicalFormula::from_str("[13C]H4").unwrap();
    let one_hot = formula.element_one_hot_vector();
    let counts = formula.element_count_vector::<u32>().unwrap();

    assert!(one_hot[element_index(Element::C)]);
    assert_eq!(counts[element_index(Element::C)], 1);
    assert_eq!(counts[element_index(Element::H)], 4);
}

#[test]
fn test_element_count_vector_reports_overflow() {
    let formula: ChemicalFormula = ChemicalFormula::from_str("300H300").unwrap();

    assert_eq!(
        formula.element_count_vector::<u16>(),
        Err(CountError::Numeric(NumericError::PositiveOverflow))
    );
}
