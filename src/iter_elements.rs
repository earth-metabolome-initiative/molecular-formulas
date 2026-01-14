//! Submodule providing the `iter_elements` method for the `MolecularFormula`
//! struct.

use elements_rs::{Element, ElementMask, ElementVariant};

use crate::MolecularFormula;

impl MolecularFormula {
    /// Returns an iterator over all elements in the molecular formula.
    pub fn iter_elements(&self) -> Box<dyn Iterator<Item = Element>> {
        match self {
            MolecularFormula::Element(element) => Box::new(std::iter::once(*element)),
            MolecularFormula::Isotope(isotope) => Box::new(std::iter::once(isotope.element())),
            MolecularFormula::Sequence(formulas) | MolecularFormula::Mixture(formulas) => {
                Box::new(
                    formulas.iter().flat_map(|f| f.iter_elements()).collect::<Vec<_>>().into_iter(),
                )
            }
            MolecularFormula::Ion(ion) => Box::new(ion.entry.iter_elements()),
            MolecularFormula::Count(formula, _)
            | MolecularFormula::Complex(formula)
            | MolecularFormula::RepeatingUnit(formula)
            | MolecularFormula::Radical(formula, _) => Box::new(formula.iter_elements()),
            MolecularFormula::Greek(_) | MolecularFormula::Residual => {
                // These types do not contain elements
                Box::new(std::iter::empty())
            }
        }
    }

    /// Returns an iterator over all elements in the molecular formula,
    /// repeating the repeating units according to their counts.
    pub fn iter_counted_elements(&self) -> Box<dyn Iterator<Item = Element>> {
        match self {
            MolecularFormula::Element(element) => Box::new(std::iter::once(*element)),
            MolecularFormula::Isotope(isotope) => Box::new(std::iter::once(isotope.element())),
            MolecularFormula::Sequence(formulas) | MolecularFormula::Mixture(formulas) => {
                Box::new(
                    formulas
                        .iter()
                        .flat_map(|f| f.iter_counted_elements())
                        .collect::<Vec<_>>()
                        .into_iter(),
                )
            }
            MolecularFormula::Ion(ion) => Box::new(ion.entry.iter_counted_elements()),
            MolecularFormula::Count(formula, repeats) => {
                let mut iterator: Box<dyn Iterator<Item = Element>> = Box::new(std::iter::empty());
                for _ in 0..*repeats {
                    iterator = Box::new(iterator.chain(formula.iter_counted_elements()));
                }
                iterator
            }
            MolecularFormula::Complex(formula)
            | MolecularFormula::RepeatingUnit(formula)
            | MolecularFormula::Radical(formula, _) => Box::new(formula.iter_counted_elements()),
            MolecularFormula::Greek(_) | MolecularFormula::Residual => {
                // These types do not contain elements
                Box::new(std::iter::empty())
            }
        }
    }

    /// Returns whether the molecular formula has repeated elements.
    ///
    /// Grouped elements are those that are represented as Element/Isotope
    /// plus a count, e.g., C6, H12, O6 in C6H12O6, and not
    /// `CCHHHHHHHHHHHOOOOOO`.
    pub fn has_repeated_elements(&self) -> bool {
        match self {
            MolecularFormula::Mixture(formulas) => {
                for formula in formulas {
                    if formula.has_repeated_elements() {
                        return true;
                    }
                }
            }
            _ => {
                let mut element_mask = ElementMask::default();
                for element in self.iter_elements() {
                    if !element_mask.insert(element) {
                        return true;
                    }
                }
            }
        }
        false
    }
}
