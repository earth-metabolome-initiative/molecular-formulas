//! Test submodule for `contains_residual` method of `MolecularFormula` struct.

use std::str::FromStr;

use molecular_formulas::ResidualFormula;

const FORMULAS_WITH_RESIDUALS: &[&str] = &[
    "CH4R", "C2H6R", "C3H8R", "C4H10R", "C5H12R", "C6H14R", "C7H16R", "C8H18R", "C9H20R", "C10H22R",
];
const FORMULAS_WITHOUT_RESIDUALS: &[&str] =
    &["CH4", "C2H6", "C3H8", "C4H10", "C5H12", "C6H14", "C7H16", "C8H18", "C9H20", "C10H22"];

#[test]
/// Test to check that the `contains_residuals` method works as expected
fn test_residual_detection() {
    for formula in FORMULAS_WITH_RESIDUALS {
        let formula = ResidualFormula::from_str(formula)
            .unwrap_or_else(|_| panic!("Failed to parse `{formula}`"));
        assert!(formula.contains_residuals(), "Expected {formula} to contain a residual",);
    }

    for formula in FORMULAS_WITHOUT_RESIDUALS {
        let formula = ResidualFormula::from_str(formula).unwrap();
        assert!(!formula.contains_residuals(), "Expected {formula} not to contain a residual");
    }
}
