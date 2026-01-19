use elements_rs::{Element, ElementVariant};

use crate::MolecularFormula;

impl MolecularFormula {
    /// Returns the number of atoms of the given element in the molecular
    /// formula.
    ///
    /// # Example
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use elements_rs::Element;
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let formula = MolecularFormula::try_from("C6H12O6")?;
    /// assert_eq!(formula.element_count(Element::C), 6);
    /// assert_eq!(formula.element_count(Element::H), 12);
    /// assert_eq!(formula.element_count(Element::O), 6);
    ///
    /// let formula = MolecularFormula::try_from("D2O")?;
    /// assert_eq!(formula.element_count(Element::H), 2);
    /// # Ok(())
    /// # }
    /// ```
    pub fn element_count(&self, element: impl Into<Element>) -> i32 {
        let element = element.into();
        self.element_count_internal(element)
    }

    fn element_count_internal(&self, target: Element) -> i32 {
        match self {
            MolecularFormula::Element(e) => {
                if *e == target {
                    1
                } else {
                    0
                }
            }
            MolecularFormula::Isotope(i) => {
                if i.element() == target {
                    1
                } else {
                    0
                }
            }
            MolecularFormula::Sequence(formulas) | MolecularFormula::Mixture(formulas) => {
                formulas.iter().map(|f| f.element_count_internal(target)).sum()
            }
            MolecularFormula::Ion(ion) => ion.entry.element_count_internal(target),
            MolecularFormula::Count(formula, count) => {
                formula.element_count_internal(target) * (*count as i32)
            }
            MolecularFormula::Complex(formula)
            | MolecularFormula::RepeatingUnit(formula)
            | MolecularFormula::Radical(formula, _) => formula.element_count_internal(target),
            MolecularFormula::Greek(_) | MolecularFormula::Residual => 0,
        }
    }
}
