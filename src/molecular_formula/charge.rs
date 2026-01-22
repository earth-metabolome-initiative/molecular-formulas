use super::MolecularFormula;
use crate::NoResidualsTree;
impl<T: NoResidualsTree> MolecularFormula<T> {
    /// Returns the overall charge of the molecular formula.
    /// The charge is calculated by summing the charges of all components.
    ///
    /// # Examples
    ///
    /// ```
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let formula: MolecularFormula = "H2O".parse().unwrap();
    /// assert_eq!(formula.charge(), 0.0);
    ///
    /// let formula: MolecularFormula = "[Na]+".parse().unwrap();
    /// assert_eq!(formula.charge(), 1.0);
    ///
    /// let formula: MolecularFormula = "[SO4]-2".parse().unwrap();
    /// assert_eq!(formula.charge(), -2.0);
    /// ```
    ///
    /// # Errors
    ///
    /// * If the `MolecularFormula` contains Residual.
    #[must_use]
    #[inline]
    pub fn charge(&self) -> f64 {
        let mut total_charge: f64 = 0.0;

        for (repeats, component) in &self.mixtures {
            let n: f64 = (*repeats).into();
            let component_charge = component.total_charge();
            total_charge += component_charge * n;
        }

        total_charge
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

        let na_plus: MolecularFormula = MolecularFormula::from_str("2[Na]+").unwrap();
        let so4_2minus: MolecularFormula = MolecularFormula::from_str("[SO4]-2").unwrap();

        // Manual mixture construction since string parsing of charged mixtures
        // might be tricky with dots and charges.
        let mixture = na_plus.mix(so4_2minus).unwrap();

        assert!((mixture.charge() - 0.0).abs() < f64::EPSILON);
    }
}
