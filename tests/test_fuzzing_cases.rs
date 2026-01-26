//! Submodule for testing corner cases identified during fuzzing.
use std::str::FromStr;

use molecular_formulas::{errors::ParserError, prelude::*};

#[test]
fn test_fuzzing_case1() {
    let formula_str = "805F712";
    let _formula: ChemicalFormula =
        ChemicalFormula::from_str(formula_str).expect("Failed to parse formula");
}

#[test]
fn test_fuzzing_case2() {
    let formula_str = "63F6BR.N";
    assert!(ChemicalFormula::<u16, i16>::from_str(formula_str).is_err());
    let _residual: ResidualFormula =
        ResidualFormula::from_str(formula_str).expect("Failed to parse formula");
}

#[test]
fn test_fuzzing_case3() {
    let formula = "T.3870T";
    let parsed: ChemicalFormula =
        ChemicalFormula::from_str(formula).expect("Failed to parse formula");
    assert_eq!(parsed.to_string(), "[³H].3870[³H]", "Parsed formula was {parsed:?}");
}

#[test]
fn test_fuzzing_case4() {
    let formula = "Cp-";
    let parsed: ChemicalFormula =
        ChemicalFormula::from_str(formula).expect("Failed to parse formula");
    assert_eq!(parsed.to_string(), "(C₅H₅)²⁻", "Parsed formula was {parsed:?}");
}

#[test]
fn test_fuzzing_case5() {
    let formula = "VUU[TU]";
    let parsed: ChemicalFormula =
        ChemicalFormula::from_str(formula).expect("Failed to parse formula");
    assert_eq!(parsed.to_string(), "VUU[[³H]U]");
}

#[test]
fn test_fuzzing_case6() {
    let formula = "V[11N]";
    let parsed: ChemicalFormula =
        ChemicalFormula::from_str(formula).expect("Failed to parse formula");
    assert_eq!(parsed.to_string(), "V[¹¹N]", "Parsed formula was {parsed:?}");
}

#[test]
fn test_fuzzing_case7() {
    let formula = "V[¹¹N]";
    let parsed: ChemicalFormula =
        ChemicalFormula::from_str(formula).expect("Failed to parse formula");
    assert_eq!(parsed.to_string(), "V[¹¹N]");
}

#[test]
fn test_fuzzing_case8() {
    let formula = "Cp+";
    let parsed: ChemicalFormula =
        ChemicalFormula::from_str(formula).expect("Failed to parse formula");
    assert_eq!(parsed.to_string(), "(C₅H₅)");
}

#[test]
fn test_fuzzing_case9() {
    let formula = "Cp+Cp+";
    let parsed: ChemicalFormula =
        ChemicalFormula::from_str(formula).expect("Failed to parse formula");
    assert_eq!(parsed.to_string(), "(C₅H₅)(C₅H₅)");
}

#[test]
fn test_fuzzing_case10() {
    let formula2 = "Cp+Cp+";
    let formula1 = "(C₅H₅)(C₅H₅)";
    let parsed1: ChemicalFormula =
        ChemicalFormula::from_str(formula1).expect("Failed to parse formula");
    let parsed2: ChemicalFormula =
        ChemicalFormula::from_str(formula2).expect("Failed to parse formula");
    assert_eq!(parsed1, parsed2);
}

#[test]
fn test_fuzzing_case11() {
    let formula = "Bu";
    let parsed: ChemicalFormula =
        ChemicalFormula::from_str(formula).expect("Failed to parse formula");
    assert_eq!(parsed.to_string(), "(C₄H₉)");
}

#[test]
fn test_fuzzing_case12() {
    let formula1 = "Bu";
    let formula2 = "(C₄H₉)";
    let parsed1: ChemicalFormula =
        ChemicalFormula::from_str(formula1).expect("Failed to parse formula");
    let parsed2: ChemicalFormula =
        ChemicalFormula::from_str(formula2).expect("Failed to parse formula");
    assert_eq!(parsed1, parsed2);
}

#[test]
fn test_fuzzing_case13() {
    let formula1 = "Bu";
    let formula2 = "(C₄H₉)";
    let parsed1: ResidualFormula =
        ResidualFormula::from_str(formula1).expect("Failed to parse formula");
    let parsed2: ResidualFormula =
        ResidualFormula::from_str(formula2).expect("Failed to parse formula");
    assert_eq!(parsed1, parsed2);
}

#[test]
fn test_fuzzing_case14() {
    let formula = "BBBuBu";
    let parsed: ResidualFormula =
        ResidualFormula::from_str(formula).expect("Failed to parse formula");
    assert_eq!(parsed.to_string(), "BB(C₄H₉)(C₄H₉)");
}

#[test]
fn test_fuzzing_case15() {
    let formula1 = "BBBuBu";
    let formula2 = "BB(C₄H₉)(C₄H₉)";
    let parsed1: ResidualFormula =
        ResidualFormula::from_str(formula1).expect("Failed to parse formula");
    let parsed2: ResidualFormula =
        ResidualFormula::from_str(formula2).expect("Failed to parse formula");
    assert_eq!(parsed1, parsed2);
}

#[test]
fn test_fuzzing_case16() {
    let formula = "S.1998S.P";
    let parsed: ChemicalFormula<u16, i16> =
        ChemicalFormula::from_str(formula).expect("Failed to parse formula");
    assert_eq!(parsed.to_string(), "S.1998S.P");
}

