//! Submodule defining a parsable entity.

mod from_str_impls;
mod molecule_parser;
mod parsable_formula;
mod parsable_molecular_tree;
mod tokens;

use core::fmt::Debug;

pub(crate) use parsable_formula::ParsableFormula;
pub use tokens::*;

use crate::parsable::{
    molecule_parser::MoleculeParser, parsable_molecular_tree::ParsableMolecularTree,
};

/// Trait for tokens used in parsing molecular formulas.
pub(crate) trait TokenLike: Copy + Eq + Sized + Debug {
    /// The count type used by this token.
    type Count: CountLike;

    /// Returns whether this token is a mixture separator.
    fn is_mixture_separator(&self) -> bool;

    /// Returns the mixture separator token.
    fn mixture_separator() -> Self;

    /// Returns the count associated with this token if it represents a count.
    fn as_count(&self) -> Option<Self::Count>;

    /// Returns the element associated with this token if it represents an
    /// element.
    fn as_element(&self) -> Option<elements_rs::Element>;
}
