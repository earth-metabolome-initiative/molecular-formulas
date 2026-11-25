//! Test submodule for contains and contains_count methods on MolecularFormula.

use std::str::FromStr;

use molecular_formulas::{
    errors::{CountError, NumericError},
    prelude::*,
};

#[test]
fn containment_exceeds_stored_coefficient() {
    let water: ChemicalFormula<u8, i16> = "250H2O".parse().unwrap();
    let hydrogen = "H".parse().unwrap();
    assert_eq!(water.contains(&hydrogen), Ok(true));
    assert_eq!(water.contains_count(&hydrogen), Ok(500));
}

#[test]
fn containment_divides_wide_totals_before_converting_the_result() {
    let sample: ChemicalFormula<u32, i32> = "65536((H65536)65536)65536".parse().unwrap();
    let hydrogen = "H".parse().unwrap();
    assert_eq!(sample.contains(&hydrogen), Ok(true));
    assert_eq!(
        sample.contains_count(&hydrogen),
        Err(CountError::Numeric(NumericError::PositiveOverflow))
    );
    assert_eq!(sample.contains_count(&sample), Ok(1));
}

#[test]
fn containment_reports_atom_totals_beyond_the_accumulator() {
    let oversized: ChemicalFormula<u32, i32> =
        "65536((((((H65536)65536)65536)65536)65536)65536)65536".parse().unwrap();
    let hydrogen = "H".parse().unwrap();
    let overflow = CountError::Numeric(NumericError::PositiveOverflow);
    assert_eq!(oversized.contains(&hydrogen), Err(overflow));
    assert_eq!(hydrogen.contains_count(&oversized), Err(overflow));
}

#[test]
fn labeled_carbon_cannot_supply_unlabeled_carbon() {
    let labeled: ChemicalFormula<u8, i16> = "250[13C]2".parse().unwrap();
    let carbon = "C".parse().unwrap();
    let carbon_13 = "[13C]".parse().unwrap();
    assert_eq!(labeled.contains_count(&carbon), Ok(0));
    assert_eq!(labeled.contains_count(&carbon_13), Ok(500));
}

// -- contains: basic element containment --

#[test]
fn test_contains_simple() {
    let glucose: ChemicalFormula = ChemicalFormula::from_str("C6H12O6").unwrap();
    let water: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();

    assert_eq!(glucose.contains(&water), Ok(true));
    assert_eq!(water.contains(&glucose), Ok(false));
}

#[test]
fn test_contains_self() {
    let formula: ChemicalFormula = ChemicalFormula::from_str("C6H12O6").unwrap();
    assert_eq!(formula.contains(&formula), Ok(true));
}

#[test]
fn test_contains_exact_match() {
    let f1: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();
    let f2: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();
    assert_eq!(f1.contains(&f2), Ok(true));
    assert_eq!(f2.contains(&f1), Ok(true));
}

#[test]
fn test_contains_insufficient_count() {
    let water: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();
    let dihydrogen: ChemicalFormula = ChemicalFormula::from_str("H4").unwrap();

    assert_eq!(water.contains(&dihydrogen), Ok(false));
}

#[test]
fn test_contains_missing_element() {
    let water: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();
    let methane: ChemicalFormula = ChemicalFormula::from_str("CH4").unwrap();

    assert_eq!(water.contains(&methane), Ok(false));
}

#[test]
fn test_contains_single_element() {
    let formula: ChemicalFormula = ChemicalFormula::from_str("C6H12O6").unwrap();
    let carbon: ChemicalFormula = ChemicalFormula::from_str("C").unwrap();

    assert_eq!(formula.contains(&carbon), Ok(true));
}

// -- contains: isotope containment --

#[test]
fn test_contains_isotope_match() {
    let labeled: ChemicalFormula = ChemicalFormula::from_str("[13C]2H4").unwrap();
    let single: ChemicalFormula = ChemicalFormula::from_str("[13C]").unwrap();

    assert_eq!(labeled.contains(&single), Ok(true));
}

#[test]
fn test_contains_isotope_not_satisfied_by_element() {
    // Unlabeled C cannot satisfy [13C] requirement
    let unlabeled: ChemicalFormula = ChemicalFormula::from_str("C2H4").unwrap();
    let labeled: ChemicalFormula = ChemicalFormula::from_str("[13C]").unwrap();

    assert_eq!(unlabeled.contains(&labeled), Ok(false));
}

#[test]
fn test_contains_mixed_isotope_and_element() {
    // self has both [13C] and unlabeled C
    let mixed: ChemicalFormula = ChemicalFormula::from_str("[13C]C2H4").unwrap();
    let labeled: ChemicalFormula = ChemicalFormula::from_str("[13C]C").unwrap();

    assert_eq!(mixed.contains(&labeled), Ok(true));
}

#[test]
fn test_contains_isotope_insufficient() {
    let labeled: ChemicalFormula = ChemicalFormula::from_str("[13C]H4").unwrap();
    let double: ChemicalFormula = ChemicalFormula::from_str("[13C]2").unwrap();

    assert_eq!(labeled.contains(&double), Ok(false));
}

