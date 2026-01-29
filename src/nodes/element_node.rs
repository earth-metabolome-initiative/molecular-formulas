//! Submodule providing a struct and implementation of the `ExtensionTree` trait
//! for molecular formulas that can contain element symbols such as 'C', 'He',
//! 'Mg', etc.

use elements_rs::{BondsNumber, Element, RelativeAtomicMass};

use crate::{ChargedMolecularTree, MolecularTree};

impl<Count> MolecularTree<Count> for Element {
    type ElementIter<'a>
        = core::iter::Once<Element>
    where
        Self: 'a;

    type NonHydrogenElementIter<'a>
        = core::iter::Filter<core::iter::Once<Element>, fn(&Element) -> bool>
    where
        Self: 'a;

    #[inline]
    fn elements(&self) -> Self::ElementIter<'_> {
        core::iter::once(*self)
    }

    #[inline]
    fn non_hydrogens(&self) -> Self::NonHydrogenElementIter<'_> {
        core::iter::once(*self).filter(|&e| e != Element::H)
    }

    #[inline]
    fn contains_elements(&self) -> bool {
        true
    }

    #[inline]
    fn contains_non_hydrogens(&self) -> bool {
        *self != Element::H
    }

    #[inline]
    fn contains_isotopes(&self) -> bool {
        false
    }

    #[inline]
    fn contains_element(&self, element: Element) -> bool {
        *self == element
    }

    #[inline]
    fn contains_isotope(&self, _isotope: elements_rs::Isotope) -> bool {
        // TODO: Ask Pierre whether this is the desired behavior or
        // it should check whether the isotope is the natural one of the element.
        false
    }

    #[inline]
    fn number_of_elements(&self) -> usize {
        1
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
        Some(if *self == element { C::ONE } else { C::ZERO })
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
        self.relative_atomic_mass()
    }

    fn is_noble_gas_compound(&self) -> bool {
        self.is_noble_gas()
    }

    fn isotopic_normalization(&self) -> Self {
        *self
    }

    fn check_hill_ordering(
        &self,
        predecessor: Option<Element>,
        has_carbon: bool,
    ) -> Result<Option<Element>, ()> {
        if let Some(prev) = predecessor
            && !crate::molecular_tree::is_hill_sorted_pair(prev, *self, has_carbon)
        {
            return Err(());
        }
        Ok(Some(*self))
    }
}

impl<Count, Charge> ChargedMolecularTree<Count, Charge> for Element {
    fn charge(&self) -> f64 {
        0.0
    }

    fn isotopologue_mass_with_charge(&self) -> f64 {
        self.relative_atomic_mass()
    }

    fn molar_mass(&self) -> f64 {
        self.standard_atomic_weight()
    }
}
