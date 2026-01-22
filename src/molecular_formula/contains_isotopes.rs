//! Submodule providing the `contains_isotope` method for the
//! `MolecularFormula` struct
use elements_rs::Isotope;

use crate::Tree;

impl<T: Tree> super::MolecularFormula<T> {
    /// Checks if the molecular formula is isotopically defined.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let water: MolecularFormula = MolecularFormula::try_from("H2O")?;
    /// assert!(!water.contains_isotopes());
    ///
    /// let formula: MolecularFormula = MolecularFormula::try_from("³H2O")?;
    /// assert!(formula.contains_isotopes());
    ///
    /// let formula: MolecularFormula = MolecularFormula::try_from("D2O")?;
    /// assert!(formula.contains_isotopes());
    ///
    /// let formula: MolecularFormula = MolecularFormula::try_from("T2O")?;
    /// assert!(formula.contains_isotopes());
    ///
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    #[inline]
    pub fn contains_isotopes(&self) -> bool {
        self.as_ref().iter().any(|(_, component)| component.contains_isotopes())
    }

    /// Returns whether the molecular formula contains a specific isotope.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use elements_rs::isotopes::{HeliumIsotope, HydrogenIsotope};
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let formula: MolecularFormula = MolecularFormula::try_from("D2O.³He")?;
    /// assert!(formula.contains_isotope(HydrogenIsotope::D));
    /// assert!(formula.contains_isotope(HeliumIsotope::He3));
    /// assert!(!formula.contains_isotope(HydrogenIsotope::H1));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn contains_isotope(&self, isotope: impl Into<Isotope>) -> bool {
        let isotope = isotope.into();
        self.as_ref().iter().any(|(_, component)| component.contains_isotope(isotope))
    }

    /// Iterates over all isotopic elements in the molecular formula.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use elements_rs::{
    ///     Element, Isotope,
    ///     isotopes::{HeliumIsotope, HydrogenIsotope},
    /// };
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let formula: MolecularFormula = MolecularFormula::try_from("D2O.³He")?;
    /// let isotopes: Vec<Isotope> = formula.iter_isotopes().collect();
    /// assert_eq!(isotopes, vec![HydrogenIsotope::D.into(), HeliumIsotope::He3.into()]);
    ///
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn iter_isotopes(&self) -> impl Iterator<Item = Isotope> + '_ {
        self.as_ref().iter().flat_map(|(_, component)| component.iter_isotopes())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{DefaultTree, MolecularFormula};

    #[test]
    fn test_contains_isotope_branches() {
        assert!(!MolecularFormula::<DefaultTree>::from_str("H").unwrap().contains_isotopes());
        assert!(MolecularFormula::<DefaultTree>::from_str("²H").unwrap().contains_isotopes());

        // Mixture with isotope
        let mix: MolecularFormula = MolecularFormula::from_str("D2O.H2O").unwrap(); // D is isotope of H
        assert!(mix.contains_isotopes());

        let mix_clean: MolecularFormula = MolecularFormula::from_str("H2O.NaCl").unwrap();
        assert!(!mix_clean.contains_isotopes());
    }
}
