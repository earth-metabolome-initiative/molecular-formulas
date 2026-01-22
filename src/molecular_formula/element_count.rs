use elements_rs::Element;

use super::{MolecularFormula, Tree};

impl<T: Tree> MolecularFormula<T> {
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
    /// let formula: MolecularFormula = MolecularFormula::try_from("C6H12O6")?;
    /// assert_eq!(formula.element_count(Element::C), Some(6));
    /// assert_eq!(formula.element_count(Element::H), Some(12));
    /// assert_eq!(formula.element_count(Element::O), Some(6));
    ///
    /// let formula: MolecularFormula = MolecularFormula::try_from("D2O")?;
    /// assert_eq!(formula.element_count(Element::H), Some(2));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn element_count(&self, element: impl Into<Element>) -> Option<u64> {
        let target = element.into();
        let mut count = 0u64;
        for (repeats, tree) in &self.mixtures {
            let n: u64 = (*repeats).into();
            let c = tree.element_count(target)?;
            count = count.checked_add(n.checked_mul(c)?)?;
        }
        Some(count)
    }

    /// Returns the number of atoms of the given isotope in the molecular
    /// formula.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use elements_rs::isotopes::HydrogenIsotope;
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let formula: MolecularFormula = MolecularFormula::try_from("²H2O")?;
    /// assert_eq!(formula.isotope_count(HydrogenIsotope::D), Some(2));
    /// assert_eq!(formula.isotope_count(HydrogenIsotope::H1), Some(0));
    /// assert_eq!(formula.isotope_count(HydrogenIsotope::T), Some(0));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn isotope_count(&self, isotope: impl Into<elements_rs::Isotope>) -> Option<u64> {
        let target = isotope.into();
        let mut count = 0u64;
        for (repeats, tree) in &self.mixtures {
            let n: u64 = (*repeats).into();
            let c = tree.isotope_count(target)?;
            count = count.checked_add(n.checked_mul(c)?)?;
        }
        Some(count)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use elements_rs::Element;

    use crate::{MolecularFormula, ResidualFormula};

    #[test]
    fn test_element_count_branches() {
        let mix: MolecularFormula = MolecularFormula::from_str("2H2O.NaCl").unwrap();
        // H: 2*2 = 4
        // O: 2*1 = 2
        // Na: 1
        // Cl: 1
        assert_eq!(mix.element_count(Element::H), Some(4));
        assert_eq!(mix.element_count(Element::O), Some(2));
        assert_eq!(mix.element_count(Element::Na), Some(1));

        // Isotope check
        let d2o: MolecularFormula = MolecularFormula::from_str("D2O").unwrap(); // D is H isotope
        assert_eq!(d2o.element_count(Element::H), Some(2));

        let residual: ResidualFormula = MolecularFormula::from_str("R").unwrap();
        assert_eq!(residual.element_count(Element::C), Some(0));
    }
}