// -- contains: mixtures --

#[test]
fn test_contains_mixture() {
    // CuSO4.5H2O contains H2O (10 H, 5 O from hydrate + 4 O from sulfate = 10
    // H, 9 O total)
    let hydrate: ChemicalFormula = ChemicalFormula::from_str("CuSO4.5H2O").unwrap();
    let water: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();

    assert_eq!(hydrate.contains(&water), Ok(true));
}

#[test]
fn test_contains_mixture_aggregated() {
    // Two water molecules: need 4 H and 2 O
    let hydrate: ChemicalFormula = ChemicalFormula::from_str("CuSO4.5H2O").unwrap();
    let two_water: ChemicalFormula = ChemicalFormula::from_str("H4O2").unwrap();

    assert_eq!(hydrate.contains(&two_water), Ok(true));
}

// -- contains: InChIFormula --

#[test]
fn test_contains_inchi() {
    let ethanol: InChIFormula = InChIFormula::from_str("C2H6O").unwrap();
    let methyl: InChIFormula = InChIFormula::from_str("CH3").unwrap();

    assert_eq!(ethanol.contains(&methyl), Ok(true));
}

// -- contains_count: basic --

#[test]
fn test_contains_count_simple() {
    let glucose: ChemicalFormula = ChemicalFormula::from_str("C6H12O6").unwrap();
    let water: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();

    // C6H12O6 / H2O: H limits to 12/2=6, O limits to 6/1=6 => 6
    assert_eq!(glucose.contains_count(&water), Ok(6));
}

#[test]
fn test_contains_count_single_element() {
    let formula: ChemicalFormula = ChemicalFormula::from_str("C6H12O6").unwrap();
    let carbon: ChemicalFormula = ChemicalFormula::from_str("C").unwrap();

    assert_eq!(formula.contains_count(&carbon), Ok(6));
}

#[test]
fn test_contains_count_limiting_element() {
    let formula: ChemicalFormula = ChemicalFormula::from_str("C6H12O6").unwrap();
    let ch2o: ChemicalFormula = ChemicalFormula::from_str("CH2O").unwrap();

    // C: 6/1=6, H: 12/2=6, O: 6/1=6 => 6
    assert_eq!(formula.contains_count(&ch2o), Ok(6));
}

#[test]
fn test_contains_count_limiting_by_hydrogen() {
    let formula: ChemicalFormula = ChemicalFormula::from_str("C6H10O6").unwrap();
    let ch2o: ChemicalFormula = ChemicalFormula::from_str("CH2O").unwrap();

    // C: 6/1=6, H: 10/2=5, O: 6/1=6 => 5
    assert_eq!(formula.contains_count(&ch2o), Ok(5));
}

#[test]
fn test_contains_count_self() {
    let formula: ChemicalFormula = ChemicalFormula::from_str("C6H12O6").unwrap();
    assert_eq!(formula.contains_count(&formula), Ok(1));
}

#[test]
fn test_contains_count_zero_when_not_contained() {
    let water: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();
    let glucose: ChemicalFormula = ChemicalFormula::from_str("C6H12O6").unwrap();

    assert_eq!(water.contains_count(&glucose), Ok(0));
}

#[test]
fn test_contains_count_missing_element() {
    let water: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();
    let methane: ChemicalFormula = ChemicalFormula::from_str("CH4").unwrap();

    assert_eq!(water.contains_count(&methane), Ok(0));
}

// -- contains_count: isotopes --

#[test]
fn test_contains_count_isotope() {
    let labeled: ChemicalFormula = ChemicalFormula::from_str("[13C]6H12").unwrap();
    let single: ChemicalFormula = ChemicalFormula::from_str("[13C]2").unwrap();

    assert_eq!(labeled.contains_count(&single), Ok(3));
}

#[test]
fn test_contains_count_isotope_not_satisfied_by_element() {
    let unlabeled: ChemicalFormula = ChemicalFormula::from_str("C6H12").unwrap();
    let labeled: ChemicalFormula = ChemicalFormula::from_str("[13C]").unwrap();

    assert_eq!(unlabeled.contains_count(&labeled), Ok(0));
}

// -- contains_count: mixtures --

#[test]
fn test_contains_count_mixture() {
    let hydrate: ChemicalFormula = ChemicalFormula::from_str("CuSO4.5H2O").unwrap();
    let water: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();

    // Hydrate has 10 H and 9 O total. H2O needs 2 H, 1 O.
    // H: 10/2=5, O: 9/1=9 => 5
    assert_eq!(hydrate.contains_count(&water), Ok(5));
}

// -- contains_count: InChIFormula --

#[test]
fn test_contains_count_inchi() {
    let ethanol: InChIFormula = InChIFormula::from_str("C2H6O").unwrap();
    let ch3: InChIFormula = InChIFormula::from_str("CH3").unwrap();

    // C: 2/1=2, H: 6/3=2 => 2
    assert_eq!(ethanol.contains_count(&ch3), Ok(2));
}
