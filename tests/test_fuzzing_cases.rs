//! Submodule for testing corner cases identified during fuzzing.
use std::str::FromStr;

use molecular_formulas::MolecularFormula;

#[test]
fn test_fuzzing_case1() {
    let formula_str = "805F712";
    assert!(MolecularFormula::from_str(formula_str).is_ok());
}

#[test]
fn test_fuzzing_case2() {
    let formula_str = "63F6BR.N";
    assert!(MolecularFormula::from_str(formula_str).is_ok());
}
