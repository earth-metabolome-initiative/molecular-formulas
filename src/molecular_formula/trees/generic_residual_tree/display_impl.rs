//! Submodule implementing the display functionality for the
//! `GenericResidualTree` struct.

use core::fmt::{Display, Formatter, Result};

use crate::{ChargeLike, CountLike, Residual, molecular_formula::GenericResidualTree};

impl<S: ChargeLike, U: CountLike> Display for GenericResidualTree<S, U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            super::GenericResidualTree::Tree(tree) => write!(f, "{tree}"),
            super::GenericResidualTree::Residual => write!(f, "{Residual}"),
        }
    }
}
