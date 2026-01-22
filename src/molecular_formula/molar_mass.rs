//! Submodule implementing the `molar_mass` method for the `MolecularFormula`
//! struct

use super::MolecularFormula;
use crate::NoResidualsTree;

impl<T: NoResidualsTree> MolecularFormula<T> {
    /// Returns the molar mass of the molecular formula.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let formula: MolecularFormula = "H2O".parse().unwrap();
    /// assert!((formula.molar_mass() - 18.015).abs() < 1e-2);
    /// ```
    ///
    /// # Errors
    ///
    /// * If the formula is a `Residual`, an error is returned.
    #[must_use]
    #[inline]
    pub fn molar_mass(&self) -> f64 {
        let mut total_mass = 0.0;
        for (count, component) in &self.mixtures {
            let n: f64 = (*count).into();
            total_mass += component.molar_mass() * n;
        }
        total_mass
    }
}
