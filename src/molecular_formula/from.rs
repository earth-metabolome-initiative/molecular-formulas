//! Submodule implementing several `From` traits for the `MolecularFormula`
//! struct

use elements_rs::{Element, Isotope};
use num_traits::ConstOne;

use super::{MolecularFormula, Tree};
use crate::molecular_formula::trees::InstantiableTree;

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

#[cfg(test)]
mod tests {
    use elements_rs::{Element, Isotope, isotopes::HydrogenIsotope};
    use num_traits::ConstOne;

    use crate::{MolecularFormula, Tree};

    #[test]
    fn test_from_element() {
        let formula: MolecularFormula = Element::H.into();
        assert_eq!(formula.mixtures.len(), 1);
        assert_eq!(formula.mixtures[0].0, <crate::DefaultTree as Tree>::Unsigned::ONE);
        assert_eq!(formula.to_string(), "H");
    }

    #[test]
    fn test_from_isotope() {
        let isotope: Isotope = HydrogenIsotope::D.into();
        let formula: MolecularFormula = isotope.into();
        assert_eq!(formula.mixtures.len(), 1);
        assert_eq!(formula.mixtures[0].0, <crate::DefaultTree as Tree>::Unsigned::ONE);
        assert!(formula.to_string().contains('D'));
    }
}
