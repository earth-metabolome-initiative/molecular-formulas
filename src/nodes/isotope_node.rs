//! Submodule providing a struct and implementation of the `ExtensionTree` trait
//! for molecular formulas that can contain isotopes such as `[13C]`.

use elements_rs::{BondsNumber, Element, ElementVariant, Isotope, RelativeAtomicMass};

use crate::{ChargedMolecularTree, MolecularTree};

impl<Count> MolecularTree<Count> for Isotope {
    type ElementIter<'a>
        = core::iter::Once<Element>
    where
        Self: 'a;

    #[inline]
    fn elements(&self) -> Self::ElementIter<'_> {
        core::iter::once(self.element())
    }

    #[inline]
    fn contains_elements(&self) -> bool {
        true
    }

    #[inline]
    fn contains_isotopes(&self) -> bool {
        true
    }

    #[inline]
    fn contains_element(&self, element: Element) -> bool {
        self.element() == element
    }

    #[inline]
    fn contains_isotope(&self, isotope: elements_rs::Isotope) -> bool {
        *self == isotope
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
        Some(if self.element() == element { C::ONE } else { C::ZERO })
    }

    #[inline]
    fn count_of_isotope<C>(&self, isotope: Isotope) -> Option<C>
    where
        C: From<Count>
            + num_traits::CheckedAdd
            + num_traits::CheckedMul
            + num_traits::ConstZero
            + num_traits::ConstOne,
    {
        Some(if *self == isotope { C::ONE } else { C::ZERO })
    }

    fn isotopologue_mass(&self) -> f64 {
        self.relative_atomic_mass()
    }

    #[inline]
    fn is_noble_gas_compound(&self) -> bool {
        self.is_noble_gas()
    }
}

impl<Count, Charge> ChargedMolecularTree<Count, Charge> for Isotope {
    fn charge(&self) -> f64 {
        0.0
    }

    fn isotopologue_mass_with_charge(&self) -> f64 {
        self.relative_atomic_mass()
    }

    fn molar_mass(&self) -> f64 {
        self.element().standard_atomic_weight()
    }
}
