//! Implementation of the `InstantiableTree` trait for `GenericResidualTree`.

use crate::{
    ChargeLike, Complex, CountLike, ParseError,
    molecular_formula::{GenericResidualTree, GenericTree, trees::InstantiableTree},
};

impl<S: ChargeLike + TryFrom<U>, U: CountLike> InstantiableTree for GenericResidualTree<S, U> {
    fn is_leaf(&self) -> bool {
        match self {
            Self::Tree(tree) => tree.is_leaf(),
            Self::Residual => false,
        }
    }

    fn charge(
        self,
        charge: Self::Signed,
    ) -> Result<Self, ParseError<Self::Signed, Self::Unsigned>> {
        Ok(Self::Tree(match self {
            Self::Residual => GenericTree::Extension(Box::new(Self::Residual)).charge(charge)?,
            Self::Tree(tree) => tree.charge(charge)?,
        }))
    }

    fn uncharge(self) -> Result<(Self, Self::Signed), Self> {
        match self {
            Self::Tree(tree) => {
                let (uncharged_tree, charge) = tree.uncharge().map_err(|t| Self::Tree(t))?;
                Ok(match uncharged_tree {
                    GenericTree::Extension(e) => (*e, charge),
                    _ => (Self::Tree(uncharged_tree), charge),
                })
            }
            Self::Residual => Err(self),
        }
    }

    fn round(self) -> Self {
        Self::Tree(match self {
            Self::Residual => GenericTree::Extension(Box::new(Self::Residual)).round(),
            Self::Tree(tree) => tree.round(),
        })
    }

    fn residual() -> Result<Self, ParseError<Self::Signed, Self::Unsigned>> {
        Ok(Self::Residual)
    }

    fn square(self) -> Self {
        Self::Tree(match self {
            Self::Residual => GenericTree::Extension(Box::new(Self::Residual)).square(),
            Self::Tree(tree) => tree.square(),
        })
    }

    fn repeat(
        self,
        times: Self::Unsigned,
    ) -> Result<Self, ParseError<Self::Signed, Self::Unsigned>> {
        Ok(Self::Tree(match self {
            Self::Residual => GenericTree::Extension(Box::new(Self::Residual)).repeat(times)?,
            Self::Tree(tree) => tree.repeat(times)?,
        }))
    }

    fn left_radical(self) -> Self {
        Self::Tree(GenericTree::left_radical(self.into()))
    }

    fn right_radical(self) -> Self {
        Self::Tree(GenericTree::right_radical(self.into()))
    }

    fn isotope(isotope: elements_rs::Isotope) -> Self {
        Self::Tree(GenericTree::isotope(isotope))
    }

    fn complex(complex: Complex) -> Self {
        Self::Tree(GenericTree::complex(complex))
    }

    fn element(element: elements_rs::Element) -> Self {
        Self::Tree(GenericTree::element(element))
    }

    fn from_iter<I: IntoIterator<Item = Self>>(
        iter: I,
    ) -> Result<Self, ParseError<Self::Signed, Self::Unsigned>> {
        // If all entries in the iterator are `Tree`, we can convert them
        // into an iterator of `GenericTree` directly and provide that
        // to the `GenericTree::from_iter` method.
        let sequence = iter.into_iter().collect::<Vec<Self>>();
        Ok(Self::Tree(if sequence.iter().all(|tree| matches!(tree, Self::Tree(_))) {
            GenericTree::from_iter(
                sequence
                    .into_iter()
                    .map(|tree| if let Self::Tree(t) = tree { t } else { unreachable!() }),
            )?
        } else {
            GenericTree::from_iter(sequence.into_iter().map(Into::into))?
        }))
    }

    fn into_sequence(self) -> Result<Vec<Self>, Self> {
        match self {
            Self::Tree(tree) => {
                Ok(tree
                    .into_sequence()
                    .map_err(Self::Tree)?
                    .into_iter()
                    .map(|t| if let GenericTree::Extension(e) = t { *e } else { Self::Tree(t) })
                    .collect())
            }
            Self::Residual => Err(Self::Residual),
        }
    }
}
