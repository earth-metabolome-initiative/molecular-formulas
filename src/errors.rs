//! Submodule defining the error enumeration which might occur when working
//! with molecular formula.

use crate::Bracket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
/// Errors associated with numeric operations.
pub enum NumericError {
    /// Leading zero not allowed in counts.
    #[error("Leading zero not allowed in counts.")]
    LeadingZero,
    /// A positive overflow occurred during a numeric operation.
    #[error("Positive overflow occurred during numeric operation.")]
    PositiveOverflow,
    /// A negative overflow occurred during a numeric operation.
    #[error("Negative overflow occurred during numeric operation.")]
    NegativeOverflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
/// Error enumeration when parsing a molecular formula.
pub enum ParserError {
    /// Unexpected end of input reached while parsing tokens.
    #[error("Unexpected end of input while parsing tokens.")]
    UnexpectedEndOfInput,
    /// An error regarding numeric parsing or operations occurred.
    #[error("Numeric error: {0}")]
    Numeric(#[from] NumericError),
    /// A provided character is not allowed in the current molecular formula
    /// tree.
    #[error("Character '{0}' is not allowed in the current molecular formula tree.")]
    UnexpectedCharacter(char),
    /// The formula is not compliant with the Hill system ordering.
    #[error("The formula is not compliant with the Hill system ordering.")]
    NotHillOrdered,
    /// An error encountered from the elements-rs crate.
    #[error("Element error: {0}")]
    Element(#[from] elements_rs::errors::Error),
    /// A superscripted number could not be processed neither as a charge nor
    /// as an isotopic number.
    #[error("A number could not be processed neither as a charge nor as an isotopic number.")]
    UnprocessableNumber,
    /// A closing bracket was expected but not found.
    #[error("A closing bracket '{}' was expected but not found.", .0.closing())]
    MissingClosingBracket(Bracket),
    /// The molecular tree is empty.
    #[error("The molecular tree is empty.")]
    EmptyMolecularTree,
}
