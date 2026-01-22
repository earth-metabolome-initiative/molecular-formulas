//! Submodule providing the `contains_mixture` method for the
//! `MolecularFormula` struct

use std::iter::repeat_n;

use crate::Tree;

impl<T: Tree> super::MolecularFormula<T> {
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
    #[must_use]
    #[inline]
    pub fn number_of_mixtures(&self) -> u64 {
        self.as_ref()
            .iter()
            .map(|(repeats, _)| {
                let n: u64 = (*repeats).into();
                n
            })
            .sum()
    }

    /// Returns an iterator over the mixtures in the molecular formula.
    ///
    /// # Examples
    ///
    /// ```
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let mixture: MolecularFormula = "H2O.D2O".parse().unwrap();
    /// let components: Vec<String> = mixture.mixtures().map(|t| format!("{t:?}")).collect();
    ///
    /// assert_eq!(components.len(), 2);
    /// ```
    #[inline]
    pub fn mixtures(&self) -> impl Iterator<Item = &T> + '_ {
        self.as_ref().iter().flat_map(|(repeats, component)| {
            let repeats: u64 = (*repeats).into();
            repeat_n(component, usize::try_from(repeats).unwrap_or(usize::MAX))
        })
    }

    /// Returns an iterator over the mixtures in the molecular formula, as
    /// sub-formulas.
    ///
    /// # Examples
    ///
    /// ```
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let mixture: MolecularFormula = "H2O.NaCl".parse().unwrap();
    /// let subformulas: Vec<MolecularFormula> = mixture.subformulas().collect();
    ///
    /// assert_eq!(subformulas[0].to_string(), "H₂O");
    /// assert_eq!(subformulas[1].to_string(), "NaCl");
    /// ```
    #[inline]
    pub fn subformulas(&self) -> impl Iterator<Item = Self> + '_ {
        self.as_ref().iter().flat_map(|(repeats, component)| {
            let repeats: u64 = (*repeats).into();
            repeat_n(component, usize::try_from(repeats).unwrap_or(usize::MAX))
                .map(|c| c.clone().into())
        })
    }
}
