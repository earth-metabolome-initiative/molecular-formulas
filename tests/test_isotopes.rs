//! Tests for isotopic notation in molecular formulas.
use elements_rs::isotopes::HydrogenIsotope;
use molecular_formulas::MolecularFormula;

#[test]
/// Test standard isotope notation like ¹³C
fn test_standard_isotopes() {
    // ¹³C
    let formula: MolecularFormula = "¹³CH4".parse().unwrap();
    let mass = formula.isotopologue_mass_without_charge();
    // 13.00335 + 4 * 1.007825
    assert!((mass - 17.03465).abs() < 1e-3);
}

#[test]
/// Test bracket isotope notation like [13C]
fn test_bracket_isotopes() {
    // [13C]H4
    let formula: MolecularFormula = "[13C]H4".parse().unwrap();
    let mass = formula.isotopologue_mass_without_charge();
    // 13.00335 + 4 * 1.007825
    assert!((mass - 17.03465).abs() < 1e-3, "Mass was {mass} but expected ~17.03465");
}

#[test]
/// Test round bracket isotope notation like (13C)
fn test_round_bracket_isotopes() {
    // (13C)H4
    let formula: MolecularFormula = "(13C)H4".parse().unwrap();
    let mass = formula.isotopologue_mass_without_charge();
    // 13.00335 + 4 * 1.007825
    assert!((mass - 17.03465).abs() < 1e-3, "Mass was {mass} but expected ~17.03465");
}

#[test]
/// Test deuterium notation D
fn test_deuterium() {
    // D2O
    let formula: MolecularFormula = "D2O".parse().unwrap();
    // D mass approx 2.014
    // 2 * 2.014 + 15.9949 = ~20.023
    let mass = formula.isotopologue_mass_without_charge();
    assert!((mass - 20.023).abs() < 1e-3);

    assert!(formula.contains_isotope(HydrogenIsotope::D));
}

#[test]
/// Test tritium notation T
fn test_tritium() {
    // T2
    let formula: MolecularFormula = "T2".parse().unwrap();
    // T mass approx 3.016
    let mass = formula.isotopologue_mass_without_charge();
    assert!((mass - 6.032).abs() < 1e-3);
}

#[test]
/// Test other bracket isotope notation like [18O]ù
fn test_other_bracket_isotopes() {
    // [18O]
    let formula: MolecularFormula = "H2[18O]".parse().unwrap();
    // 2*1.008 + 17.999 = ~20.015
    let mass = formula.isotopologue_mass_without_charge();
    assert!((mass - 20.015).abs() < 1e-3);
}
