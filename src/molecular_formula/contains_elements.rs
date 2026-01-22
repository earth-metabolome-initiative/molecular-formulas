//! Submodule providing the `contains_elements` method for the
//! `MolecularFormula` struct

use super::Tree;

impl<T: Tree> super::MolecularFormula<T> {
    /// Checks if the molecular formula contains elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let formula: MolecularFormula = "H2O".parse().unwrap();
    /// assert!(formula.contains_elements());
    /// ```
    #[must_use]
    #[inline]
    pub fn contains_elements(&self) -> bool {
        self.iter_elements().next().is_some()
    }

    /// Returns whether the molecular formula contains a specific element.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::Element;
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let formula: MolecularFormula = "C6H12O6".parse().unwrap();
    /// assert!(formula.contains_element(Element::C));
    /// assert!(formula.contains_element(Element::H));
    /// assert!(formula.contains_element(Element::O));
    /// assert!(!formula.contains_element(Element::N));
    /// ```
    #[inline]
    pub fn contains_element(&self, element: impl Into<elements_rs::Element>) -> bool {
        let element = element.into();
        self.iter_elements().any(|el| el == element)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{DefaultTree, MolecularFormula};

    #[test]
    fn test_contains_elements_branches() {
        assert!(MolecularFormula::<DefaultTree>::from_str("H").unwrap().contains_elements());
        assert!(MolecularFormula::<DefaultTree>::from_str("²H").unwrap().contains_elements());
        // Residuals are not supported in DefaultTree
        assert!(MolecularFormula::<DefaultTree>::from_str("R").is_err());

        let mix = MolecularFormula::<DefaultTree>::from_str("H2O.NaCl").unwrap();
        assert!(mix.contains_elements());

        let repeated = MolecularFormula::<DefaultTree>::from_str("(CH2)2").unwrap();
        assert!(repeated.contains_elements());
    }
}
