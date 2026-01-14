//! Submodule defining index operations for the `MolecularFormula` struct.

use elements_rs::Element;

use crate::MolecularFormula;

impl MolecularFormula {
    /// Returns the element at the specified index in the molecular formula,
    /// counting repeating units according to their counts.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use elements_rs::Element;
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let formula = MolecularFormula::try_from("H2O")?;
    /// assert_eq!(formula.get_counted_element(0), Some(Element::H));
    /// assert_eq!(formula.get_counted_element(1), Some(Element::H));
    /// assert_eq!(formula.get_counted_element(2), Some(Element::O));
    /// assert_eq!(formula.get_counted_element(3), None);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_counted_element(&self, index: usize) -> Option<Element> {
        let mut count = 0;
        for element in self.iter_counted_elements() {
            if count == index {
                return Some(element);
            }
            count += 1;
        }
        None
    }

    /// Returns the element at the specified index in the molecular formula,
    /// not counting repeating units according to their counts.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use elements_rs::Element;
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let formula = MolecularFormula::try_from("H2O")?;
    /// assert_eq!(formula.get_element(0), Some(Element::H));
    /// assert_eq!(formula.get_element(1), Some(Element::O));
    /// assert_eq!(formula.get_element(2), None);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_element(&self, index: usize) -> Option<Element> {
        let mut count = 0;
        for element in self.iter_elements() {
            if count == index {
                return Some(element);
            }
            count += 1;
        }
        None
    }

    /// Returns the element at the specified index in the molecular formula,
    /// counting repeating units according to their counts, and ignoring
    /// any hydrogens (used typically for InchI parsing).
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use elements_rs::Element;
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let formula = MolecularFormula::try_from("H2O")?;
    /// assert_eq!(formula.get_counted_element_ignore_hydrogens(0), Some(Element::O));
    /// assert_eq!(formula.get_counted_element_ignore_hydrogens(1), None);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_counted_element_ignore_hydrogens(&self, index: usize) -> Option<Element> {
        let mut count = 0;
        for element in self.iter_counted_elements() {
            if element == Element::H {
                continue;
            }
            if count == index {
                return Some(element);
            }
            count += 1;
        }
        None
    }

    /// Returns the element at the specified index in the molecular formula,
    /// not counting repeating units according to their counts, and ignoring
    /// any hydrogens (used typically for InchI parsing).
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use elements_rs::Element;
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let formula = MolecularFormula::try_from("H2O")?;
    /// assert_eq!(formula.get_element_ignore_hydrogens(0), Some(Element::O));
    /// assert_eq!(formula.get_element_ignore_hydrogens(1), None);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_element_ignore_hydrogens(&self, index: usize) -> Option<Element> {
        let mut count = 0;
        for element in self.iter_elements() {
            if element == Element::H {
                continue;
            }
            if count == index {
                return Some(element);
            }
            count += 1;
        }
        None
    }
}
