//! Submodule testing the correct parsing of radicals in molecular formulas.

use molecular_formulas::{errors::ParserError, prelude::*};

#[test]
/// Test to validate that the appropriate error is raised for invalid radicals.
fn test_invalid_radicals() {
    assert_eq!(ChemicalFormula::<u16, i16>::try_from("·"), Err(ParserError::EmptyMolecularTree));

    assert_eq!(ChemicalFormula::<u16, i16>::try_from("·+"), Err(ParserError::EmptyMolecularTree));

    assert_eq!(
        ChemicalFormula::<u16, i16>::try_from("-·"),
        Err(ParserError::EmptyMolecularTree)
    );

    assert_eq!(
        ChemicalFormula::<u16, i16>::try_from("H2O··"),
        Err(ParserError::UnexpectedCharacter('·'))
    );
}

#[test]
fn test_clorine_radical() {
    let formula: ChemicalFormula = ChemicalFormula::<u16, i16>::try_from("Cl·").unwrap();
    assert_eq!(formula.to_string(), "Cl•");
    let formula: ChemicalFormula = ChemicalFormula::<u16, i16>::try_from("•Cl").unwrap();
    assert_eq!(formula.to_string(), "•Cl");
}

#[test]
fn test_radical_precedes_charge() {
    let formula = "O2•−";
    let formula = ChemicalFormula::<u16, i16>::try_from(formula).unwrap();
    assert_eq!(formula.to_string(), "O₂•⁻");
}

#[test]
fn test_radical_follows_charge() {
    let formula = "O2−•";
    let formula = ChemicalFormula::<u16, i16>::try_from(formula).unwrap();
    assert_eq!(formula.to_string(), "O₂⁻•");
}