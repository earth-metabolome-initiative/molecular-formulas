//! Submodule implementing `ResidualTree` for `GenericTree` parametrized with
//! a residual extension.

use crate::{
    ChargeLike, CountLike,
    molecular_formula::{GenericTree, trees::ResidualTree},
};

impl<S: ChargeLike + TryFrom<U>, U: CountLike, T: ResidualTree<Signed = S, Unsigned = U>>
    ResidualTree for GenericTree<S, U, T>
{
    fn contains_residuals(&self) -> bool {
        match self {
            Self::Extension(ext) => ext.contains_residuals(),
            Self::Charge(inner, _)
            | Self::Unit(inner, _)
            | Self::Repeat(inner, _)
            | Self::Radical(inner, _) => inner.contains_residuals(),
            Self::Sequence(formulas) => formulas.iter().any(ResidualTree::contains_residuals),
            Self::Element(_) | Self::Isotope(_) => false,
        }
    }
}
