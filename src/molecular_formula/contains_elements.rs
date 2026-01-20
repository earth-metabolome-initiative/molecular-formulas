//! Submodule providing the `contains_elements` method for the
//! `MolecularFormula` struct

impl super::MolecularFormula {
    /// Checks if the molecular formula contains elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let formula: MolecularFormula = "H2O".parse().unwrap();
    /// assert!(formula.contains_elements());
    /// ```
    pub fn contains_elements(&self) -> bool {
        match self {
            Self::Element(_) => true,
            Self::Isotope(_) | Self::Residual | Self::Greek(_) => false,
            Self::Ion(ion) => ion.entry.contains_elements(),
            Self::Mixture(formulas) => {
                formulas.iter().any(|(_, formula)| formula.contains_elements())
            }
            Self::Sequence(formulas) => formulas.iter().any(Self::contains_elements),
            Self::Count(formula, _) | Self::RepeatingUnit(formula) | Self::Complex(formula) => {
                formula.contains_elements()
            }
            Self::Radical(formula, _) => formula.contains_elements(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::MolecularFormula;

    #[test]
    fn test_contains_elements_branches() {
        assert!(MolecularFormula::from_str("H").unwrap().contains_elements());
        // Isotopes are NOT "Elements" in this context
        assert!(!MolecularFormula::from_str("²H").unwrap().contains_elements());
        assert!(!MolecularFormula::from_str("R").unwrap().contains_elements());

        let mix = MolecularFormula::from_str("H2O.NaCl").unwrap();
        assert!(mix.contains_elements());

        let repeated = MolecularFormula::from_str("(CH2)2").unwrap();
        assert!(repeated.contains_elements());
    }
}
