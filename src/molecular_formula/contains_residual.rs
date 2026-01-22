//! Submodule providing the `contains_residuals` method for the
//! `MolecularFormula` struct

use crate::molecular_formula::trees::ResidualTree;

impl<T: ResidualTree> super::MolecularFormula<T> {
    /// Checks if the molecular formula contains a residual.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use molecular_formulas::ResidualFormula;
    ///
    /// let formula: ResidualFormula = "H2O".parse().unwrap();
    /// assert!(!formula.contains_residuals());
    /// let formula_with_residual: ResidualFormula = "C6H5R".parse().unwrap();
    /// assert!(formula_with_residual.contains_residuals());
    /// ```
    #[must_use]
    #[inline]
    pub fn contains_residuals(&self) -> bool {
        self.as_ref().iter().any(|(_, component)| component.contains_residuals())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::ResidualFormula;

    #[test]
    fn test_contains_residuals_branches() {
        assert!(!ResidualFormula::from_str("H").unwrap().contains_residuals());
        assert!(ResidualFormula::from_str("R").unwrap().contains_residuals());

        // Mixture with residual
        let mix = ResidualFormula::from_str("H2O.R").unwrap();
        assert!(mix.contains_residuals());

        // Nested in complex/radical
        let rad = ResidualFormula::from_str("R·").unwrap(); // Radical
        assert!(rad.contains_residuals());
    }
}
