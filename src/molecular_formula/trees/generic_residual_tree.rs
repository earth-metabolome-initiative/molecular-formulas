//! Submodule defining a generic tree which can contain residuals
//! in molecular formulas.

use crate::{ChargeLike, CountLike, ParseError, Residual, molecular_formula::GenericTree};

mod display_impl;
mod instantiable_tree_impl;
mod residual_tree_impl;
mod tree_impl;

/// Generic tree implementation which can contain residuals.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GenericResidualTree<S: ChargeLike, U: CountLike> {
    /// The tree contains the syntax tree other than residuals.
    Tree(GenericTree<S, U, Box<Self>>),
    /// The residual marker variant.
    Residual,
}

impl<S: ChargeLike, U: CountLike> TryFrom<Residual> for GenericResidualTree<S, U> {
    type Error = ParseError<S, U>;

    #[inline]
    fn try_from(_: Residual) -> Result<Self, ParseError<S, U>> {
        Ok(GenericResidualTree::Residual)
    }
}

impl<S: ChargeLike + TryFrom<U>, U: CountLike> From<GenericResidualTree<S, U>>
    for GenericTree<S, U, Box<GenericResidualTree<S, U>>>
{
    #[inline]
    fn from(value: GenericResidualTree<S, U>) -> Self {
        GenericTree::Extension(Box::new(value))
    }
}
