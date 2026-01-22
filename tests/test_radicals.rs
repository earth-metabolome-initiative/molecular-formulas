//! Submodule testing the correct parsing of radicals in molecular formulas.

use molecular_formulas::{
    AllowedCharacter, DefaultTree, MolecularFormula, ParseError, SubTokenError, TokenError,
};

#[test]
/// Test to validate that the appropriate error is raised for invalid radicals.
fn test_invalid_radicals() {
    assert_eq!(
        MolecularFormula::<DefaultTree>::try_from("·"),
        Err(ParseError::EmptySequenceNotSupportedInCurrentTree)
    );

    assert_eq!(
        MolecularFormula::<DefaultTree>::try_from("·+"),
        Err(ParseError::EmptySequenceNotSupportedInCurrentTree)
    );

    assert_eq!(
        MolecularFormula::<DefaultTree>::try_from("-·"),
        Err(ParseError::EmptySequenceNotSupportedInCurrentTree)
    );

    assert_eq!(
        MolecularFormula::<DefaultTree>::try_from("H2O··"),
        Err(ParseError::Token(TokenError::SubToken(SubTokenError::InvalidRepeatedCharacter(
            AllowedCharacter::Radical
        )))),
    );
}

#[test]
fn test_clorine_radical() {
    let formula: MolecularFormula = MolecularFormula::<DefaultTree>::try_from("Cl·").unwrap();
    assert_eq!(formula.to_string(), "Cl•");
    let formula: MolecularFormula = MolecularFormula::<DefaultTree>::try_from("•Cl").unwrap();
    assert_eq!(formula.to_string(), "•Cl");
}
