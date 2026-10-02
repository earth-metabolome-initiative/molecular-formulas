//! Submodule providing traits for parsable molecular trees.

use core::iter::Peekable;

use elements_rs::Element;

use crate::{TokenLike, errors::ParserError};

/// Trait for parsable molecular trees.
pub(crate) trait ParsableMolecularTree<Count> {
    /// The type of token used to parse the molecular formula.
    type Token: TokenLike<Count = Count>;
    /// The iterator which converts a stream of characters into the tokens used
    /// to parse the molecular formula.
    type Tokens<I>: Iterator<Item = Result<Self::Token, ParserError>> + From<Peekable<I>>
    where
        I: Iterator<Item = char>;

    /// Returns an empty molecular tree.
    fn empty() -> Self;

    /// Returns whether the molecular tree is empty.
    fn is_empty(&self) -> bool;

    /// Adds a new element to the molecular tree.
    fn element(self, element: Element) -> Self;
}
