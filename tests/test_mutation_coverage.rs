//! Tests added to close mutation-testing gaps (cargo-mutants).
//!
//! Each assertion here is designed to fail under a specific surviving mutant,
//! exercising tree/node predicate methods, charge computation, Hill ordering,
//! element counting, and InChI mixture merging through the public parsing API.

use std::str::FromStr;

use elements_rs::{Element, Isotope};
use molecular_formulas::prelude::*;

fn chem(s: &str) -> ChemicalFormula<u16, i16> {
    ChemicalFormula::<u16, i16>::from_str(s).unwrap_or_else(|e| panic!("parse {s:?}: {e:?}"))
}

fn inchi(s: &str) -> InChIFormula<u16> {
    InChIFormula::<u16>::from_str(s).unwrap_or_else(|e| panic!("parse {s:?}: {e:?}"))
}

#[track_caller]
fn assert_charge(actual: f64, expected: f64, context: &str) {
    assert!((actual - expected).abs() < f64::EPSILON, "{context}: charge {actual} != {expected}");
}

/// `is_noble_gas_compound` across every wrapper node (bracket, charge, radical,
/// repeat, sequence) and the InChI delegation. Kills `-> false`/`-> true` and
/// `all()`/`any()` mutants.
#[test]
fn noble_gas_compound() {
    for s in
        ["He", "Ne", "Ar", "HeNe", "He2", "(He)", "(He)2", "(HeNe)2", "He·", "Ar.Kr", "He⁺", "Xe²⁺"]
    {
        assert!(chem(s).is_noble_gas_compound(), "{s} should be a noble gas compound");
    }
    for s in ["H2O", "HeO", "CH4", "(HeO)", "HeO·", "He.O", "NaCl"] {
        assert!(!chem(s).is_noble_gas_compound(), "{s} should not be a noble gas compound");
    }
    // InChI delegation.
    assert!(inchi("He").is_noble_gas_compound());
    assert!(!inchi("CH4").is_noble_gas_compound());
}

/// `contains_isotopes` across wrapper nodes and InChI delegation.
#[test]
fn contains_isotopes_cases() {
    for s in ["[13C]", "[13C]H4", "([13C])", "[13C]2", "([13C])2", "[13C]·", "[13C]⁻", "C.[13C]"]
    {
        assert!(chem(s).contains_isotopes(), "{s} should contain isotopes");
    }
    for s in ["C", "CH4", "(CH4)", "CH4·", "(CO2)", "CO2⁻", "C.O"] {
        assert!(!chem(s).contains_isotopes(), "{s} should not contain isotopes");
    }
    // InChI is element-only, so it never contains isotopes.
    assert!(!inchi("CH4").contains_isotopes());
}

/// `contains_non_hydrogens` across wrapper nodes, sequences, and InChI.
#[test]
fn contains_non_hydrogens_cases() {
    for s in ["C", "CH4", "(CH4)", "CH4·", "(CO2)⁻", "CO2", "[13C]", "HC"] {
        assert!(chem(s).contains_non_hydrogens(), "{s} should contain non-hydrogens");
    }
    for s in ["H", "H2", "(H2)", "H2·", "[2H]", "[2H]2", "HH", "H⁻", "(H2)²⁻", "H2·"] {
        assert!(!chem(s).contains_non_hydrogens(), "{s} should not contain non-hydrogens");
    }
    assert!(inchi("CH4").contains_non_hydrogens());
    assert!(!inchi("H2").contains_non_hydrogens());
}

