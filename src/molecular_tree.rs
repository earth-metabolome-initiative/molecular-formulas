//! Properties that can be computed from trees of molecular nodes.

use crate::prelude::Element;
mod blankets;
mod chemical_tree;
mod inchi_tree;

pub(crate) use chemical_tree::ChemicalTree;
pub(crate) use inchi_tree::InChITree;
use num_traits::{CheckedAdd, CheckedMul, ConstOne, ConstZero};

/// Helper to check if two elements are in Hill order.
#[must_use]
pub fn is_hill_sorted_pair(prev: Element, next: Element, has_carbon: bool) -> bool {
    if has_carbon {
        match (prev, next) {
            (Element::C, Element::C) | (Element::H, Element::H) => false,
            (Element::C, _) => true,
            (_, Element::C) => false,
            (Element::H, _) => true,
            (_, Element::H) => false,
            (a, b) => {
                let a_str: &str = a.as_ref();
                let b_str: &str = b.as_ref();
                a_str < b_str
            }
        }
    } else {
        let prev_str: &str = prev.as_ref();
        let next_str: &str = next.as_ref();
        prev_str < next_str
    }
}

/// Trait for computing various molecular properties.
pub trait MolecularTree<Count>: Sized {
    /// Type of the element iterator.
    type ElementIter<'a>: Iterator<Item = Element>
    where
        Self: 'a;

    /// Type of the non-hydrogen element iterator.
    type NonHydrogenElementIter<'a>: Iterator<Item = Element>
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

    /// Iterates over the non-hydrogen elements in the molecular formula.
    fn non_hydrogens(&self) -> Self::NonHydrogenElementIter<'_>;

    /// Returns whether the molecular tree contains any elements.
    fn contains_elements(&self) -> bool;

    /// Returns whether the molecular tree contains any non-hydrogen elements.
    fn contains_non_hydrogens(&self) -> bool;

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
    /// Returns the total number of elements in the molecular tree.
    fn number_of_elements(&self) -> usize;
    /// Returns the isotopologue mass of the molecular tree without considering
    /// any charge.
    fn isotopologue_mass(&self) -> f64;

    /// Returns whether the molecular tree is a noble gas compound.
    fn is_noble_gas_compound(&self) -> bool;

    #[must_use]
    /// Returns a new molecular tree with isotopic normalization applied.
    fn isotopic_normalization(&self) -> Self;

    /// Checks if the tree is Hill sorted given context about Carbon presence.
    ///
    /// The `predecessor` is the element that appeared immediately before the
    /// current subtree traversal.
    ///
    /// Returns `Ok(Some(last_element))` if the subtree is sorted and non-empty.
    /// Returns `Ok(predecessor)` (or `Ok(None)` if predecessor was None) if the
    /// subtree is empty. Returns `Err(())` if unsorted.
    ///
    /// # Errors
    ///
    /// Returns `Err(())` if the molecular tree is not Hill sorted.
    #[allow(clippy::result_unit_err)]
    fn check_hill_ordering(
        &self,
        predecessor: Option<Element>,
        has_carbon: bool,
    ) -> Result<Option<Element>, ()>;

    /// Returns whether the molecular tree is Hill sorted.
    fn is_hill_sorted(&self) -> bool {
        let has_carbon = self.contains_element(Element::C);
        self.check_hill_ordering(None, has_carbon).is_ok()
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

#[cfg(test)]
mod tests {
    use core::str::FromStr;

    use crate::{MolecularFormula, prelude::ChemicalFormula};

    #[test]
    fn test_is_hill_sorted_cases() {
        let cases = [
            // --- Branch 1: Starts with C ---
            // Valid cases
            ("C2H5O", true), // C, H, O (sorted alphabetically after H)
            ("CO2", true),   // C, O (Valid, no H)
            ("CH4", true),   // C, H (Valid)
            // Invalid: Branch 2 (C appears again immediately)
            ("CC", false),
            // Invalid: Branch 4 (H appears again immediately)
            // Note: C.H.H
            ("CHH", false),
            // Invalid: Branch 5 (C appears later)
            ("COC", false),
            // Invalid: Branch 5 (H appears later)
            ("CHBrH", false), // C, H, Br, H (H reappears)
            // Invalid: Branch 6 (Not sorted alphabetically after H)
            ("CHIBr", false), // I comes after Br, so I, Br is desc order.
            // --- Branch 7: Does not start with C ---
            // Valid cases
            ("H2O", true), // H, O. (H < O).
            ("ClH", true), // Cl, H. ("Cl" < "H" because 'C' < 'H').
            ("O2", true),  // O.
            // Invalid: Branch 8 (C appears later)
            ("HC", false),
            // Invalid: Branch 9 (Not sorted alphabetically)
            ("ON", false),  // O, N. "N" < "O". Returns false. (Unsorted: O > N).
            ("HCl", false), // H, Cl. "H" > "Cl". Returns false. (Unsorted).
        ];

        for (formula_str, expected) in cases {
            let formula = ChemicalFormula::<u32, i32>::from_str(formula_str)
                .unwrap_or_else(|_| panic!("Failed to parse {formula_str}"));

            assert_eq!(formula.is_hill_sorted(), expected, "Mismatch for formula {formula_str}");
        }
    }
}
