//! Submodule providing the `is_hill_sorted` method for the `MolecularFormula`
//! struct.

use elements_rs::Element;

use crate::{MolecularFormula, errors::Error};

impl MolecularFormula {
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
    /// let formula1 = MolecularFormula::from_str("C6H12O6").unwrap();
    /// assert!(formula1.is_hill_sorted().unwrap(), "Formula `C6H12O6` should be Hill sorted");
    /// let formula2 = MolecularFormula::from_str("H2O").unwrap();
    /// assert!(formula2.is_hill_sorted().unwrap(), "Formula `H2O` should be Hill sorted");
    /// let formula3 = MolecularFormula::from_str("C2H5OH").unwrap();
    /// assert!(!formula3.is_hill_sorted().unwrap(), "Formula `C2H5OH` should not be Hill sorted");
    /// let formula4 = MolecularFormula::from_str("NaCl").unwrap();
    /// assert!(!formula4.is_hill_sorted().unwrap(), "Formula `NaCl` should not be Hill sorted");
    /// let formula5 = MolecularFormula::from_str("C2H6O").unwrap();
    /// assert!(formula5.is_hill_sorted().unwrap(), "Formula `C2H6O` should be Hill sorted");
    /// let formula6 = MolecularFormula::from_str("C6H8O6").unwrap();
    /// assert!(formula6.is_hill_sorted().unwrap(), "Formula `C6H8O6` should be Hill sorted");
    /// let formula7 = MolecularFormula::from_str("C16H25NS").unwrap();
    /// assert!(formula7.is_hill_sorted().unwrap(), "Formula `C16H25NS` should be Hill sorted");
    /// let mixture = MolecularFormula::from_str("C32H34N4O4.Ni").unwrap();
    /// assert!(mixture.is_hill_sorted().unwrap(), "Mixture `C32H34N4O4.Ni` should be Hill sorted");
    /// let mixture2 = MolecularFormula::from_str("ClH.Na").unwrap();
    /// assert!(mixture2.is_hill_sorted().unwrap(), "Mixture `ClH.Na` should be Hill sorted");
    /// let unsorted_mixture1 = MolecularFormula::from_str("C32H34O4N4.Ni").unwrap();
    /// assert!(
    ///     !unsorted_mixture1.is_hill_sorted().unwrap(),
    ///     "Mixture `C32H34O4N4.Ni` should not be Hill sorted"
    /// );
    /// let residual_formula = MolecularFormula::from_str("C6H12O6.R").unwrap();
    /// assert!(
    ///     residual_formula.is_hill_sorted().is_err(),
    ///     "Formula `C6H12O6.R` should return an error when checking if Hill sorted"
    /// );
    /// let unsorted_mixture2 = MolecularFormula::from_str("HCl.Na").unwrap();
    /// assert!(
    ///     !unsorted_mixture2.is_hill_sorted().unwrap(),
    ///     "Mixture `HCl.Na` should not be Hill sorted"
    /// );
    /// let unsorted_mixture3 = MolecularFormula::from_str("C15H18O7.C15O6H16").unwrap();
    /// assert!(
    ///     !unsorted_mixture3.is_hill_sorted().unwrap(),
    ///     "Mixture `C15H18O7.C15O6H16` should not be Hill sorted"
    /// );
    /// let unsorted_formula = MolecularFormula::from_str("CH2SCl2O3").unwrap();
    /// assert!(
    ///     !unsorted_formula.is_hill_sorted().unwrap(),
    ///     "Formula `CH2SCl2O3` should not be Hill sorted"
    /// );
    /// let unsorted_formula2 = MolecularFormula::from_str("C6H18NaNSi4").unwrap();
    /// assert!(
    ///     !unsorted_formula2.is_hill_sorted().unwrap(),
    ///     "Formula `C6H18NaNSi4` should not be Hill sorted"
    /// );
    /// ```
    ///
    /// # Errors
    ///
    /// * If the molecular formula contains unsortable tokens, such as
    ///   residuals.
    pub fn is_hill_sorted(&self) -> Result<bool, Error> {
        if self.contains_residual() {
            return Err(Error::InvalidOperationForResidual);
        }
        if self.has_repeated_elements() {
            return Ok(false);
        }
        Ok(match self {
            MolecularFormula::Mixture(mixtures) => {
                let mut sorted = true;
                for formula in mixtures {
                    sorted &= formula.is_hill_sorted()?;
                }
                sorted
            }
            _ => {
                let mut previous = None;
                let mut found_carbon = false;
                for element in self.iter_elements() {
                    if let Element::C = element {
                        found_carbon = true;
                        if previous.is_some() {
                            // Carbon must be first
                            return Ok(false);
                        }
                    }

                    if let Element::H = element
                        && found_carbon
                        && previous != Some(Element::C)
                    {
                        return Ok(false);
                    }

                    // Otherwise, elements must be in alphabetical order
                    if let Some(prev) = previous {
                        let prev_symbol: &str = prev.as_ref();
                        let element_symbol: &str = element.as_ref();
                        if element_symbol < prev_symbol {
                            return Ok(false);
                        }
                    }

                    previous = Some(element);
                }
                true
            }
        })
    }
}
