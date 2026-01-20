//! Submodule implementing the `countable` method for the `MolecularFormula`
//! struct which returns whether the specific sub-formula component can be
//! counted.

use super::MolecularFormula;

impl MolecularFormula {
    #[must_use]
    /// Returns `true` if the formula can be counted, i.e., it is not a `Count`
    /// or `Greek` type.
    ///
    /// # Examples
    ///
    /// ```
    /// use elements_rs::Element;
    /// use molecular_formulas::{GreekLetter, MolecularFormula};
    ///
    /// // Elements are countable
    /// let h = MolecularFormula::try_from("H").unwrap();
    /// assert!(h.is_countable());
    ///
    /// // Isotopes are countable
    /// let c13 = MolecularFormula::try_from("¹³C").unwrap();
    /// assert!(c13.is_countable());
    ///
    /// // Ions are countable
    /// let na_plus = MolecularFormula::try_from("[Na]+").unwrap();
    /// assert!(na_plus.is_countable());
    ///
    /// // Complexes are countable
    /// let ph = MolecularFormula::try_from("Ph").unwrap();
    /// assert!(ph.is_countable());
    ///
    /// // Repeating units are countable
    /// let ch2 = MolecularFormula::try_from("(CH2)").unwrap();
    /// assert!(ch2.is_countable());
    ///
    /// // Mixtures are countable
    /// let mixture = MolecularFormula::try_from("H2O.D2O").unwrap();
    /// assert!(mixture.is_countable());
    ///
    /// // Sequences are countable
    /// let water = MolecularFormula::try_from("H2O").unwrap();
    /// assert!(water.is_countable());
    ///
    /// // Radicals are countable
    /// let radical = MolecularFormula::try_from("Cl•").unwrap();
    /// assert!(radical.is_countable());
    ///
    /// // Counts are NOT countable (H2 is a count of H atoms)
    /// let h2 = MolecularFormula::try_from("H2").unwrap();
    /// assert!(!h2.is_countable());
    ///
    /// // Greek letters are NOT countable
    /// let alpha: MolecularFormula = GreekLetter::Alpha.into();
    /// assert!(!alpha.is_countable());
    /// ```
    pub fn is_countable(&self) -> bool {
        match self {
            Self::Sequence(_)
            | Self::Radical(_, _)
            | Self::Mixture(_)
            | Self::Isotope(_)
            | Self::Element(_)
            | Self::Ion(_)
            | Self::Residual
            | Self::Complex(_)
            | Self::RepeatingUnit(_) => true,
            Self::Count(_, _) | Self::Greek(_) => false,
        }
    }
}
