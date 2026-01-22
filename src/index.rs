//! Submodule defining index operations for the `MolecularFormula` struct.

use elements_rs::Element;

use crate::{MolecularFormula, Tree};

impl<T: Tree> MolecularFormula<T> {
    /// Returns the element at the specified index in the molecular formula,
    /// counting repeating units according to their counts.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use elements_rs::Element;
    /// use molecular_formulas::{DefaultTree, MolecularFormula};
    ///
    /// let formula: MolecularFormula<DefaultTree> = MolecularFormula::try_from("H2O")?;
    /// assert_eq!(formula.get_counted_element(0), Some(Element::H));
    /// assert_eq!(formula.get_counted_element(1), Some(Element::H));
    /// assert_eq!(formula.get_counted_element(2), Some(Element::O));
    /// assert_eq!(formula.get_counted_element(3), None);
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn get_counted_element(&self, index: usize) -> Option<Element> {
        let mut current_idx = index as u64;
        for (repeats, component) in self.as_ref() {
            let n: u64 = (*repeats).into();
            match component.get_counted_element_or_size(current_idx) {
                Ok(element) => return Some(element),
                Err(size) => {
                    let total_opt = size.checked_mul(n);
                    if let Some(total) = total_opt {
                        if current_idx < total {
                            let inner_idx = current_idx % size;
                            // We use get_counted_element here, which uses
                            // get_counted_element_or_size internally
                            // and since inner_idx < size, it will return Ok(element).
                            return component.get_counted_element(inner_idx);
                        }
                        current_idx -= total;
                    } else {
                        // Overflow. total > u64::MAX > current_idx.
                        let inner_idx = current_idx % size;
                        return component.get_counted_element(inner_idx);
                    }
                }
            }
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
    /// use molecular_formulas::{DefaultTree, MolecularFormula};
    ///
    /// let formula: MolecularFormula<DefaultTree> = MolecularFormula::try_from("H2O")?;
    /// assert_eq!(formula.get_element(0), Some(Element::H));
    /// assert_eq!(formula.get_element(1), Some(Element::O));
    /// assert_eq!(formula.get_element(2), None);
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn get_element(&self, index: usize) -> Option<Element> {
        self.iter_elements().enumerate().find_map(
            |(i, element)| {
                if i == index { Some(element) } else { None }
            },
        )
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
    /// use molecular_formulas::{DefaultTree, MolecularFormula};
    ///
    /// let formula: MolecularFormula<DefaultTree> = MolecularFormula::try_from("H2O")?;
    /// assert_eq!(formula.get_counted_element_ignore_hydrogens(0), Some(Element::O));
    /// assert_eq!(formula.get_counted_element_ignore_hydrogens(1), None);
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn get_counted_element_ignore_hydrogens(&self, index: usize) -> Option<Element> {
        self.iter_counted_elements().filter(|&e| e != Element::H).enumerate().find_map(
            |(i, element)| {
                if i == index { Some(element) } else { None }
            },
        )
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
    /// use molecular_formulas::{DefaultTree, MolecularFormula};
    ///
    /// let formula: MolecularFormula<DefaultTree> = MolecularFormula::try_from("H2O")?;
    /// assert_eq!(formula.get_element_ignore_hydrogens(0), Some(Element::O));
    /// assert_eq!(formula.get_element_ignore_hydrogens(1), None);
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn get_element_ignore_hydrogens(&self, index: usize) -> Option<Element> {
        self.iter_elements()
            .filter(|&e| e != Element::H)
            .enumerate()
            .find_map(|(i, element)| if i == index { Some(element) } else { None })
    }
}
