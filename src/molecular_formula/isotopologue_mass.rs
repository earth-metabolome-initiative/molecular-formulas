//! Submodule implementing the `isotopologue_mass_with_charge` and
//! `isotopologue_mass_without_charge` methods for the `MolecularFormula` struct

use super::MolecularFormula;
use crate::NoResidualsTree;

impl<T: NoResidualsTree> MolecularFormula<T> {
    /// Returns the isotopologue mass of the molecular formula, including the
    /// charge.
    ///
    /// # Examples
    ///
    /// ```
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let formula: MolecularFormula = "H2O".parse().unwrap();
    /// let mass = formula.isotopologue_mass_with_charge();
    /// assert!((mass - 18.01056).abs() < 1e-4);
    /// ```
    ///
    /// # Errors
    ///
    /// * If the `MolecularFormula` contains Residual.
    #[must_use]
    #[inline]
    pub fn isotopologue_mass_with_charge(&self) -> f64 {
        let mut total_mass = 0.0;
        for (repeats, component) in &self.mixtures {
            let mass = component.isotopologue_mass_with_charge();
            let n: f64 = (*repeats).into();
            total_mass += mass * n;
        }
        total_mass
    }

    /// Returns the isotopologue mass of the molecular formula, excluding the
    /// charge.
    ///
    /// # Examples
    ///
    /// ```
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let formula: MolecularFormula = "[Na]+".parse().unwrap();
    /// // Mass of atomic Na, ignoring the missing electron
    /// assert!((formula.isotopologue_mass_without_charge() - 22.98977).abs() < 1e-4);
    /// ```
    ///
    /// # Errors
    ///
    /// * If the `MolecularFormula` contains Residual.
    #[must_use]
    #[inline]
    pub fn isotopologue_mass_without_charge(&self) -> f64 {
        let mut total_mass = 0.0;
        for (repeats, component) in &self.mixtures {
            let mass = component.isotopologue_mass_without_charge();
            let n: f64 = (*repeats).into();
            total_mass += mass * n;
        }
        total_mass
    }
}
