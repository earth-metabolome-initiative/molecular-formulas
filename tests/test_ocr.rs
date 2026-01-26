//! Test OCR-like variants and potential parsing errors.

#[test]
fn test_ocr_variants() {
    use std::str::FromStr;

    use molecular_formulas::prelude::*;

    let f1: ChemicalFormula<u16, i16> = ChemicalFormula::from_str("C6H5(CH2)2OH").unwrap();
    let f2: ChemicalFormula<u16, i16> = ChemicalFormula::from_str("C₆H₅(CH₂)₂OH").unwrap();
    // Mixed
    let f3: ChemicalFormula<u16, i16> = ChemicalFormula::from_str("C6H₅(CH₂)2OH").unwrap();

    assert_eq!(f1, f2);
    assert_eq!(f1, f3);

    let c1: ChemicalFormula<u16, i16> = ChemicalFormula::from_str("Fe+3").unwrap();
    let c2: ChemicalFormula<u16, i16> = ChemicalFormula::from_str("Fe³⁺").unwrap();

    assert_eq!(c1, c2);
    assert_eq!(c1.charge(), 3.0);

    // Does "Fe+++" work?
    let c3: ChemicalFormula<u16, i16> = ChemicalFormula::from_str("Fe+++").expect("Fe+++ failed");
    assert_eq!(c3.charge(), 3.0);
    assert_eq!(c1, c3);

    // OCR "Crippled" Examples
    // Hydrate with unusual dots
    // '｡' is Halfwidth Ideographic Full Stop (U+FF61)
    let h1: ChemicalFormula<u16, i16> = ChemicalFormula::from_str("CuSO4.5H2O").unwrap();
    let h2: ChemicalFormula<u16, i16> = ChemicalFormula::from_str("CuSO4｡5H2O").unwrap();
    assert_eq!(h1, h2);

    // Ions with unusual dashes
    // '–' is En Dash (U+2013)
    let i1: ChemicalFormula<u16, i16> = ChemicalFormula::from_str("SO4-2").unwrap();
    // Using en-dash
    let i2: ChemicalFormula<u16, i16> = ChemicalFormula::from_str("SO4–2").unwrap();
    assert_eq!(i1, i2);
    assert_eq!(i1.charge(), -2.0);
}