/// `contains_element` and `contains_isotope`: present and absent cases through
/// wrapper nodes.
#[test]
fn contains_specific_element_and_isotope() {
    let c13 = Isotope::try_from((Element::C, 13u16)).unwrap();
    let c12_iso = Isotope::try_from((Element::C, 12u16)).unwrap();

    for s in ["CO2", "(CO2)", "CO2·", "(CO2)⁻", "CO2.H2O"] {
        let f = chem(s);
        assert!(f.contains_element(Element::O), "{s} should contain O");
        assert!(f.contains_element(Element::C), "{s} should contain C");
        assert!(!f.contains_element(Element::N), "{s} should not contain N");
    }

    for s in ["[13C]H4", "([13C]H4)", "[13C]H4·", "([13C]H4)⁻"] {
        let f = chem(s);
        assert!(f.contains_isotope(c13), "{s} should contain 13C");
        assert!(!f.contains_isotope(c12_iso), "{s} should not contain 12C isotope");
    }

    // An isotope node must answer `contains_element` by its base element: an
    // isotope-only formula contains its base element but not other elements.
    for s in ["[13C]", "[13C]2", "([13C])⁻"] {
        let f = chem(s);
        assert!(f.contains_element(Element::C), "{s} should contain C");
        assert!(!f.contains_element(Element::O), "{s} should not contain O");
    }
    assert!(inchi("CO2").contains_element(Element::O));
    assert!(!inchi("CO2").contains_element(Element::N));
}

/// `number_of_elements` repeat multiplication (`*` vs `/`).
#[test]
fn number_of_elements_repeat() {
    assert_eq!(chem("(H2O)2").number_of_elements(), 6);
    assert_eq!(chem("(H3)2").number_of_elements(), 6);
    assert_eq!(chem("O10").number_of_elements(), 10);
    assert_eq!(chem("(CH3)3").number_of_elements(), 12);
    assert_eq!(chem("Ca(OH)2").number_of_elements(), 5);
}

/// Charge through wrapper nodes: kills the bracket `charge -> -1.0` mutant and
/// repeat/sequence charge multiplication/summation.
#[test]
fn charge_cases() {
    assert_charge(chem("Cl⁻").charge(), -1.0, "Cl⁻");
    assert_charge(chem("Ca²⁺").charge(), 2.0, "Ca²⁺");
    assert_charge(chem("(H2O)").charge(), 0.0, "(H2O)"); // bracket charge delegates, not -1.0
    assert_charge(chem("(OH)⁻").charge(), -1.0, "(OH)⁻");
    assert_charge(chem("H₂O").charge(), 0.0, "H₂O");
    // Multi-mixture charge sums.
    assert_charge(chem("Na⁺.Cl⁻").charge(), 0.0, "Na⁺.Cl⁻");
    assert_charge(chem("Ca²⁺.Cl⁻").charge(), 1.0, "Ca²⁺.Cl⁻");
}

/// Charge accumulation overflow classification (the parse_charge fix): the most
/// negative charge value is representable and overflow is labelled by sign.
#[test]
fn charge_overflow_sign() {
    use molecular_formulas::errors::{NumericError, ParserError};

    let pos_max = format!("H{}", "+".repeat(127));
    let neg_min = format!("H{}", "-".repeat(128));
    let pos_over = format!("H{}", "+".repeat(128));
    let neg_over = format!("H{}", "-".repeat(129));

    assert_charge(ChemicalFormula::<u16, i8>::from_str(&pos_max).unwrap().charge(), 127.0, "+127");
    assert_charge(ChemicalFormula::<u16, i8>::from_str(&neg_min).unwrap().charge(), -128.0, "-128");
    assert_eq!(
        ChemicalFormula::<u16, i8>::from_str(&pos_over),
        Err(ParserError::Numeric(NumericError::PositiveOverflow))
    );
    assert_eq!(
        ChemicalFormula::<u16, i8>::from_str(&neg_over),
        Err(ParserError::Numeric(NumericError::NegativeOverflow))
    );
}

/// Hill ordering routed through wrapper nodes: kills `check_hill_ordering ->
/// Ok(None)` mutants on bracket/charge/radical/isotope/sequence nodes.
#[test]
fn hill_ordering_through_wrappers() {
    // Sorted.
    for s in ["CH4", "C2H6O", "(CH3)2", "H2O", "CO2⁻"] {
        assert!(chem(s).is_hill_sorted(), "{s} should be Hill sorted");
    }
    // Unsorted (carbon not first / wrong alpha order), via various nodes. The
    // `O[13C]` case routes Hill ordering through the isotope node (a 13C isotope
    // ordered after O is out of order, since carbon must come first).
    for s in ["OC", "(OC)", "OC·", "H2OC", "OC⁻", "HCl", "O[13C]"] {
        assert!(!chem(s).is_hill_sorted(), "{s} should not be Hill sorted");
    }
}

