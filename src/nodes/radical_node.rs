//! Submodule providing a struct and implementation of the `ExtensionTree` trait
//! for molecular formulas that can contain radicals such as `•`.

use core::fmt::Display;

use crate::{Baseline, CharacterMarker, ChargedMolecularTree, MolecularTree};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Marker for a Radical group in a molecular formula.
pub struct Radical;

impl Display for Radical {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "•")
    }
}

impl CharacterMarker for Radical {
    const CANONICAL: char = '•';
    type TS = Baseline;
    fn matches(c: char) -> bool {
        matches!(c, '\u{2022}' | '\u{2981}' | '\u{2219}' | '·')
    }
}

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A radical node representing a molecular formula with a radical on either the
/// left or right side.
pub struct RadicalNode<T> {
    /// The tree node being represented as a radical.
    node: T,
    /// Whether the radical is on the left or right side.
    left_side: bool,
}

impl<T> AsRef<T> for RadicalNode<T> {
    fn as_ref(&self) -> &T {
        &self.node
    }
}

impl<T> RadicalNode<T> {
    /// Creates a new left-hand side radical node.
    pub fn left(node: T) -> Self {
        Self { node, left_side: true }
    }

    /// Creates a new right-hand side radical node.
    pub fn right(node: T) -> Self {
        Self { node, left_side: false }
    }
}

impl<Count, T: MolecularTree<Count>> MolecularTree<Count> for RadicalNode<T> {
    type ElementIter<'a>
        = T::ElementIter<'a>
    where
        Self: 'a;

    type NonHydrogenElementIter<'a>
        = T::NonHydrogenElementIter<'a>
    where
        Self: 'a;

    #[inline]
    fn elements(&self) -> Self::ElementIter<'_> {
        self.node.elements()
    }

    #[inline]
    fn non_hydrogens(&self) -> Self::NonHydrogenElementIter<'_> {
        self.node.non_hydrogens()
    }

    #[inline]
    fn contains_elements(&self) -> bool {
        self.node.contains_elements()
    }

    #[inline]
    fn contains_non_hydrogens(&self) -> bool {
        self.node.contains_non_hydrogens()
    }

    #[inline]
    fn contains_isotopes(&self) -> bool {
        self.node.contains_isotopes()
    }

    #[inline]
    fn contains_element(&self, element: elements_rs::Element) -> bool {
        self.node.contains_element(element)
    }

    #[inline]
    fn contains_isotope(&self, isotope: elements_rs::Isotope) -> bool {
        self.node.contains_isotope(isotope)
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

    #[inline]
    fn is_noble_gas_compound(&self) -> bool {
        self.node.is_noble_gas_compound()
    }

    #[inline]
    fn check_hill_ordering(
        &self,
        predecessor: Option<elements_rs::Element>,
        has_carbon: bool,
    ) -> Result<Option<elements_rs::Element>, ()> {
        self.node.check_hill_ordering(predecessor, has_carbon)
    }
}

impl<T: Display> Display for RadicalNode<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.left_side {
            write!(f, "{Radical}{}", self.node)
        } else {
            write!(f, "{}{Radical}", self.node)
        }
    }
}

impl<T: ChargedMolecularTree<Count, Charge>, Count, Charge> ChargedMolecularTree<Count, Charge>
    for RadicalNode<T>
{
    fn charge(&self) -> f64 {
        self.node.charge()
    }

    fn isotopologue_mass_with_charge(&self) -> f64 {
        self.node.isotopologue_mass_with_charge()
    }

    fn molar_mass(&self) -> f64 {
        self.node.molar_mass()
    }
}
