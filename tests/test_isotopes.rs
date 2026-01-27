//! Tests for isotopic notation in molecular formulas.
use elements_rs::isotopes::HydrogenIsotope;
use molecular_formulas::prelude::*;
use num_traits::Zero;

#[test]
/// Test standard isotope notation like ¹³C
fn test_standard_isotopes() {
    // ¹³C
    let formula: ChemicalFormula = "¹³CH4".parse().unwrap();
    let mass = formula.isotopologue_mass();
    // 13.00335 + 4 * 1.007825
    assert!((mass - 17.03465).abs() < 1e-3);
    assert!(
        (formula.isotopologue_mass() - formula.isotopologue_mass_with_charge()).abs()
            < f64::EPSILON
    );
    assert!(formula.contains_isotope(Isotope::try_from((Element::C, 13u8)).unwrap()));
    assert_eq!(
        formula.count_of_isotope::<u32>(Isotope::try_from((Element::C, 13u8)).unwrap()),
        Some(1)
    );
    assert!(!formula.is_noble_gas_compound());
    assert!(formula.charge().is_zero());

    // We iterate over the elements in the formula and check they match expected
    // isotopes
    assert_eq!(
        formula.elements().collect::<Vec<_>>(),
        vec![Element::C, Element::H, Element::H, Element::H, Element::H]
    );

    // We check that the same formula can also be parsed by the
    // ResidualFormula parser.
    let residual_formula: ResidualFormula = "¹³CH4".parse().unwrap();
    assert_eq!(formula.to_string(), residual_formula.to_string());
}

#[test]
/// Test bracket isotope notation like [13C]
fn test_bracket_isotopes() {
    // [13C]H4
    let formula: ChemicalFormula = "[13C]H4".parse().unwrap();
    let mass = formula.isotopologue_mass();
    // 13.00335 + 4 * 1.007825
    assert!((mass - 17.03465).abs() < 1e-3, "Mass was {mass} but expected ~17.03465");
    assert!(
        (formula.isotopologue_mass() - formula.isotopologue_mass_with_charge()).abs()
            < f64::EPSILON
    );
    assert!(formula.contains_isotope(Isotope::try_from((Element::C, 13u8)).unwrap()));
    assert_eq!(
        formula.count_of_isotope::<u32>(Isotope::try_from((Element::C, 13u8)).unwrap()),
        Some(1)
    );
    assert_eq!(
        formula.elements().collect::<Vec<_>>(),
        vec![Element::C, Element::H, Element::H, Element::H, Element::H]
    );
    assert!(!formula.is_noble_gas_compound());
    assert!(formula.charge().is_zero());

    // We check that the same formula can also be parsed by the
    // ResidualFormula parser.
    let residual_formula: ResidualFormula = "[13C]H4".parse().unwrap();
    assert_eq!(formula.to_string(), residual_formula.to_string());
}

#[test]
/// Test round bracket isotope notation like (13C)
fn test_round_bracket_isotopes() {
    // (13C)H4
    let formula: ChemicalFormula = "(13C)H4".parse().unwrap();
    let mass = formula.isotopologue_mass();
    // 13.00335 + 4 * 1.007825
    assert!((mass - 17.03465).abs() < 1e-3, "Mass was {mass} but expected ~17.03465");
    assert!(
        (formula.isotopologue_mass() - formula.isotopologue_mass_with_charge()).abs()
            < f64::EPSILON
    );
    assert!(formula.contains_isotope(Isotope::try_from((Element::C, 13u8)).unwrap()));
    assert_eq!(
        formula.count_of_isotope::<u32>(Isotope::try_from((Element::C, 13u8)).unwrap()),
        Some(1)
    );
    assert_eq!(
        formula.elements().collect::<Vec<_>>(),
        vec![Element::C, Element::H, Element::H, Element::H, Element::H]
    );
    assert!(!formula.is_noble_gas_compound());

    // We check that the same formula can also be parsed by the
    // ResidualFormula parser.
    let residual_formula: ResidualFormula = "(13C)H4".parse().unwrap();
    assert_eq!(formula.to_string(), residual_formula.to_string());
}

#[test]
/// Test deuterium notation D
fn test_deuterium() {
    // D2O
    let formula: ChemicalFormula = "D2O".parse().unwrap();
    // D mass approx 2.014
    // 2 * 2.014 + 15.9949 = ~20.023
    let mass = formula.isotopologue_mass();
    assert!((mass - 20.023).abs() < 1e-3);
    assert!(
        (formula.isotopologue_mass() - formula.isotopologue_mass_with_charge()).abs()
            < f64::EPSILON
    );
    assert!(formula.contains_isotope(HydrogenIsotope::D.into()));
    assert_eq!(formula.count_of_isotope::<u32>(HydrogenIsotope::D.into()), Some(2));
    assert_eq!(formula.elements().collect::<Vec<_>>(), vec![Element::H, Element::H, Element::O]);
    assert!(!formula.is_noble_gas_compound());
    assert!(formula.charge().is_zero());

    // We check that the same formula can also be parsed by the
    // ResidualFormula parser.
    let residual_formula: ResidualFormula = "D2O".parse().unwrap();
    assert_eq!(formula.to_string(), residual_formula.to_string());
}

