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
    /// assert!(formula1.is_hill_sorted().unwrap());
    /// let formula2 = MolecularFormula::from_str("H2O").unwrap();
    /// assert!(formula2.is_hill_sorted().unwrap());
    /// let formula3 = MolecularFormula::from_str("C2H5OH").unwrap();
    /// assert!(!formula3.is_hill_sorted().unwrap());
    /// let formula4 = MolecularFormula::from_str("NaCl").unwrap();
    /// assert!(!formula4.is_hill_sorted().unwrap());
    /// let formula5 = MolecularFormula::from_str("C2H6O").unwrap();
    /// assert!(formula5.is_hill_sorted().unwrap());
    /// let formula6 = MolecularFormula::from_str("C6H8O6").unwrap();
    /// assert!(formula6.is_hill_sorted().unwrap());
    /// let formula7 = MolecularFormula::from_str("C16H25NS").unwrap();
    /// assert!(formula7.is_hill_sorted().unwrap());
    /// let mixture = MolecularFormula::from_str("C32H34N4O4.Ni").unwrap();
    /// assert!(mixture.is_hill_sorted().unwrap());
    /// let mixture2 = MolecularFormula::from_str("ClH.Na").unwrap();
    /// assert!(mixture2.is_hill_sorted().unwrap());
    /// let unsorted_mixture1 = MolecularFormula::from_str("C32H34O4N4.Ni").unwrap();
    /// assert!(!unsorted_mixture1.is_hill_sorted().unwrap());
    /// let residual_formula = MolecularFormula::from_str("C6H12O6.R").unwrap();
    /// assert!(residual_formula.is_hill_sorted().is_err());
    /// let unsorted_mixture2 = MolecularFormula::from_str("HCl.Na").unwrap();
    /// assert!(!unsorted_mixture2.is_hill_sorted().unwrap());
    /// let unsorted_mixture3 = MolecularFormula::from_str("C15H18O7.C15O6H16").unwrap();
    /// assert!(!unsorted_mixture3.is_hill_sorted().unwrap());
    /// let unsorted_formula = MolecularFormula::from_str("CH2SCl2O3").unwrap();
    /// assert!(!unsorted_formula.is_hill_sorted().unwrap());
    /// let unsorted_formula2 = MolecularFormula::from_str("C6H18NaNSi4").unwrap();
    /// assert!(!unsorted_formula2.is_hill_sorted().unwrap());
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
                let mut only_carbons = true;
                for element in self.iter_elements() {
                    if let Element::C = element {
                        found_carbon = true;
                        if previous.is_some() && !only_carbons {
                            // Carbon must be first
                            return Ok(false);
                        }
                    } else {
                        only_carbons = false;
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
