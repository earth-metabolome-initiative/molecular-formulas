use super::MolecularFormula;

impl MolecularFormula {
    /// Returns the overall charge of the molecular formula.
    /// The charge is calculated by summing the charges of all components.
    ///
    /// # Examples
    ///
    /// ```
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let formula: MolecularFormula = "H2O".parse().unwrap();
    /// assert_eq!(formula.charge().unwrap(), 0);
    ///
    /// let formula: MolecularFormula = "[Na]+".parse().unwrap();
    /// assert_eq!(formula.charge().unwrap(), 1);
    ///
    /// let formula: MolecularFormula = "[SO4]-2".parse().unwrap();
    /// assert_eq!(formula.charge().unwrap(), -2);
    /// ```
    ///
    /// # Errors
    ///
    /// * If the `MolecularFormula` contains Residual.
    pub fn charge(&self) -> Result<i16, crate::errors::Error> {
        Ok(match self {
            Self::Ion(ion) => ion.charge,
            Self::Element(_) | Self::Isotope(_) | Self::Greek(_) => 0,
            Self::Count(formula, count) => {
                formula.charge()?
                    * i16::try_from(*count).map_err(|_| crate::errors::Error::InvalidNumber)?
            }
            Self::Sequence(formulas) => {
                let mut charge = 0;
                for formula in formulas {
                    charge += formula.charge()?;
                }
                charge
            }
            Self::Mixture(formulas) => {
                let mut charge = 0;
                for (repeats, formula) in formulas {
                    charge += i16::try_from(*repeats)
                        .map_err(|_| crate::errors::Error::InvalidNumber)?
                        * formula.charge()?;
                }
                charge
            }
            Self::Radical(formula, _) | Self::RepeatingUnit(formula) | Self::Complex(formula) => {
                formula.charge()?
            }
            Self::Residual => return Err(crate::errors::Error::InvalidOperationForResidual),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::MolecularFormula;

    #[test]
    fn test_mixture_charge() {
        // Mixture of Na+ and Cl-
        // Note: Parser support for charged mixtures might be specific.
        // Let's rely on constructing manual mixtures if parse falls short,
        // or simple cases like separate ions if supported.
        // Assuming dot separator works for mixtures of ions if implemented.
        // If not, we test components.

        let na_plus = MolecularFormula::from_str("[Na]+").unwrap();
        let so4_2minus = MolecularFormula::from_str("[SO4]-2").unwrap();

        // Manual mixture construction since string parsing of charged mixtures
        // might be tricky with dots and charges.
        let mixture = MolecularFormula::Mixture(vec![(2, na_plus), (1, so4_2minus)]);

        assert_eq!(mixture.charge().unwrap(), 0);
    }

    #[test]
    fn test_residual_error() {
        let residual = MolecularFormula::from_str("R").unwrap(); // R is residual
        assert!(residual.charge().is_err());

        let complex_residual = MolecularFormula::from_str("PhR").unwrap(); // Phenyl + Residual
        assert!(complex_residual.charge().is_err());
    }
}
