//! Submodule implementing several `From` traits for the `MolecularFormula`
//! struct

use elements_rs::{Element, Isotope};
use num_traits::ConstOne;

use super::{MolecularFormula, Tree};
use crate::molecular_formula::{parser::GreekLetter, trees::InstantiableTree};

impl<T: Tree> From<T> for MolecularFormula<T> {
    #[inline]
    fn from(tree: T) -> Self {
        MolecularFormula { mixtures: vec![(T::Unsigned::ONE, tree)], greek: None }
    }
}

impl<T: InstantiableTree> From<Element> for MolecularFormula<T> {
    #[inline]
    fn from(element: Element) -> Self {
        let tree: T = T::element(element);
        tree.into()
    }
}

impl<T: InstantiableTree> From<Isotope> for MolecularFormula<T> {
    #[inline]
    fn from(isotope: Isotope) -> Self {
        let tree: T = T::isotope(isotope);
        tree.into()
    }
}

impl<T: Tree> From<GreekLetter> for MolecularFormula<T> {
    #[inline]
    fn from(greek_letter: GreekLetter) -> Self {
        MolecularFormula { mixtures: vec![], greek: Some(greek_letter) }
    }
}