/// `has_carbon` (`==` vs `!=`) drives InChI merge ordering: an isotope-only
/// carbon must count as carbon.
#[test]
fn has_carbon_via_merge() {
    // Merging a formula whose only carbon is an isotope must still treat it as
    // carbon-bearing, so Hill ordering puts carbon first. Boron sorts before
    // carbon alphabetically, so if `has_carbon` wrongly returned false the merged
    // display would start with boron instead. This exercises the
    // `isotope.element() == Element::C` branch of `has_carbon`.
    let merged = chem("[13C]H4.B2H6").merge_mixtures().unwrap();
    assert!(merged.contains_isotope(Isotope::try_from((Element::C, 13u16)).unwrap()));
    assert!(merged.to_string().starts_with("[¹³C]"), "got {merged}");
}

/// InChI `merge_mixtures` condition (`&&`/`==`): single unit mixture is
/// returned unchanged; multi-mixture formulas are actually merged.
#[test]
fn merge_mixtures_inchi() {
    // Single unit-count mixture: unchanged.
    let single = inchi("CH4");
    let merged_single = single.merge_mixtures().unwrap();
    assert_eq!(merged_single.number_of_mixtures(), 1);
    assert_eq!(merged_single.count_of_element::<u32>(Element::H), Ok(4));

    // Multi-mixture: merged into one with summed counts.
    let multi = inchi("CH4.CH4");
    let merged_multi = multi.merge_mixtures().unwrap();
    assert_eq!(merged_multi.number_of_mixtures(), 1);
    assert_eq!(merged_multi.count_of_element::<u32>(Element::H), Ok(8));
    assert_eq!(merged_multi.count_of_element::<u32>(Element::C), Ok(2));
}

/// `counted_mixtures_mut` must yield one entry per mixture (not an empty
/// iterator) for InChI and Mineral formulas.
#[test]
fn counted_mixtures_mut_yields_entries() {
    let mut inchi_formula = inchi("CH4.H2O");
    assert_eq!(inchi_formula.counted_mixtures_mut().count(), 2);

    let mut mineral = MineralFormula::<u16, i16>::from_str("CaCO3").unwrap();
    assert_eq!(mineral.counted_mixtures_mut().count(), 1);
}

/// InChI `is_empty` (`-> false`): an empty mixture (e.g. a trailing or leading
/// separator) must be rejected as an empty molecular tree.
#[test]
fn inchi_rejects_empty_mixture() {
    for s in [".CH4", "CH4..H2O"] {
        assert!(
            InChIFormula::<u16>::from_str(s).is_err(),
            "{s} should be rejected (empty mixture)"
        );
    }
}

/// Non-ASCII minus variants (BaselineMinus `||` vs `&&`): each must parse as a
/// negative charge.
#[test]
fn baseline_minus_variants() {
    // U+002D hyphen-minus, U+2212 minus, U+2010 hyphen, U+2011, U+2012, U+2013,
    // U+2014, U+2015, U+FF0D, U+FE63.
    for c in [
        '\u{002D}', '\u{2212}', '\u{2010}', '\u{2011}', '\u{2012}', '\u{2013}', '\u{2014}',
        '\u{2015}', '\u{FF0D}', '\u{FE63}',
    ] {
        let s = format!("Cl{c}");
        assert_charge(chem(&s).charge(), -1.0, &format!("{s:?} (U+{:04X})", c as u32));
    }
}

/// Illegal charge successor (`parse_any_illegal_charge_successor` `||` vs
/// `&&`): a charge immediately followed by another charge/superscript-digit is
/// an error.
#[test]
fn illegal_charge_successor() {
    for s in ["H+-", "H-+", "Na⁺⁻", "Cl⁻⁺"] {
        assert!(
            ChemicalFormula::<u16, i16>::from_str(s).is_err(),
            "{s} should be an illegal charge successor"
        );
    }
}
