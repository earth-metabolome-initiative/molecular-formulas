//! Properties that can be computed from trees of molecular nodes.

use crate::prelude::Element;
mod blankets;
mod chemical_tree;
mod inchi_tree;

pub(crate) use chemical_tree::ChemicalTree;
pub(crate) use inchi_tree::InChITree;
use num_traits::{CheckedAdd, CheckedMul, ConstOne, ConstZero};

/// Trait for computing various molecular properties.
pub trait MolecularTree<Count>: Sized {
    /// Type of the element iterator.
    type ElementIter<'a>: Iterator<Item = Element>
    where
        Self: 'a;

    /// Iterates over the elements in the molecular formula.
    ///
    /// # Implementation Notes
    ///
    /// Returns an iterator over the non-counted elements in the formula,
    /// which means that if an element appears with a count, it is still
    /// yielded only once.
    /// Isotopes, if present, are normalized to their base elements.
    /// If the formula contains residuals, they are ignored.
    fn elements(&self) -> Self::ElementIter<'_>;

    /// Returns whether the molecular tree contains any elements.
    fn contains_elements(&self) -> bool;

    /// Returns whether the molecular tree contains the provided element.
    fn contains_element(&self, element: Element) -> bool;

    /// Returns whether the molecular tree contains any isotopes.
    fn contains_isotopes(&self) -> bool;

    /// Returns whether the molecular tree contains the provided isotope.
    fn contains_isotope(&self, isotope: elements_rs::Isotope) -> bool;

    /// Returns the number of elements of a specific type in the molecular
    /// tree.
    ///
    /// Returns None if the provided data type C cannot represent the count.
    fn count_of_element<C>(&self, element: Element) -> Option<C>
    where
        C: From<Count> + CheckedAdd + CheckedMul + ConstZero + ConstOne;

    /// Returns the number of isotopes of a specific type in the molecular
    /// tree.
    ///
    /// Returns None if the provided data type C cannot represent the count.
    fn count_of_isotope<C>(&self, isotope: elements_rs::Isotope) -> Option<C>
    where
        C: From<Count> + CheckedAdd + CheckedMul + ConstZero + ConstOne;

    /// Returns the isotopologue mass of the molecular tree without considering
    /// any charge.
    fn isotopologue_mass(&self) -> f64;

    /// Returns whether the molecular tree is a noble gas compound.
    fn is_noble_gas_compound(&self) -> bool;

    /// Returns whether the molecular tree is Hill sorted.
    fn is_hill_sorted(&self) -> bool {
        let mut elements = self.elements().peekable();

        if elements.peek() == Some(&Element::C) {
            elements.next();
            // C cannot appear again
            if elements.peek() == Some(&Element::C) {
                return false;
            }

            if elements.peek() == Some(&Element::H) {
                elements.next();
                // H cannot appear again
                if elements.peek() == Some(&Element::H) {
                    return false;
                }
            }

            let mut previous_element: Option<Element> = None;
            for element in elements {
                if matches!(element, Element::C | Element::H) {
                    return false;
                }
                if let Some(prev) = previous_element {
                    let current_str: &str = element.as_ref();
                    let prev_str: &str = prev.as_ref();
                    if current_str <= prev_str {
                        return false;
                    }
                }
                previous_element = Some(element);
            }
        } else {
            let mut previous_element: Option<Element> = None;
            for element in elements {
                if element == Element::C {
                    return false;
                }
                if let Some(prev) = previous_element {
                    let current_str: &str = element.as_ref();
                    let prev_str: &str = prev.as_ref();
                    if current_str <= prev_str {
                        return false;
                    }
                }
                previous_element = Some(element);
            }
        }
        true
    }
}

/// Trait for molecular trees which can hold a charge.
pub trait ChargedMolecularTree<Count, Charge>: MolecularTree<Count> {
    /// Returns the charge of the molecular tree.
    fn charge(&self) -> f64;

    /// Returns the isotopologue mass with charge considered.
    fn isotopologue_mass_with_charge(&self) -> f64;

    /// Returns the molar mass.
    fn molar_mass(&self) -> f64;
}
