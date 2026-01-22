//! Submodule implementing the `isotopologue_mass_over_charge` methods for the
//! `MolecularFormula` struct

use crate::NoResidualsTree;

impl<T: NoResidualsTree> super::MolecularFormula<T> {
    /// Returns the isotopologue mass over charge for the given molecular
    /// formula. Equivalent to `isotopologue_mass_with_charge` divided by the
    /// charge.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let formula: MolecularFormula = MolecularFormula::from_str("OH-").unwrap();
    /// let m_z = formula.isotopologue_mass_over_charge();
    /// assert!((m_z - -17.00328823171).abs() < 1e-9);
    /// ```
    ///
    /// # Errors
    ///
    /// * If the `MolecularFormula` contains Residual.
    #[must_use]
    #[inline]
    pub fn isotopologue_mass_over_charge(&self) -> f64 {
        self.isotopologue_mass_with_charge() / self.charge()
    }
}
