//! Implements the `ResidualTree` trait for generic residual trees.

use crate::{
    ChargeLike, CountLike,
    molecular_formula::{GenericResidualTree, trees::ResidualTree},
};

impl<S: ChargeLike + TryFrom<U>, U: CountLike> ResidualTree for GenericResidualTree<S, U> {
    fn contains_residuals(&self) -> bool {
        match self {
            GenericResidualTree::Tree(tree) => tree.contains_residuals(),
            GenericResidualTree::Residual => true,
        }
    }
}
