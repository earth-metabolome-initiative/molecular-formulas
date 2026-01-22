//! Submodule testing whether formulas including mineral variants are parsed
//! correctly.
//!
//! The formulas are primarily characterized by having a greek letter in the
//! formula, which does not have any specific chemical meaning, but simply
//! distinguishes the different variants of the same mineral.

use molecular_formulas::{
    AllowedCharacterError, DefaultTree, GreekLetter, MolecularFormula, ParseError, SubTokenError,
    TokenError,
};

#[test]
/// Test checking that a formula consisting solely of one or more greek letters
/// with and without a minus sign raises the appropriate error.
fn test_only_greek_letter() {
    const IMPROPER_FORMULAS: &[(&str, GreekLetter)] = &[
        ("\u{03b1}", GreekLetter::Alpha),
        ("β", GreekLetter::Beta),
        ("βδ", GreekLetter::Beta),
        ("H2Oβ", GreekLetter::Beta),
        ("H2Oβδ", GreekLetter::Beta),
        ("βH2O", GreekLetter::Beta),
        ("βH2Oδ", GreekLetter::Beta),
        ("H2Oβ-", GreekLetter::Beta),
        ("β-H2O\u{03b1}", GreekLetter::Alpha),
        ("δ-δ", GreekLetter::Delta),
    ];

    for (formula, greek_letter) in IMPROPER_FORMULAS {
        let result: ParseError<i16, u16> =
            MolecularFormula::<DefaultTree>::try_from(*formula).unwrap_err();

        if result != ParseError::UnexpectedGreekLetter(*greek_letter)
            && result
                != ParseError::Token(TokenError::SubToken(SubTokenError::AllowedCharacter(
                    AllowedCharacterError::GreekLetterMustBeFollowedByHyphen(*greek_letter),
                )))
        {
            panic!("Expected error for formula `{formula}` with greek letter `{greek_letter}`");
        }
    }
}
