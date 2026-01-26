//! Submodule providing a struct and implementation of the `ExtensionTree` trait
//! for molecular formulas that can contain groups closed in brackets such as
//! `(CH3)2` or `[OH]3`. This is not valid syntax for InChI, but is used in
//! other contexts.

use core::fmt::Display;

use crate::{Bracket, ChargeLike, ChargedMolecularTree, CountLike};

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Bracket node representing a molecular formula wrapped in brackets.
pub struct BracketNode<T> {
    /// The underlying tree.
    tree: T,
    /// The type of bracket used.
    bracket: Bracket,
}

impl<T> AsRef<T> for BracketNode<T> {
    fn as_ref(&self) -> &T {
        &self.tree
    }
}

impl<T> BracketNode<T> {
    /// Creates a new `BracketNode` wrapping the given tree with round brackets
    /// `()`.
    pub fn round(tree: T) -> Self {
        Self { tree, bracket: Bracket::Round }
    }

    /// Creates a new `BracketNode` wrapping the given tree with square brackets
    /// `[]`.
    pub fn square(tree: T) -> Self {
        Self { tree, bracket: Bracket::Square }
    }
}

impl<Count, T: crate::MolecularTree<Count>> crate::MolecularTree<Count> for BracketNode<T> {
    type ElementIter<'a>
        = T::ElementIter<'a>
    where
        Self: 'a;

    #[inline]
    fn elements(&self) -> Self::ElementIter<'_> {
        self.tree.elements()
    }

    #[inline]
    fn contains_elements(&self) -> bool {
        self.tree.contains_elements()
    }

    #[inline]
    fn contains_isotopes(&self) -> bool {
        self.tree.contains_isotopes()
    }

    #[inline]
    fn contains_element(&self, element: elements_rs::Element) -> bool {
        self.tree.contains_element(element)
    }

    #[inline]
    fn contains_isotope(&self, isotope: elements_rs::Isotope) -> bool {
        self.tree.contains_isotope(isotope)
    }

    #[inline]
    fn count_of_element<C>(&self, element: elements_rs::Element) -> Option<C>
    where
        C: From<Count>
            + num_traits::CheckedAdd
            + num_traits::CheckedMul
            + num_traits::ConstZero
            + num_traits::ConstOne,
    {
        self.tree.count_of_element::<C>(element)
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
        self.tree.count_of_isotope::<C>(isotope)
    }

    fn isotopologue_mass(&self) -> f64 {
        self.tree.isotopologue_mass()
    }
    
    fn is_noble_gas_compound(&self) -> bool {
        self.tree.is_noble_gas_compound()
    }
}

impl<T: Display> Display for BracketNode<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}{}{}", self.bracket.opening(), self.tree, self.bracket.closing())
    }
}

impl<Count: CountLike, Charge: ChargeLike, T: ChargedMolecularTree<Count, Charge>>
    ChargedMolecularTree<Count, Charge> for BracketNode<T>
{
    fn charge(&self) -> f64 {
        self.tree.charge()
    }

    fn isotopologue_mass_with_charge(&self) -> f64 {
        self.tree.isotopologue_mass_with_charge()
    }

    fn molar_mass(&self) -> f64 {
        self.tree.molar_mass()
    }
}
