//! Tree trait implementation for `GenericResidualTree`.

use crate::{ChargeLike, CountLike, Tree, molecular_formula::GenericResidualTree};

impl<S: ChargeLike + TryFrom<U>, U: CountLike> Tree for GenericResidualTree<S, U> {
    type Signed = S;
    type Unsigned = U;

    fn iter_elements(&self) -> Box<dyn Iterator<Item = elements_rs::Element> + '_> {
        match self {
            GenericResidualTree::Tree(tree) => tree.iter_elements(),
            GenericResidualTree::Residual => Box::new(std::iter::empty()),
        }
    }

    fn iter_counted_elements(&self) -> Box<dyn Iterator<Item = elements_rs::Element> + '_> {
        match self {
            GenericResidualTree::Tree(tree) => tree.iter_counted_elements(),
            GenericResidualTree::Residual => Box::new(std::iter::empty()),
        }
    }

    fn iter_isotopes(&self) -> Box<dyn Iterator<Item = elements_rs::Isotope> + '_> {
        match self {
            GenericResidualTree::Tree(tree) => tree.iter_isotopes(),
            GenericResidualTree::Residual => Box::new(std::iter::empty()),
        }
    }

    fn element_count(&self, target: elements_rs::Element) -> u64 {
        match self {
            GenericResidualTree::Tree(tree) => tree.element_count(target),
            GenericResidualTree::Residual => 0,
        }
    }

    fn isotope_count(&self, target: elements_rs::Isotope) -> u64 {
        match self {
            GenericResidualTree::Tree(tree) => tree.isotope_count(target),
            GenericResidualTree::Residual => 0,
        }
    }
}
