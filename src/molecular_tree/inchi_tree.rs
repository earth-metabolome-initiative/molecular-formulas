//! General enumeration for chemical tree nodes.

use core::fmt::Display;

use crate::{
    CountLike, MolecularTree,
    prelude::{Element, RepeatNode},
};

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A tree node representing molecular formulas in InChI format.
pub struct InChITree<Count> {
    node: RepeatNode<Count, Element>,
}

impl<Count: CountLike> MolecularTree<Count> for InChITree<Count> {
    type ElementIter<'a>
        = <RepeatNode<Count, Element> as MolecularTree<Count>>::ElementIter<'a>
    where
        Self: 'a;

    #[inline]
    fn elements(&self) -> Self::ElementIter<'_> {
        self.node.elements()
    }

    #[inline]
    fn contains_elements(&self) -> bool {
        self.node.contains_elements()
    }

    #[inline]
    fn contains_isotopes(&self) -> bool {
        self.node.contains_isotopes()
    }

    #[inline]
    fn contains_element(&self, element: Element) -> bool {
        self.node.contains_element(element)
    }

    #[inline]
    fn contains_isotope(&self, isotope: elements_rs::Isotope) -> bool {
        self.node.contains_isotope(isotope)
    }

    #[inline]
    fn count_of_element<C>(&self, element: Element) -> Option<C>
    where
        C: From<Count>
            + num_traits::CheckedAdd
            + num_traits::CheckedMul
            + num_traits::ConstZero
            + num_traits::ConstOne,
    {
        self.node.count_of_element::<C>(element)
    }

    #[inline]
    fn count_of_isotope<C>(&self, isotope: elements_rs::Isotope) -> Option<C>
    where
        C: From<Count>
            + num_traits::CheckedAdd
            + num_traits::CheckedMul
            + num_traits::ConstZero
            + num_traits::ConstOne,
    {
        self.node.count_of_isotope::<C>(isotope)
    }

    fn isotopologue_mass(&self) -> f64 {
        self.node.isotopologue_mass()
    }

    fn is_noble_gas_compound(&self) -> bool {
        self.node.is_noble_gas_compound()
    }
}

impl<Count: CountLike> From<Element> for InChITree<Count> {
    fn from(element: Element) -> Self {
        InChITree { node: RepeatNode::new(Count::ONE, element) }
    }
}

impl<Count> From<RepeatNode<Count, Element>> for InChITree<Count> {
    fn from(node: RepeatNode<Count, Element>) -> Self {
        InChITree { node }
    }
}

impl<Count: CountLike> Display for InChITree<Count> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}{}", self.node.node(), self.node.count())
    }
}
