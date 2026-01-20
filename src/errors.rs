//! Submodule providing the enumeration of errors which may occur while parsing
//! a molecular formula.

use std::num::TryFromIntError;

use thiserror::Error;

use crate::token::{Token, greek_letters::GreekLetter};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enumeration of errors which may occur while parsing a molecular formula.
pub enum Error {
    /// Error indicating that an unknown element was encountered.
    #[error("Element error: {0}")]
    Element(#[from] elements_rs::errors::Error),
    /// Error indicating that a character in the formula is invalid.
    #[error("Invalid character: {0}")]
    InvalidCharacter(char),
    /// Invalid repeated token in the formula.
    #[error("Invalid repeated token: {0:?}")]
    InvalidRepeatedToken(Token),
    /// Error indicating that a greek letter in the formula is at an
    /// invalid position.
    #[error("Invalid greek letter position: {0}")]
    InvalidGreekLetterPosition(GreekLetter),
    /// Error indicating that a number in the formula is invalid.
    #[error("Invalid number")]
    InvalidNumber,
    /// Error indicating that a number in the formula is invalid.
    #[error("Empty formula")]
    EmptyFormula,
    /// Error indicating that a formula is invalid.
    #[error("Invalid formula")]
    InvalidFormula,
    /// Error indicating that the expected closing token was not found.
    #[error("Expected closing token: {expected:?}, found: {found:?}")]
    ClosingToken {
        /// The expected closing token.
        expected: Option<Token>,
        /// The found closing token.
        found: Option<Token>,
    },
    /// Error raised when an uncountable term is being counted.
    #[error("Counting uncountable term")]
    CountingUncountable,
    /// When the leading token is not a number or an element.
    #[error("Invalid leading token: {0:?}")]
    InvalidLeadingToken(Token),
    /// When the parser is not completely consumed.
    #[error("Unconsumed parser")]
    UnconsumedParser,
    /// When an ion has a charge of 0.
    #[error("Ion has a charge of 0")]
    ZeroCharge,
    /// When a count has a value of 0.
    #[error("Count has a value of 0")]
    ZeroCount,
    /// When a charge is not at the end of the formula.
    #[error("Charge is not at the end of the formula")]
    InvalidChargePosition,
    /// When a superscript is at an invalid position.
    #[error("Superscript is at an invalid position")]
    InvalidSuperscriptPosition,
    /// When an operation is not defined for residuals.
    #[error("Operation is not defined for residuals")]
    InvalidOperationForResidual,
    /// When an operation is not defined for a mixture.
    #[error("Operation is not defined for mixtures")]
    InvalidOperationForMixture,
    /// When an operation is only defined for diatomic formulas.
    #[error("Operation is only defined for diatomic formulas")]
    InvalidOperationForNonDiatomic,
    /// When an oxidation state is invalid.
    #[error("Oxidation state is invalid: {0}")]
    InvalidOxidationState(i16),
    /// When a provided string is not a valid greek letter.
    #[error("Provided string is not a valid greek letter: {0}")]
    InvalidGreekLetter(String),
    /// When a provided string is not a valid complex group fragment.
    #[error("Provided string is not a valid complex group fragment: {0}")]
    InvalidComplexGroupFragment(String),
}

impl From<TryFromIntError> for Error {
    fn from(_: TryFromIntError) -> Self {
        Error::InvalidNumber
    }
}
