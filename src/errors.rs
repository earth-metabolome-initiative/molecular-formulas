//! Errors which might occur when working with molecular formulas.

use crate::molecular_formula::ParseError;

#[derive(Debug, thiserror::Error)]
/// Errors which might occur when working with molecular formulas.
pub enum Error<Signed, Unsigned> {
    /// An error was encountered when parsing a molecular formula.
    #[error("An error occurred while parsing the molecular formula: {0}")]
    ParseError(#[from] ParseError<Signed, Unsigned>),
    /// The signed type used for charges was insufficiently large.
    #[error(
        "The signed type used for charges is insufficiently large to represent the charge of the molecular formula."
    )]
    InsufficientSignedTypeForCharge,
    /// The unsigned type used for counts was insufficiently large.
    #[error("The unsigned type used for counts is insufficiently large to represent a count.")]
    InsufficientUnsignedTypeForCount,
    /// Unclear what to do with the greek letter in the provided formula.
    #[error(
        "The molecular formula contains a greek letter decorator, which is not supported in this context."
    )]
    GreekLetterNotSupported,
}
