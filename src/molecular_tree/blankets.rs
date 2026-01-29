//! Blanket implementations for molecular trees.

use alloc::boxed::Box;

use crate::{ChargeLike, ChargedMolecularTree, CountLike, MolecularTree};

impl<T: MolecularTree<Count>, Count: CountLike> MolecularTree<Count> for Box<T> {
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
        (**self).elements()
    }

    #[inline]
    fn non_hydrogens(&self) -> Self::NonHydrogenElementIter<'_> {
        (**self).non_hydrogens()
    }

    #[inline]
    fn contains_elements(&self) -> bool {
        (**self).contains_elements()
    }

    #[inline]
    fn contains_non_hydrogens(&self) -> bool {
        (**self).contains_non_hydrogens()
    }

    #[inline]
    fn contains_isotopes(&self) -> bool {
        (**self).contains_isotopes()
    }

    #[inline]
    fn contains_element(&self, element: elements_rs::Element) -> bool {
        (**self).contains_element(element)
    }

    #[inline]
    fn contains_isotope(&self, isotope: elements_rs::Isotope) -> bool {
        (**self).contains_isotope(isotope)
    }

    #[inline]
    fn number_of_elements(&self) -> usize {
        (**self).number_of_elements()
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
        (**self).count_of_element::<C>(element)
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
        (**self).count_of_isotope::<C>(isotope)
    }

    fn isotopologue_mass(&self) -> f64 {
        (**self).isotopologue_mass()
    }

    fn is_noble_gas_compound(&self) -> bool {
        (**self).is_noble_gas_compound()
    }

    fn isotopic_normalization(&self) -> Self {
        Box::new((**self).isotopic_normalization())
    }

    fn check_hill_ordering(
        &self,
        predecessor: Option<elements_rs::Element>,
        has_carbon: bool,
    ) -> Result<Option<elements_rs::Element>, ()> {
        (**self).check_hill_ordering(predecessor, has_carbon)
    }
}

impl<T: ChargedMolecularTree<Count, Charge>, Count: CountLike, Charge: ChargeLike>
    ChargedMolecularTree<Count, Charge> for Box<T>
{
    fn charge(&self) -> f64 {
        (**self).charge()
    }

    fn isotopologue_mass_with_charge(&self) -> f64 {
        (**self).isotopologue_mass_with_charge()
    }

    fn molar_mass(&self) -> f64 {
        (**self).molar_mass()
    }
}
