//! Submodule implmenting the `is_homonuclear` method for the `MolecularFormula`
//! struct, which returns whether the formula is composed solely of one
//! element.

use elements_rs::{Element, ElementVariant};

impl crate::MolecularFormula {
    fn inner_is_homonuclear(
        &self,
        mut other: Option<elements_rs::Element>,
    ) -> Result<(bool, Element), crate::errors::Error> {
        Ok(match self {
            Self::Element(element) => {
                other.map_or((true, *element), |other| (*element == other, other))
            }
            Self::Isotope(isotope) => {
                other.map_or((true, isotope.element()), |other| (isotope.element() == other, other))
            }
            Self::Residual => {
                return Err(crate::errors::Error::InvalidOperationForResidual);
            }
            Self::Ion(ion) => ion.entry.inner_is_homonuclear(other)?,
            Self::Count(formula, _) => formula.inner_is_homonuclear(other)?,
            Self::Complex(formula) | Self::RepeatingUnit(formula) | Self::Radical(formula, _) => {
                formula.inner_is_homonuclear(other)?
            }
            Self::Mixture(_) | Self::Greek(_) => {
                unreachable!(
                    "Mixture of greek letters should not be passed to inner_is_homonuclear"
                );
            }
            Self::Sequence(formulas) => {
                let mut homonuclear = true;
                for formula in formulas {
                    if matches!(formula, Self::Greek(_)) {
                        continue;
                    }

                    let (this_homonuclear, element) = formula.inner_is_homonuclear(other)?;
                    if other.is_none() {
                        other = Some(element);
                    }
                    homonuclear &= this_homonuclear;
                }
                (homonuclear, other.unwrap())
            }
        })
    }

    /// Returns whether the formula is homonuclear, such as `H2`, `O2`,
    /// `N2`, `H2+`, `O2-`, etc. A mixture is considered homonuclear if all
    /// of the sub-formulas are homonuclear.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let formula = MolecularFormula::from_str("H2").unwrap();
    /// assert_eq!(formula.is_homonuclear().unwrap(), true);
    ///
    /// let formula = MolecularFormula::from_str("H2O").unwrap();
    /// assert_eq!(formula.is_homonuclear().unwrap(), false);
    ///
    /// let formula = MolecularFormula::from_str("H2O2").unwrap();
    /// assert_eq!(formula.is_homonuclear().unwrap(), false);
    /// ```
    ///
    /// # Errors
    ///
    /// * If the formula contains a non-element, such as a `Residual`.
    pub fn is_homonuclear(&self) -> Result<bool, crate::errors::Error> {
        Ok(match self {
            Self::Element(_)
            | Self::Isotope(_)
            | Self::Ion(_)
            | Self::Count(_, _)
            | Self::Complex(_)
            | Self::RepeatingUnit(_)
            | Self::Radical(_, _)
            | Self::Sequence(_) => self.inner_is_homonuclear(None)?.0,
            Self::Mixture(formulas) => {
                let mut common_element = None;
                for (_, formula) in formulas {
                    let (homonuclear, element) = formula.inner_is_homonuclear(None)?;
                    if !homonuclear {
                        return Ok(false);
                    }
                    if let Some(ce) = common_element {
                        if ce != element {
                            return Ok(false);
                        }
                    } else {
                        common_element = Some(element);
                    }
                }
                true
            }
            Self::Greek(_) => {
                unreachable!("Illegal state: Formula cannot have as root a Greek letter");
            }
            Self::Residual => {
                return Err(crate::errors::Error::InvalidOperationForResidual);
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::MolecularFormula;

    #[test]
    fn test_homonuclear_branches() {
        assert!(MolecularFormula::from_str("H2").unwrap().is_homonuclear().unwrap());

        // Ion
        assert!(MolecularFormula::from_str("[H2]+").unwrap().is_homonuclear().unwrap());

        // Isotope: H vs D. They are same element (Hydrogen), so should be homonuclear.
        // H = Element::H, D = Isotope(H). inner_is_homonuclear compares Elements.
        // H has element H. D has element H. They match.
        // However, standard parser might parse HD as H D (sequence).
        let hd = MolecularFormula::from_str("HD").unwrap(); // H and D
        assert!(hd.is_homonuclear().unwrap());

        // Mixture of homonuclear same element?
        // H2.H2 -> Homonuclear H.
        let mix_same = MolecularFormula::from_str("H2.H2").unwrap();
        assert!(mix_same.is_homonuclear().unwrap());

        // Mixture different
        let mix_diff = MolecularFormula::from_str("H2.O2").unwrap();
        assert!(!mix_diff.is_homonuclear().unwrap());

        // Residual error
        assert!(MolecularFormula::from_str("R").unwrap().is_homonuclear().is_err());
    }
}
