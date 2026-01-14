//! Submodule providing the `iter_elements` method for the `MolecularFormula`
//! struct.

use elements_rs::{Element, ElementVariant};

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
}
