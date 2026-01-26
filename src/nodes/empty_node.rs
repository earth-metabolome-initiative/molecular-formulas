//! Marker type for an empty node and its trait implementations.

use core::fmt::Display;

use crate::{ChargeLike, ChargedMolecularTree, CountLike, MolecularTree};

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Marker type for an empty node.
pub struct Empty;

impl Display for Empty {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "")
    }
}

#[cfg(feature = "fuzzing")]
impl<'a> arbitrary::Arbitrary<'a> for Empty {
    fn arbitrary(_: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Empty)
    }
}

impl TryFrom<char> for Empty {
    type Error = ();

    fn try_from(_value: char) -> Result<Self, Self::Error> {
        Err(())
    }
}
impl From<Empty> for char {
    fn from(_: Empty) -> Self {
        '\0'
    }
}

impl<Count: CountLike> MolecularTree<Count> for Empty {
    type ElementIter<'a>
        = core::iter::Empty<elements_rs::Element>
    where
        Self: 'a;

    #[inline]
    fn elements(&self) -> Self::ElementIter<'_> {
        core::iter::empty()
    }

    #[inline]
    fn contains_elements(&self) -> bool {
        false
    }

    #[inline]
    fn contains_isotopes(&self) -> bool {
        false
    }

    #[inline]
    fn contains_element(&self, _element: elements_rs::Element) -> bool {
        false
    }

    #[inline]
    fn contains_isotope(&self, _isotope: elements_rs::Isotope) -> bool {
        false
    }

    #[inline]
    fn count_of_element<C>(&self, _element: elements_rs::Element) -> Option<C>
    where
        C: From<Count>
            + num_traits::CheckedAdd
            + num_traits::CheckedMul
            + num_traits::ConstZero
            + num_traits::ConstOne,
    {
        Some(C::ZERO)
    }

    #[inline]
    fn count_of_isotope<C>(&self, _isotope: elements_rs::Isotope) -> Option<C>
    where
        C: From<Count>
            + num_traits::CheckedAdd
            + num_traits::CheckedMul
            + num_traits::ConstZero
            + num_traits::ConstOne,
    {
        Some(C::ZERO)
    }

    fn isotopologue_mass(&self) -> f64 {
        0.0
    }

    #[inline]
    fn is_noble_gas_compound(&self) -> bool {
        false
    }
}

impl<Count: CountLike, Charge: ChargeLike> ChargedMolecularTree<Count, Charge> for Empty {
    fn charge(&self) -> f64 {
        0.0
    }

    fn isotopologue_mass_with_charge(&self) -> f64 {
        0.0
    }

    fn molar_mass(&self) -> f64 {
        0.0
    }
}
