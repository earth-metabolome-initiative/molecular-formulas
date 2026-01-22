//! Submodule providing the `is_hill_sorted` method for the `MolecularFormula`
//! struct.

use crate::{MolecularFormula, NoResidualsTree};

impl<T: NoResidualsTree> MolecularFormula<T> {
    /// Returns whether the molecular formula is sorted according to Hill
    /// system.
    ///
    /// If the formula contains carbon atoms, they must be listed first,
    /// followed by hydrogen atoms, and then all other elements in
    /// alphabetical order. If the formula does not contain carbon atoms,
    /// all elements must be listed in alphabetical order, including hydrogen.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let formula1: MolecularFormula = MolecularFormula::from_str("C6H12O6").unwrap();
    /// assert!(formula1.is_hill_sorted(), "Formula `C6H12O6` should be Hill sorted");
    /// let formula2: MolecularFormula = MolecularFormula::from_str("H2O").unwrap();
    /// assert!(formula2.is_hill_sorted(), "Formula `H2O` should be Hill sorted");
    /// let formula3: MolecularFormula = MolecularFormula::from_str("C2H5OH").unwrap();
    /// assert!(!formula3.is_hill_sorted(), "Formula `C2H5OH` should not be Hill sorted");
    /// let formula4: MolecularFormula = MolecularFormula::from_str("NaCl").unwrap();
    /// assert!(!formula4.is_hill_sorted(), "Formula `NaCl` should not be Hill sorted");
    /// let formula5: MolecularFormula = MolecularFormula::from_str("C2H6O").unwrap();
    /// assert!(formula5.is_hill_sorted(), "Formula `C2H6O` should be Hill sorted");
    /// let formula6: MolecularFormula = MolecularFormula::from_str("C6H8O6").unwrap();
    /// assert!(formula6.is_hill_sorted(), "Formula `C6H8O6` should be Hill sorted");
    /// let formula7: MolecularFormula = MolecularFormula::from_str("C16H25NS").unwrap();
    /// assert!(formula7.is_hill_sorted(), "Formula `C16H25NS` should be Hill sorted");
    /// let formula8: MolecularFormula = MolecularFormula::from_str("C28H23ClO7").unwrap();
    /// assert!(formula8.is_hill_sorted(), "Formula `{formula8}` should be Hill sorted");
    /// let mixture: MolecularFormula = MolecularFormula::from_str("C32H34N4O4.Ni").unwrap();
    /// assert!(mixture.is_hill_sorted(), "Mixture `C32H34N4O4.Ni` should be Hill sorted");
    /// let mixture2: MolecularFormula = MolecularFormula::from_str("ClH.Na").unwrap();
    /// assert!(mixture2.is_hill_sorted(), "Mixture `ClH.Na` should be Hill sorted");
    /// let mixture3: MolecularFormula = MolecularFormula::from_str("C20H18F3N4O8P.Na").unwrap();
    /// assert!(mixture3.is_hill_sorted(), "Mixture `{mixture3}` should be Hill sorted");
    /// let unsorted_mixture1: MolecularFormula = MolecularFormula::from_str("C32H34O4N4.Ni").unwrap();
    /// assert!(
    ///     !unsorted_mixture1.is_hill_sorted(),
    ///     "Mixture `C32H34O4N4.Ni` should not be Hill sorted"
    /// );
    /// let unsorted_mixture2: MolecularFormula = MolecularFormula::from_str("HCl.Na").unwrap();
    /// assert!(!unsorted_mixture2.is_hill_sorted(), "Mixture `HCl.Na` should not be Hill sorted");
    /// let unsorted_mixture3: MolecularFormula =
    ///     MolecularFormula::from_str("C15H18O7.C15O6H16").unwrap();
    /// assert!(
    ///     !unsorted_mixture3.is_hill_sorted(),
    ///     "Mixture `C15H18O7.C15O6H16` should not be Hill sorted"
    /// );
    /// let unsorted_formula: MolecularFormula = MolecularFormula::from_str("CH2SCl2O3").unwrap();
    /// assert!(!unsorted_formula.is_hill_sorted(), "Formula `CH2SCl2O3` should not be Hill sorted");
    /// let unsorted_formula2: MolecularFormula = MolecularFormula::from_str("C6H18NaNSi4").unwrap();
    /// assert!(!unsorted_formula2.is_hill_sorted(), "Formula `C6H18NaNSi4` should not be Hill sorted");
    /// ```
    #[must_use]
    pub fn is_hill_sorted(&self) -> bool {
        let mut sorted = true;
        for (_, component) in self.as_ref() {
            if !component.is_hill_sorted() {
                sorted = false;
            }
        }
        sorted
    }
}
