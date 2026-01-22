//! Submodule for testing corner cases identified during fuzzing.
use std::str::FromStr;

use molecular_formulas::{
    Bracket, DefaultTree, LargestTree, MolecularFormula, ParseError, ResidualFormula, Terminator,
    TokenError,
};

#[test]
fn test_fuzzing_case1() {
    let formula_str = "805F712";
    assert!(MolecularFormula::<DefaultTree>::from_str(formula_str).is_ok());
}

#[test]
fn test_fuzzing_case2() {
    let formula_str = "63F6BR.N";
    assert!(MolecularFormula::<DefaultTree>::from_str(formula_str).is_err());
    assert!(ResidualFormula::from_str(formula_str).is_ok());
}

#[test]
fn test_fuzzing_case3() {
    let formula = "T.3870T";
    let parsed: MolecularFormula<DefaultTree> =
        MolecularFormula::from_str(formula).expect("Failed to parse formula");
    assert_eq!(parsed.to_string(), "T.3870T");
}

#[test]
fn test_fuzzing_case4() {
    let formula = "Cp-";
    let parsed: MolecularFormula<DefaultTree> =
        MolecularFormula::from_str(formula).expect("Failed to parse formula");
    assert_eq!(parsed.to_string(), "(C₅H₅)²⁻");
}

#[test]
fn test_fuzzing_case5() {
    let formula = "VUU[TU]";
    let parsed: MolecularFormula<DefaultTree> =
        MolecularFormula::from_str(formula).expect("Failed to parse formula");
    assert_eq!(parsed.to_string(), "VUU[TU]");
}

#[test]
fn test_fuzzing_case6() {
    let formula = "V[11N]";
    let parsed: MolecularFormula<DefaultTree> =
        MolecularFormula::from_str(formula).expect("Failed to parse formula");
    assert_eq!(parsed.to_string(), "V[¹¹N]");
}

#[test]
fn test_fuzzing_case7() {
    let formula = "V[¹¹N]";
    let parsed: MolecularFormula<DefaultTree> =
        MolecularFormula::from_str(formula).expect("Failed to parse formula");
    assert_eq!(parsed.to_string(), "V[¹¹N]");
}

#[test]
fn test_fuzzing_case8() {
    let formula = "Cp+";
    let parsed: MolecularFormula<DefaultTree> =
        MolecularFormula::from_str(formula).expect("Failed to parse formula");
    assert_eq!(parsed.to_string(), "(C₅H₅)");
}

#[test]
fn test_fuzzing_case9() {
    let formula = "Cp+Cp+";
    let parsed: MolecularFormula<DefaultTree> =
        MolecularFormula::from_str(formula).expect("Failed to parse formula");
    assert_eq!(parsed.to_string(), "(C₅H₅)(C₅H₅)");
}

#[test]
fn test_fuzzing_case10() {
    let formula2 = "Cp+Cp+";
    let formula1 = "(C₅H₅)(C₅H₅)";
    let parsed1: MolecularFormula<DefaultTree> =
        MolecularFormula::from_str(formula1).expect("Failed to parse formula");
    let parsed2: MolecularFormula<DefaultTree> =
        MolecularFormula::from_str(formula2).expect("Failed to parse formula");
    assert_eq!(parsed1, parsed2);
}

#[test]
fn test_fuzzing_case11() {
    let formula = "Bu";
    let parsed: MolecularFormula<DefaultTree> =
        MolecularFormula::from_str(formula).expect("Failed to parse formula");
    assert_eq!(parsed.to_string(), "(C₄H₉)");
}

#[test]
fn test_fuzzing_case12() {
    let formula1 = "Bu";
    let formula2 = "(C₄H₉)";
    let parsed1: MolecularFormula<DefaultTree> =
        MolecularFormula::from_str(formula1).expect("Failed to parse formula");
    let parsed2: MolecularFormula<DefaultTree> =
        MolecularFormula::from_str(formula2).expect("Failed to parse formula");
    assert_eq!(parsed1, parsed2);
}

#[test]
fn test_fuzzing_case13() {
    let formula1 = "Bu";
    let formula2 = "(C₄H₉)";
    let parsed1: ResidualFormula =
        MolecularFormula::from_str(formula1).expect("Failed to parse formula");
    let parsed2: ResidualFormula =
        MolecularFormula::from_str(formula2).expect("Failed to parse formula");
    assert_eq!(parsed1, parsed2);
}

#[test]
fn test_fuzzing_case14() {
    let formula = "BBBuBu";
    let parsed: ResidualFormula =
        MolecularFormula::from_str(formula).expect("Failed to parse formula");
    assert_eq!(parsed.to_string(), "BB(C₄H₉)(C₄H₉)");
}

#[test]
fn test_fuzzing_case15() {
    let formula1 = "BBBuBu";
    let formula2 = "BB(C₄H₉)(C₄H₉)";
    let parsed1: ResidualFormula =
        MolecularFormula::from_str(formula1).expect("Failed to parse formula");
    let parsed2: ResidualFormula =
        MolecularFormula::from_str(formula2).expect("Failed to parse formula");
    assert_eq!(parsed1, parsed2);
}

#[test]
fn test_fuzzing_case16() {
    let formula = "S.166632998S.P";
    let parsed: MolecularFormula<LargestTree> =
        MolecularFormula::from_str(formula).expect("Failed to parse formula");
    assert_eq!(parsed.to_string(), "S.166632998S.P");
}

#[test]
fn test_fuzzing_case17() {
    let formula = "S.166632998S.P";
    let parsed: ResidualFormula =
        MolecularFormula::from_str(formula).expect("Failed to parse formula");
    assert_eq!(parsed.to_string(), "S.166632998S.P");
}

#[test]
fn test_fuzzing_case18() {
    let formula = "·UUP.88";
    // We expect this to fail parsing due to the second part
    // of the mixture being invalid.
    assert_eq!(
        MolecularFormula::<LargestTree>::from_str(formula).unwrap_err(),
        ParseError::EmptySequenceNotSupportedInCurrentTree
    );
    assert!(ResidualFormula::from_str(formula).is_err());
}

#[test]
fn test_fuzzing_case19() {
    let formula = "36[·VUU]U]U";
    // We expect this to fail parsing due to unbalanced brackets.
    assert_eq!(
        MolecularFormula::<LargestTree>::from_str(formula).unwrap_err(),
        TokenError::UnexpectedTerminatorWhileParsingTokens(Terminator::CloseBracket(
            Bracket::Square
        ))
        .into()
    );
    assert_eq!(
        ResidualFormula::from_str(formula).unwrap_err(),
        TokenError::UnexpectedTerminatorWhileParsingTokens(Terminator::CloseBracket(
            Bracket::Square
        ))
        .into()
    );
}

#[test]
fn test_fuzzing_case20() {
    let formula = "H[]";
    // We expect this to fail parsing due to unbalanced brackets.
    assert_eq!(
        MolecularFormula::<LargestTree>::from_str(formula).unwrap_err(),
        TokenError::UnexpectedTerminatorWhileParsingTokens(Terminator::CloseBracket(
            Bracket::Square
        ))
        .into()
    );
    assert_eq!(
        ResidualFormula::from_str(formula).unwrap_err(),
        TokenError::UnexpectedTerminatorWhileParsingTokens(Terminator::CloseBracket(
            Bracket::Square
        ))
        .into()
    );
}
