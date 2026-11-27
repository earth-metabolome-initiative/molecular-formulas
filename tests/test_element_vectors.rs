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

#[test]
fn labeled_groups_in_a_charged_mixture_share_element_totals() {
    let formula: ChemicalFormula = "2([13C]H3)2O.D2O.Na+".parse().unwrap();
    let mut expected = vec![0u32; 118];
    for (element, count) in [(Element::C, 4), (Element::H, 14), (Element::O, 3), (Element::Na, 1)] {
        expected[element_index(element)] = count;
    }
    assert_eq!(formula.element_count_vector(), Ok(expected));
}

#[test]
fn count_vector_uses_the_requested_width_for_products_and_sums() {
    for input in ["2(H65535)65535", "65535H65535.65535H65535"] {
        let formula: ChemicalFormula = input.parse().unwrap();
        assert_eq!(
            formula.element_count_vector::<u32>(),
            Err(CountError::Numeric(NumericError::PositiveOverflow))
        );
        let counts = formula.element_count_vector::<u64>().unwrap();
        assert_eq!(counts[element_index(Element::H)], 8_589_672_450);
    }
}

#[test]
fn superoxide_counts_both_oxygen_atoms() {
    let formula: ChemicalFormula = "O2•−".parse().unwrap();
    let mut expected = vec![0u32; 118];
    expected[element_index(Element::O)] = 2;
    assert_eq!(formula.element_count_vector(), Ok(expected));
}

#[test]
fn inchi_component_coefficients_contribute_to_element_totals() {
    let formula: InChIFormula = "2H2O.ClNa".parse().unwrap();
    let mut expected = vec![0u32; 118];
    for (element, count) in [(Element::H, 4), (Element::O, 2), (Element::Cl, 1), (Element::Na, 1)] {
        expected[element_index(element)] = count;
    }
    assert_eq!(formula.element_count_vector(), Ok(expected));
}
