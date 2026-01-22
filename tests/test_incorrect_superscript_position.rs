//! Test submodule to verify that the proper error is raised when a
//! superscript is found in the wrong position.

use std::str::FromStr;

use molecular_formulas::{DefaultTree, MolecularFormula, ParseError, TokenError};

const INCORRECT_SUPERSCRIPT_POSITION: &[&str] = &["H²", "CH²"];

#[test]
/// Test that the error is raised when a superscript is found in the wrong
/// position.
fn test_incorrect_superscript_position() {
    for formula in INCORRECT_SUPERSCRIPT_POSITION {
        let error: ParseError<i16, u16> =
            MolecularFormula::<DefaultTree>::from_str(formula).unwrap_err();
        assert_eq!(error, TokenError::UnexpectedEndOfInputWhileParsingTokens.into(),);
    }
}