#[test]
/// Test tritium notation T
fn test_tritium() {
    // T2
    let formula: ChemicalFormula = "T2".parse().unwrap();
    // T mass approx 3.016
    let mass = formula.isotopologue_mass();
    assert!((mass - 6.032).abs() < 1e-3);
    assert!(
        (formula.isotopologue_mass() - formula.isotopologue_mass_with_charge()).abs()
            < f64::EPSILON
    );
    assert!(formula.contains_isotope(HydrogenIsotope::T.into()));
    assert_eq!(formula.count_of_isotope::<u32>(HydrogenIsotope::T.into()), Some(2));
    assert_eq!(formula.elements().collect::<Vec<_>>(), vec![Element::H, Element::H]);
    assert!(!formula.is_noble_gas_compound());
    assert!(formula.charge().is_zero());

    // We check that the same formula can also be parsed by the
    // ResidualFormula parser.
    let residual_formula: ResidualFormula = "T2".parse().unwrap();
    assert_eq!(formula.to_string(), residual_formula.to_string());
}

#[test]
/// Test other bracket isotope notation like [18O]ù
fn test_other_bracket_isotopes() {
    // [18O]
    let formula: ChemicalFormula = "H2[18O]".parse().unwrap();
    // 2*1.008 + 17.999 = ~20.015
    let mass = formula.isotopologue_mass();
    assert!((mass - 20.015).abs() < 1e-3);
    assert!(formula.contains_isotope(Isotope::try_from((Element::O, 18u8)).unwrap()));
    assert_eq!(
        formula.count_of_isotope::<u32>(Isotope::try_from((Element::O, 18u8)).unwrap()),
        Some(1)
    );
    assert!(
        (formula.isotopologue_mass() - formula.isotopologue_mass_with_charge()).abs()
            < f64::EPSILON
    );
    assert_eq!(formula.elements().collect::<Vec<_>>(), vec![Element::H, Element::H, Element::O]);
    assert!(!formula.is_noble_gas_compound());

    // We check that the same formula can also be parsed by the
    // ResidualFormula parser.
    let residual_formula: ResidualFormula = "H2[18O]".parse().unwrap();
    assert_eq!(formula.to_string(), residual_formula.to_string());
}

#[test]
/// Test for the `C[13]` notation
fn test_c13_notation() {
    // C[13]H4
    let formula: ChemicalFormula = "C[13]H4".parse().unwrap();
    let mass = formula.isotopologue_mass();
    // 13.00335 + 4 * 1.007825
    assert!((mass - 17.03465).abs() < 1e-3);
    assert!(formula.contains_isotope(Isotope::try_from((Element::C, 13u8)).unwrap()));
    assert_eq!(
        formula.count_of_isotope::<u32>(Isotope::try_from((Element::C, 13u8)).unwrap()),
        Some(1)
    );
    assert_eq!(
        formula.elements().collect::<Vec<_>>(),
        vec![Element::C, Element::H, Element::H, Element::H, Element::H]
    );
    assert!(!formula.is_noble_gas_compound());
    // We check that the same formula can also be parsed by the
    // ResidualFormula parser.
    let residual_formula: ResidualFormula = "C[13]H4".parse().unwrap();
    assert_eq!(formula.to_string(), residual_formula.to_string());
}

#[test]
fn test_helium_isotopes() {
    // 3He4He
    let formula: ChemicalFormula = "³He⁴He".parse().unwrap();
    let mass = formula.isotopologue_mass();
    // 3.016 + 4.0026 = ~7.0186
    assert!((mass - 7.0186).abs() < 1e-4, "Mass was {mass} but expected ~7.0186");
    assert!(formula.contains_isotope(Isotope::try_from((Element::He, 3u8)).unwrap()));
    assert!(formula.contains_isotope(Isotope::try_from((Element::He, 4u8)).unwrap()));
    assert_eq!(
        formula.count_of_isotope::<u32>(Isotope::try_from((Element::He, 3u8)).unwrap()),
        Some(1)
    );
    assert_eq!(
        formula.count_of_isotope::<u32>(Isotope::try_from((Element::He, 4u8)).unwrap()),
        Some(1)
    );
    assert_eq!(formula.elements().collect::<Vec<_>>(), vec![Element::He, Element::He]);
    assert!(formula.is_noble_gas_compound());

    // We check that the same formula can also be parsed by the
    // ResidualFormula parser.
    let residual_formula: ResidualFormula = "³He⁴He".parse().unwrap();
    assert_eq!(formula.to_string(), residual_formula.to_string());
}

#[test]
/// Test illegal isotope numbers
fn test_illegal_isotope_numbers() {
    // ⁵⁰⁰H
    assert!("⁵⁰⁰H".parse::<ChemicalFormula>().is_err());

    // [500H]
    assert!("[500H]".parse::<ChemicalFormula>().is_err());

    // (500H)
    assert!("(500H)".parse::<ChemicalFormula>().is_err());

    // H[500]
    assert!("H[500]".parse::<ChemicalFormula>().is_err());
}
