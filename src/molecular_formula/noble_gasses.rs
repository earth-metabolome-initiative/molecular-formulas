//! Submodule providing methods to work with molecular formulas containing noble
//! gasses.

use elements_rs::BondsNumber;

use crate::NoResidualsTree;

impl<T: NoResidualsTree> crate::MolecularFormula<T> {
    /// Returns whether the formula solely contains noble gasses.
    ///
    /// # Example
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let helium: MolecularFormula = MolecularFormula::try_from("He")?;
    /// assert!(helium.is_noble_gas_compound());
    ///
    /// let argon: MolecularFormula = MolecularFormula::try_from("Ar")?;
    /// assert!(argon.is_noble_gas_compound());
    ///
    /// let water: MolecularFormula = MolecularFormula::try_from("H2O")?;
    /// assert!(!water.is_noble_gas_compound());
    ///
    /// // A mixture of noble gasses is considered a noble gas compound
    /// let noble_gas_mixture: MolecularFormula = MolecularFormula::try_from("HeAr")?;
    /// assert!(noble_gas_mixture.is_noble_gas_compound());
    ///
    /// // Xenon tetrafluoride contains Fluorine, so it is not solely composed of noble gasses
    /// let xenon_tetrafluoride: MolecularFormula = MolecularFormula::try_from("XeF4")?;
    /// assert!(!xenon_tetrafluoride.is_noble_gas_compound());
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// * If the formula contains a non-element, such as a `Residual`.
    #[must_use]
    #[inline]
    pub fn is_noble_gas_compound(&self) -> bool {
        self.iter_elements().all(|element| element.is_noble_gas())
    }
}
