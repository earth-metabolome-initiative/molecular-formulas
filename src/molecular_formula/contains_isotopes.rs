//! Submodule providing the `contains_isotope` method for the
//! `MolecularFormula` struct

impl super::MolecularFormula {
    /// Checks if the molecular formula is isotopically defined.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let water = MolecularFormula::try_from("H2O")?;
    /// assert!(!water.contains_isotope());
    ///
    /// let formula = MolecularFormula::try_from("³H2O")?;
    /// assert!(formula.contains_isotope());
    ///
    /// let formula = MolecularFormula::try_from("D2O")?;
    /// assert!(formula.contains_isotope());
    ///
    /// let formula = MolecularFormula::try_from("T2O")?;
    /// assert!(formula.contains_isotope());
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn contains_isotope(&self) -> bool {
        match self {
            Self::Element(_) | Self::Residual | Self::Greek(_) => false,
            Self::Isotope(_) => true,
            Self::Ion(ion) => ion.entry.contains_isotope(),
            Self::Mixture(formulas) => {
                formulas.iter().any(|(_, formula)| formula.contains_isotope())
            }
            Self::Sequence(formulas) => formulas.iter().any(Self::contains_isotope),
            Self::Count(formula, _) | Self::RepeatingUnit(formula) | Self::Complex(formula) => {
                formula.contains_isotope()
            }
            Self::Radical(formula, _) => formula.contains_isotope(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::MolecularFormula;

    #[test]
    fn test_contains_isotope_branches() {
        assert!(!MolecularFormula::from_str("H").unwrap().contains_isotope());
        assert!(MolecularFormula::from_str("²H").unwrap().contains_isotope());

        // Mixture with isotope
        let mix = MolecularFormula::from_str("D2O.H2O").unwrap(); // D is isotope of H
        assert!(mix.contains_isotope());

        let mix_clean = MolecularFormula::from_str("H2O.NaCl").unwrap();
        assert!(!mix_clean.contains_isotope());
    }
}
