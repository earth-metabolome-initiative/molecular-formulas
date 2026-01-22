//! Submodule providing the `iter_elements` method for the `MolecularFormula`
//! struct.

use elements_rs::Element;

use crate::{MolecularFormula, NoResidualsTree, molecular_formula::Tree};

impl<T: Tree> MolecularFormula<T> {
    /// Returns an iterator over all elements in the molecular formula.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use elements_rs::Element;
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let formula: MolecularFormula = MolecularFormula::from_str("C2H5OH").unwrap();
    /// let elements: Vec<Element> = formula.iter_elements().collect();
    ///
    /// // Yields elements as they appear in the parsed structure components.
    /// // C2 -> C
    /// // H5 -> H
    /// // O  -> O
    /// // H  -> H
    /// assert_eq!(elements, vec![Element::C, Element::H, Element::O, Element::H]);
    /// ```
    #[must_use]
    pub fn iter_elements(&self) -> Box<dyn Iterator<Item = Element> + '_> {
        Box::new(self.as_ref().iter().flat_map(|(_, component)| component.iter_elements()))
    }

    /// Returns an iterator over all elements in the molecular formula,
    /// repeating the repeating units according to their counts.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use elements_rs::Element;
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let formula: MolecularFormula = MolecularFormula::from_str("H2O").unwrap();
    ///
    /// // H2 -> H, H
    /// // O  -> O
    /// let elements: Vec<Element> = formula.iter_counted_elements().collect();
    /// assert_eq!(elements, vec![Element::H, Element::H, Element::O]);
    /// ```
    #[must_use]
    pub fn iter_counted_elements(&self) -> Box<dyn Iterator<Item = Element> + '_> {
        Box::new(self.as_ref().iter().flat_map(|(repeats, component)| {
            let n: u64 = (*repeats).into();
            (0..n).flat_map(move |_| component.iter_counted_elements())
        }))
    }
}

impl<T: NoResidualsTree> MolecularFormula<T> {
    #[must_use]
    /// Returns whether the molecular formula has repeated elements.
    ///
    /// Grouped elements are those that are represented as Element/Isotope
    /// plus a count, e.g., C6, H12, O6 in C6H12O6, and not
    /// `CCHHHHHHHHHHHOOOOOO`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let sugar: MolecularFormula = MolecularFormula::from_str("C6H12O6").unwrap();
    /// assert!(!sugar.has_repeated_elements());
    ///
    /// let sugar: MolecularFormula = MolecularFormula::from_str("CCCCCCH12O6").unwrap();
    /// assert!(sugar.has_repeated_elements());
    ///
    /// let simple: MolecularFormula = MolecularFormula::from_str("NaCl").unwrap();
    /// assert!(!simple.has_repeated_elements());
    /// ```
    pub fn has_repeated_elements(&self) -> bool {
        for (_, component) in self.as_ref() {
            if component.has_repeated_elements() {
                return true;
            }
        }
        false
    }
}
