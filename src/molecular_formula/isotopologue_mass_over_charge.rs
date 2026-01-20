//! Submodule implementing the `isotopologue_mass_over_charge` methods for the
//! `MolecularFormula` struct

impl super::MolecularFormula {
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
    /// let formula = MolecularFormula::from_str("OH-").unwrap();
    /// let m_z = formula.isotopologue_mass_over_charge().unwrap();
    /// assert!((m_z - -17.00328823171).abs() < 1e-9);
    /// ```
    ///
    /// # Errors
    ///
    /// * If the `MolecularFormula` contains Residual.
    pub fn isotopologue_mass_over_charge(&self) -> Result<f64, crate::errors::Error> {
        Ok(self.isotopologue_mass_with_charge()? / f64::from(self.charge()?))
    }
}
