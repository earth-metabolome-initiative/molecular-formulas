//! Submodule to test that all elements are parsed correctly.

use std::str::FromStr;

use elements_rs::{Element, ElementVariant, MassNumber, isotopes::HydrogenIsotope};
use molecular_formulas::{DefaultTree, MolecularFormula};
use strum::IntoEnumIterator;

#[test]
/// Test that all elements from the periodic table are parsed correctly.
fn test_all_elements() -> Result<(), Box<dyn std::error::Error>> {
    for element in Element::iter() {
        let symbol = element.to_string();
        let formula_str = format!("{symbol}2");
        let formula = MolecularFormula::<DefaultTree>::from_str(&formula_str)?;
        assert!(formula.contains_elements(), "Formula {formula_str} does not contain elements");
        assert_eq!(
            formula.element_count(element),
            2,
            "Formula {formula_str} has incorrect count for element {element:?}"
        );
    }
    Ok(())
}

#[test]
/// Test that all elements from the periodic table are parsed correctly.
fn test_all_isotopes() -> Result<(), Box<dyn std::error::Error>> {
    for isotope in HydrogenIsotope::iter() {
        let formula_str = format!("[{}{}]2", isotope.mass_number(), isotope.element().to_string());
        let formula = MolecularFormula::<DefaultTree>::from_str(&formula_str)?;
        assert!(formula.contains_isotopes(), "Formula {formula_str} does not contain isotopes");
        assert!(formula.contains_elements(), "Formula {formula_str} does not contain elements");
        assert!(
            formula.contains_isotope(isotope),
            "Formula {formula_str} does not contain isotope {isotope:?}"
        );
        assert!(
            formula.contains_element(isotope.element()),
            "Formula {formula_str} does not contain element"
        );
        assert_eq!(
            formula.element_count(isotope.element()),
            2,
            "Formula {formula_str} has incorrect count for element {isotope:?}"
        );
    }
    Ok(())
}
