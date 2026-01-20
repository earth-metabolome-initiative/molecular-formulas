//! Submodule providing the `contains_residual` method for the
//! `MolecularFormula` struct

impl super::MolecularFormula {
    /// Checks if the molecular formula contains a residual.
    ///
    /// # Examples
    ///
    /// ```
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let formula: MolecularFormula = "H2O".parse().unwrap();
    /// assert!(!formula.contains_residual());
    /// let formula_with_residual: MolecularFormula = "C6H5R".parse().unwrap();
    /// assert!(formula_with_residual.contains_residual());
    /// ```
    pub fn contains_residual(&self) -> bool {
        match self {
            Self::Element(_) | Self::Isotope(_) | Self::Greek(_) => false,
            Self::Ion(ion) => ion.entry.contains_residual(),
            Self::Mixture(formulas) => {
                formulas.iter().any(|(_, formula)| formula.contains_residual())
            }
            Self::Sequence(formulas) => formulas.iter().any(Self::contains_residual),
            Self::Count(formula, _) | Self::RepeatingUnit(formula) | Self::Complex(formula) => {
                formula.contains_residual()
            }
            Self::Residual => true,
            Self::Radical(formula, _) => formula.contains_residual(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::MolecularFormula;

    #[test]
    fn test_contains_residual_branches() {
        assert!(!MolecularFormula::from_str("H").unwrap().contains_residual());
        assert!(MolecularFormula::from_str("R").unwrap().contains_residual());

        // Mixture with residual
        let mix = MolecularFormula::from_str("H2O.R").unwrap();
        assert!(mix.contains_residual());

        // Nested in complex/radical
        let rad = MolecularFormula::from_str("R·").unwrap(); // Radical
        assert!(rad.contains_residual());
    }
}
