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
            MolecularFormula::Sequence(formulas) => {
                formulas.iter().map(|f| f.element_count_internal(target)).sum()
            }
            MolecularFormula::Mixture(formulas) => {
                formulas
                    .iter()
                    .map(|(count, f)| (*count as i32) * f.element_count_internal(target))
                    .sum()
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use elements_rs::Element;

    use crate::MolecularFormula;

    #[test]
    fn test_element_count_branches() {
        let mix = MolecularFormula::from_str("2H2O.NaCl").unwrap();
        // H: 2*2 = 4
        // O: 2*1 = 2
        // Na: 1
        // Cl: 1
        assert_eq!(mix.element_count(Element::H), 4);
        assert_eq!(mix.element_count(Element::O), 2);
        assert_eq!(mix.element_count(Element::Na), 1);

        // Isotope check
        let d2o = MolecularFormula::from_str("D2O").unwrap(); // D is H isotope
        assert_eq!(d2o.element_count(Element::H), 2);

        let residual = MolecularFormula::from_str("R").unwrap();
        assert_eq!(residual.element_count(Element::C), 0);
    }
}
