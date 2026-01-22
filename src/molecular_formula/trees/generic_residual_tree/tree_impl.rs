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

    fn element_count(&self, target: elements_rs::Element) -> Option<u64> {
        match self {
            GenericResidualTree::Tree(tree) => tree.element_count(target),
            GenericResidualTree::Residual => Some(0),
        }
    }

    fn isotope_count(&self, target: elements_rs::Isotope) -> Option<u64> {
        match self {
            GenericResidualTree::Tree(tree) => tree.isotope_count(target),
            GenericResidualTree::Residual => Some(0),
        }
    }

    fn number_of_atoms(&self) -> Option<u64> {
        match self {
            GenericResidualTree::Tree(tree) => tree.number_of_atoms(),
            GenericResidualTree::Residual => Some(0),
        }
    }

    fn get_counted_element(&self, index: u64) -> Option<elements_rs::Element> {
        match self {
            GenericResidualTree::Tree(tree) => tree.get_counted_element(index),
            GenericResidualTree::Residual => None,
        }
    }

    fn get_counted_element_or_size(&self, index: u64) -> Result<elements_rs::Element, u64> {
        match self {
            GenericResidualTree::Tree(tree) => tree.get_counted_element_or_size(index),
            GenericResidualTree::Residual => Err(0),
        }
    }
}