#[test]
fn test_fuzzing_case17() {
    let formula = "S.1998S.P";
    let parsed: ResidualFormula<u16, i16> =
        ResidualFormula::from_str(formula).expect("Failed to parse formula");
    assert_eq!(parsed.to_string(), "S.1998S.P");
}

#[test]
fn test_fuzzing_case18() {
    let formula = "·UUP.88";
    // We expect this to fail parsing due to the second part
    // of the mixture being invalid.
    assert_eq!(
        ChemicalFormula::<u16, i16>::from_str(formula).unwrap_err(),
        ParserError::EmptyMolecularTree
    );
    assert_eq!(
        ResidualFormula::<u16, i16>::from_str(formula).unwrap_err(),
        ParserError::EmptyMolecularTree
    );
}

#[test]
fn test_fuzzing_case19() {
    let formula = "36[·VUU]U]U";
    // We expect this to fail parsing due to unbalanced brackets.
    assert_eq!(
        ChemicalFormula::<u16, i16>::from_str(formula).unwrap_err(),
        ParserError::UnexpectedCharacter(']')
    );
    assert_eq!(
        ResidualFormula::<u16, i16>::from_str(formula).unwrap_err(),
        ParserError::UnexpectedCharacter(']')
    );
}

#[test]
fn test_fuzzing_case20() {
    let formula = "H[]";
    // We expect this to fail parsing due to unbalanced brackets.
    assert_eq!(
        ChemicalFormula::<u16, i16>::from_str(formula).unwrap_err(),
        ParserError::UnexpectedCharacter(']')
    );
    assert_eq!(
        ResidualFormula::<u16, i16>::from_str(formula).unwrap_err(),
        ParserError::UnexpectedCharacter(']')
    );
}

#[test]
fn test_fuzzing_case21() {
    let formula = "Se₂64";
    // We expect this to fail parsing due to invalid count.
    assert_eq!(
        ChemicalFormula::<u16, i16>::from_str(formula).unwrap_err(),
        ParserError::UnexpectedCharacter('6')
    );
}

#[test]
fn test_fuzzing_case22() {
    let formula = "Ni134₁";
    // We expect this to fail parsing due to invalid count.
    assert_eq!(
        ChemicalFormula::<u16, i16>::from_str(formula).unwrap_err(),
        ParserError::UnexpectedCharacter('₁')
    );
}

#[test]
fn test_fuzzing_case23() {
    let formula = "[²⁶⁷Hs]⁻³²⁷⁶⁷⁻";
    // We expect this to fail parsing due to invalid charge.
    assert_eq!(
        ChemicalFormula::<u16, i16>::from_str(formula).unwrap_err(),
        ParserError::UnexpectedCharacter('⁻')
    );
}

#[test]
fn test_fuzzing_case24() {
    let formula = "[²⁶⁷Hs]BuCp³²⁷⁶⁷⁻";
    // We expect this to succeed parsing and
    // create a formula with a charge of `-32767`.
    let parsed: ChemicalFormula<u16, i16> =
        ChemicalFormula::from_str(formula).expect("Failed to parse formula");

    assert!(
        (parsed.charge() + 32768.0).abs() < f64::EPSILON,
        "Parsed formula had a charge of {}",
        parsed.charge()
    );
    assert_eq!(
        parsed.to_string(),
        "[²⁶⁷Hs](C₄H₉)(C₅H₅)³²⁷⁶⁸⁻",
        "Parsed formula was {parsed:?} with a charge of {}",
        parsed.charge()
    );

    // We ensure we can round-trip the formula string.
    let reparsed: ChemicalFormula<u16, i16> =
        ChemicalFormula::from_str(&parsed.to_string()).expect("Failed to reparse formula");
    assert_eq!(parsed, reparsed, "Reparsed formula did not match original");
}

#[test]
fn test_fuzzing_case25() {
    let formula = "[⁷⁰Fe]49281¹¹⁵⁻¹¹⁵⁺12976";
    // We expect this to fail parsing due to invalid charge.
    assert_eq!(
        ChemicalFormula::<u16, i16>::from_str(formula).unwrap_err(),
        ParserError::UnexpectedCharacter('¹')
    );
}

#[test]
fn test_fuzzing_case26() {
    let formula = "[⁷⁰Fe]49281¹¹⁵⁺¹¹⁵⁺12976";
    // We expect this to fail parsing due to invalid charge.
    assert_eq!(
        ChemicalFormula::<u16, i16>::from_str(formula).unwrap_err(),
        ParserError::UnexpectedCharacter('¹')
    );
}

#[test]
fn test_fuzzing_case27() {
    let formula = "[²⁶⁷Hs]⁻³²⁷⁶⁷⁺";
    // We expect this to fail parsing due to invalid charge.
    assert_eq!(
        ChemicalFormula::<u16, i16>::from_str(formula).unwrap_err(),
        ParserError::UnexpectedCharacter('⁺')
    );
}

#[test]
fn test_fuzzing_case28() {
    let formula = "[²⁶⁷Hs]³²⁷⁶⁷⁺⁻";
    // We expect this to fail parsing due to invalid charge.
    assert_eq!(
        ChemicalFormula::<u16, i16>::from_str(formula).unwrap_err(),
        ParserError::UnexpectedCharacter('⁻')
    );
}

#[test]
fn test_fuzzing_case29() {
    let formula = "[²⁶⁷Hs]⁺³²⁷⁶⁷⁺";
    // We expect this to fail parsing due to invalid charge.
    assert_eq!(
        ChemicalFormula::<u16, i16>::from_str(formula).unwrap_err(),
        ParserError::UnexpectedCharacter('⁺')
    );
}
