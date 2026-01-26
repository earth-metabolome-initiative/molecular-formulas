//! Test submodule to ensure all methods work for the largest InChI string.

use core::str::FromStr;

use elements_rs::isotopes::{HeliumIsotope, HydrogenIsotope};
use molecular_formulas::{errors::ParserError, prelude::*};
use num_traits::Zero;
use strum::IntoEnumIterator;

fn test_all_molecular_trait_method1<M: MolecularFormula>(m: &M) {
    assert_eq!(m.number_of_mixtures(), 76);
    for element in m.elements() {
        let count = m.count_of_element::<M::Count>(element).unwrap();
        assert!(!count.is_zero());
        assert!(m.contains_element(element));
    }
    assert!(!m.contains_isotopes());
    assert!(m.contains_elements());
    assert!(
        (m.isotopologue_mass() - 6928.64091608618).abs() < f64::EPSILON,
        "Found mass: {}",
        m.isotopologue_mass()
    );
    assert!(!m.is_noble_gas_compound());
    assert!(m.is_hill_sorted());

    for isotope in HydrogenIsotope::iter() {
        assert!(!m.contains_isotope(isotope.into()));
        assert_eq!(m.count_of_isotope::<M::Count>(isotope.into()), Some(M::Count::zero()));
    }
}

fn test_all_charged_molecular_trait_method1<M: ChargedMolecularFormula>(m: &M) {
    assert!(m.charge().is_zero());
    assert!((m.isotopologue_mass_with_charge() - 6928.64091608618).abs() < f64::EPSILON);
    assert!((m.isotopologue_mass_with_charge() - m.isotopologue_mass()).abs() < f64::EPSILON);
    assert!((m.molar_mass() - 6935.603).abs() < f64::EPSILON, "Found mass: {}", m.molar_mass());
}

fn test_all_molecular_trait_method2<M: MolecularFormula>(m: &M) {
    assert_eq!(m.number_of_mixtures(), 1);
    for element in m.elements() {
        let count = m.count_of_element::<M::Count>(element).unwrap();
        assert!(!count.is_zero());
        assert!(m.contains_element(element));
    }
    assert!(m.contains_isotopes());
    assert!(!m.contains_isotope(HeliumIsotope::He3.into()));
    assert_eq!(m.count_of_isotope::<M::Count>(HeliumIsotope::He3.into()), Some(M::Count::zero()));
    assert!(m.contains_elements());
    assert!(
        (m.isotopologue_mass() - 9492.200799867811).abs() < f64::EPSILON,
        "Found mass: {}",
        m.isotopologue_mass()
    );
    assert!(!m.is_noble_gas_compound());
    assert!(m.is_hill_sorted());
}

fn test_all_charged_molecular_trait_method2<M: ChargedMolecularFormula>(m: &M) {
    assert!((m.charge() + 39.0).abs() < f64::EPSILON, "Found charge: {}", m.charge());
    assert!(
        (m.isotopologue_mass_with_charge() - 9492.222194484264).abs() < f64::EPSILON,
        "Found mass: {}",
        m.isotopologue_mass_with_charge()
    );
    assert!(
        (m.molar_mass() - 8686.916942536453).abs() < f64::EPSILON,
        "Found mass: {}",
        m.molar_mass()
    );
}

#[test]
fn test_largest_inchi() {
    let mixture = "C5H9NO.C5H11N.C5H10O2S.C5H10OS.C5H10O.C5H10S.C4H6N2O2.C4H8N2O.C4H10N2.C4H9NO2S.C4H7NO2.C4H9NOS.C4H9NO.C4H7NO.C4H9NS.C4H9N.3C4H7N.C4H8O3S.2C4H8O2S.2C4H6O2S.C4H8OS.2C4H6OS.C4H8O.2C4H6O.C4H8S.2C4H6S.2C3H6N2O.C3H8N2.3C3H6N2.C3H8N2.3C3H6N2.C3H7NO2S.3C3H5NO2S.C3H5NO2.C3H7NOS.4C3H5NOS.C3H7NO.3C3H5NO.C3H7NS.3C3H5NS.C3H7N.C3H6O3S.C3H4O3S.C3H6O2S.C3H4O2S.C3H6O2S.C3H6O2.C3H4O2.C3H6OS.C3H4OS.C3H6OS.C3H6O.C3H6S.C2H5N.C2H4O";
    let largest_inchi: InChIFormula = InChIFormula::from_str(mixture).unwrap();
    let serialized = serde_json::to_string(&largest_inchi).unwrap();
    let deserialized: InChIFormula = serde_json::from_str(&serialized).unwrap();
    assert_eq!(largest_inchi, deserialized);
    test_all_molecular_trait_method1(&largest_inchi);

    let largest_chemical: ChemicalFormula = ChemicalFormula::from_str(mixture).unwrap();
    let serialized = serde_json::to_string(&largest_chemical).unwrap();
    let deserialized: ChemicalFormula = serde_json::from_str(&serialized).unwrap();
    assert_eq!(largest_chemical, deserialized);
    test_all_molecular_trait_method1(&largest_chemical);
    test_all_charged_molecular_trait_method1(&largest_chemical);

    let largest_chemical: MineralFormula = MineralFormula::from_str(mixture).unwrap();
    let serialized = serde_json::to_string(&largest_chemical).unwrap();
    let deserialized: MineralFormula = serde_json::from_str(&serialized).unwrap();
    assert_eq!(largest_chemical, deserialized);
    test_all_molecular_trait_method1(&largest_chemical);
    test_all_charged_molecular_trait_method1(&largest_chemical);

    let largest_chemical: ResidualFormula = ResidualFormula::from_str(mixture).unwrap();
    let serialized = serde_json::to_string(&largest_chemical).unwrap();
    let deserialized: ResidualFormula = serde_json::from_str(&serialized).unwrap();
    assert_eq!(largest_chemical, deserialized);
}

#[test]
fn test_largest_formula() {
    let mixture = "•([C₃₉₀³H₄₀₄B₂Br₂ClCs₂F₁₁K₂MnN₂₆Na₂O₁₀₀OsPdS₃W₂•])³⁹⁻";

    assert_eq!(InChIFormula::<u32>::from_str(mixture), Err(ParserError::UnexpectedCharacter('•')));

    let largest_chemical: ChemicalFormula = ChemicalFormula::from_str(mixture).unwrap();
    let serialized = serde_json::to_string(&largest_chemical).unwrap();
    let deserialized: ChemicalFormula = serde_json::from_str(&serialized).unwrap();
    assert_eq!(largest_chemical, deserialized);
    test_all_molecular_trait_method2(&largest_chemical);
    test_all_charged_molecular_trait_method2(&largest_chemical);

    let largest_chemical: MineralFormula = MineralFormula::from_str(mixture).unwrap();
    let serialized = serde_json::to_string(&largest_chemical).unwrap();
    let deserialized: MineralFormula = serde_json::from_str(&serialized).unwrap();
    assert_eq!(largest_chemical, deserialized);
    test_all_molecular_trait_method2(&largest_chemical);
    test_all_charged_molecular_trait_method2(&largest_chemical);

    let largest_chemical: ResidualFormula = ResidualFormula::from_str(mixture).unwrap();
    let serialized = serde_json::to_string(&largest_chemical).unwrap();
    let deserialized: ResidualFormula = serde_json::from_str(&serialized).unwrap();
    assert_eq!(largest_chemical, deserialized);
}
