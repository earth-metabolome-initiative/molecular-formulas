//! Submodule providing the `contains_mixture` method for the
//! `MolecularFormula` struct

impl super::MolecularFormula {
    /// Checks if the molecular formula contains a mixture.
    pub fn contains_mixture(&self) -> bool {
        match self {
            Self::Element(_) | Self::Residual | Self::Isotope(_) | Self::Greek(_) => false,
            Self::Ion(ion) => ion.entry.contains_mixture(),
            Self::Mixture(_) => true,
            Self::Sequence(formulas) => formulas.iter().any(Self::contains_mixture),
            Self::Count(formula, _) | Self::RepeatingUnit(formula) | Self::Complex(formula) => {
                formula.contains_mixture()
            }
            Self::Radical(formula, _) => {
                debug_assert!(!formula.contains_mixture(), "Radical should not contain mixtures");
                false
            }
        }
    }

    /// Returns the number of mixtures in the molecular formula.
    ///
    /// # Examples
    ///
    /// ```
    /// use molecular_formulas::MolecularFormula;
    ///
    /// // Simple mixture: H2O.D2O (2 components)
    /// let mixture: MolecularFormula = "H2O.D2O".parse().unwrap();
    /// assert_eq!(mixture.number_of_mixtures(), 2);
    ///
    /// // No mixture
    /// let formula: MolecularFormula = "H2O".parse().unwrap();
    /// assert_eq!(formula.number_of_mixtures(), 1);
    ///
    /// // Mixture with 3 components
    /// let mixture: MolecularFormula = "H2O.D2O.T2O".parse().unwrap();
    /// assert_eq!(
    ///     mixture.number_of_mixtures(),
    ///     3,
    ///     "Mixture should have 3 components, but found {}",
    ///     mixture.mixtures().map(|f| f.to_string()).collect::<Vec<_>>().join(", ")
    /// );
    /// ```
    pub fn number_of_mixtures(&self) -> usize {
        match self {
            Self::Mixture(mixture) => mixture.len(),
            _ => 1,
        }
    }

    /// Returns an iterator over the mixtures in the molecular formula.
    pub fn mixtures(&self) -> Box<dyn Iterator<Item = &Self> + '_> {
        match self {
            Self::Mixture(mixture) => Box::new(mixture.iter()),
            _ => Box::new(std::iter::once(self)),
        }
    }
}
