//! Test submodule for contains and contains_count methods on MolecularFormula.

use std::str::FromStr;

use molecular_formulas::prelude::*;

// -- contains: basic element containment --

#[test]
fn test_contains_simple() {
    let glucose: ChemicalFormula = ChemicalFormula::from_str("C6H12O6").unwrap();
    let water: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();

    assert!(glucose.contains(&water));
    assert!(!water.contains(&glucose));
}

#[test]
fn test_contains_self() {
    let formula: ChemicalFormula = ChemicalFormula::from_str("C6H12O6").unwrap();
    assert!(formula.contains(&formula));
}

#[test]
fn test_contains_exact_match() {
    let f1: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();
    let f2: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();
    assert!(f1.contains(&f2));
    assert!(f2.contains(&f1));
}

#[test]
fn test_contains_insufficient_count() {
    let water: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();
    let dihydrogen: ChemicalFormula = ChemicalFormula::from_str("H4").unwrap();

    assert!(!water.contains(&dihydrogen));
}

#[test]
fn test_contains_missing_element() {
    let water: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();
    let methane: ChemicalFormula = ChemicalFormula::from_str("CH4").unwrap();

    assert!(!water.contains(&methane));
}

#[test]
fn test_contains_single_element() {
    let formula: ChemicalFormula = ChemicalFormula::from_str("C6H12O6").unwrap();
    let carbon: ChemicalFormula = ChemicalFormula::from_str("C").unwrap();

    assert!(formula.contains(&carbon));
}

// -- contains: isotope containment --

#[test]
fn test_contains_isotope_match() {
    let labeled: ChemicalFormula = ChemicalFormula::from_str("[13C]2H4").unwrap();
    let single: ChemicalFormula = ChemicalFormula::from_str("[13C]").unwrap();

    assert!(labeled.contains(&single));
}

#[test]
fn test_contains_isotope_not_satisfied_by_element() {
    // Unlabeled C cannot satisfy [13C] requirement
    let unlabeled: ChemicalFormula = ChemicalFormula::from_str("C2H4").unwrap();
    let labeled: ChemicalFormula = ChemicalFormula::from_str("[13C]").unwrap();

    assert!(!unlabeled.contains(&labeled));
}

#[test]
fn test_contains_mixed_isotope_and_element() {
    // self has both [13C] and unlabeled C
    let mixed: ChemicalFormula = ChemicalFormula::from_str("[13C]C2H4").unwrap();
    let labeled: ChemicalFormula = ChemicalFormula::from_str("[13C]C").unwrap();

    assert!(mixed.contains(&labeled));
}

#[test]
fn test_contains_isotope_insufficient() {
    let labeled: ChemicalFormula = ChemicalFormula::from_str("[13C]H4").unwrap();
    let double: ChemicalFormula = ChemicalFormula::from_str("[13C]2").unwrap();

    assert!(!labeled.contains(&double));
}

// -- contains: mixtures --

#[test]
fn test_contains_mixture() {
    // CuSO4.5H2O contains H2O (10 H, 5 O from hydrate + 4 O from sulfate = 10 H, 9
    // O total)
    let hydrate: ChemicalFormula = ChemicalFormula::from_str("CuSO4.5H2O").unwrap();
    let water: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();

    assert!(hydrate.contains(&water));
}

#[test]
fn test_contains_mixture_aggregated() {
    // Two water molecules: need 4 H and 2 O
    let hydrate: ChemicalFormula = ChemicalFormula::from_str("CuSO4.5H2O").unwrap();
    let two_water: ChemicalFormula = ChemicalFormula::from_str("H4O2").unwrap();

    assert!(hydrate.contains(&two_water));
}

// -- contains: InChIFormula --

#[test]
fn test_contains_inchi() {
    let ethanol: InChIFormula = InChIFormula::from_str("C2H6O").unwrap();
    let methyl: InChIFormula = InChIFormula::from_str("CH3").unwrap();

    assert!(ethanol.contains(&methyl));
}

// -- contains_count: basic --

#[test]
fn test_contains_count_simple() {
    let glucose: ChemicalFormula = ChemicalFormula::from_str("C6H12O6").unwrap();
    let water: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();

    // C6H12O6 / H2O: H limits to 12/2=6, O limits to 6/1=6 => 6
    assert_eq!(glucose.contains_count(&water), 6);
}

#[test]
fn test_contains_count_single_element() {
    let formula: ChemicalFormula = ChemicalFormula::from_str("C6H12O6").unwrap();
    let carbon: ChemicalFormula = ChemicalFormula::from_str("C").unwrap();

    assert_eq!(formula.contains_count(&carbon), 6);
}

#[test]
fn test_contains_count_limiting_element() {
    let formula: ChemicalFormula = ChemicalFormula::from_str("C6H12O6").unwrap();
    let ch2o: ChemicalFormula = ChemicalFormula::from_str("CH2O").unwrap();

    // C: 6/1=6, H: 12/2=6, O: 6/1=6 => 6
    assert_eq!(formula.contains_count(&ch2o), 6);
}

#[test]
fn test_contains_count_limiting_by_hydrogen() {
    let formula: ChemicalFormula = ChemicalFormula::from_str("C6H10O6").unwrap();
    let ch2o: ChemicalFormula = ChemicalFormula::from_str("CH2O").unwrap();

    // C: 6/1=6, H: 10/2=5, O: 6/1=6 => 5
    assert_eq!(formula.contains_count(&ch2o), 5);
}

#[test]
fn test_contains_count_self() {
    let formula: ChemicalFormula = ChemicalFormula::from_str("C6H12O6").unwrap();
    assert_eq!(formula.contains_count(&formula), 1);
}

#[test]
fn test_contains_count_zero_when_not_contained() {
    let water: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();
    let glucose: ChemicalFormula = ChemicalFormula::from_str("C6H12O6").unwrap();

    assert_eq!(water.contains_count(&glucose), 0);
}

#[test]
fn test_contains_count_missing_element() {
    let water: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();
    let methane: ChemicalFormula = ChemicalFormula::from_str("CH4").unwrap();

    assert_eq!(water.contains_count(&methane), 0);
}

// -- contains_count: isotopes --

#[test]
fn test_contains_count_isotope() {
    let labeled: ChemicalFormula = ChemicalFormula::from_str("[13C]6H12").unwrap();
    let single: ChemicalFormula = ChemicalFormula::from_str("[13C]2").unwrap();

    assert_eq!(labeled.contains_count(&single), 3);
}

#[test]
fn test_contains_count_isotope_not_satisfied_by_element() {
    let unlabeled: ChemicalFormula = ChemicalFormula::from_str("C6H12").unwrap();
    let labeled: ChemicalFormula = ChemicalFormula::from_str("[13C]").unwrap();

    assert_eq!(unlabeled.contains_count(&labeled), 0);
}

// -- contains_count: mixtures --

#[test]
fn test_contains_count_mixture() {
    let hydrate: ChemicalFormula = ChemicalFormula::from_str("CuSO4.5H2O").unwrap();
    let water: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();

    // Hydrate has 10 H and 9 O total. H2O needs 2 H, 1 O.
    // H: 10/2=5, O: 9/1=9 => 5
    assert_eq!(hydrate.contains_count(&water), 5);
}

// -- contains_count: InChIFormula --

#[test]
fn test_contains_count_inchi() {
    let ethanol: InChIFormula = InChIFormula::from_str("C2H6O").unwrap();
    let ch3: InChIFormula = InChIFormula::from_str("CH3").unwrap();

    // C: 2/1=2, H: 6/3=2 => 2
    assert_eq!(ethanol.contains_count(&ch3), 2);
}
