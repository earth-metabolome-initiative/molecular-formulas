//! Submodule to test that all elements are parsed correctly.

use std::str::FromStr;

use elements_rs::{Element, ElementVariant, MassNumber, isotopes::HydrogenIsotope};
use molecular_formulas::prelude::*;
use strum::IntoEnumIterator;

/// Test that all elements from the periodic table are parsed correctly.
fn test_all_elements_on<M>()
where
    M: MolecularFormula<Count = u16> + FromStr<Err: core::error::Error>,
{
    for element in Element::iter() {
        let formula_str = format!("{element}2");
        let formula = M::from_str(&formula_str).expect("Failed to parse formula");
        assert!(formula.contains_elements(), "Formula {formula_str} does not contain elements");
        assert_eq!(
            formula.count_of_element::<u32>(element),
            Some(2),
            "Formula {formula_str} has incorrect count for element {element:?}"
        );
    }
}

/// Test that all hydrogen isotopes are parsed correctly.
fn test_hydrogen_on<M>()
where
    M: MolecularFormula<Count = u16> + FromStr<Err: core::error::Error>,
{
    for isotope in HydrogenIsotope::iter() {
        let formula_str = format!("[{}{}]2", isotope.mass_number(), isotope.element());
        let formula = M::from_str(&formula_str).expect("Failed to parse formula");
        assert!(formula.contains_isotopes(), "Formula {formula_str} does not contain isotopes");
        assert!(formula.contains_elements(), "Formula {formula_str} does not contain elements");
        assert!(
            formula.contains_isotope(isotope.into()),
            "Formula {formula_str} does not contain isotope {isotope:?}"
        );
        assert!(
            formula.contains_element(isotope.element()),
            "Formula {formula_str} does not contain element"
        );
        assert_eq!(
            formula.count_of_element::<u32>(isotope.element()),
            Some(2),
            "Formula {formula_str} has incorrect count for element {isotope:?}"
        );
    }
}

#[test]
/// Test that all elements from the periodic table are parsed correctly.
fn test_all_elements() {
    test_all_elements_on::<InChIFormula>();
    test_all_elements_on::<ChemicalFormula>();
    test_all_elements_on::<MineralFormula>();
}

#[test]
/// Test that all elements from the periodic table are parsed correctly.
fn test_all_isotopes() -> Result<(), Box<dyn std::error::Error>> {
    test_hydrogen_on::<ChemicalFormula>();
    test_hydrogen_on::<MineralFormula>();
    Ok(())
}
