//! Submodule for fuzzing molecular formulas from strings.

use std::{
    collections::hash_map::DefaultHasher,
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    str::FromStr,
};

use honggfuzz::fuzz;
use molecular_formulas::{fuzzing::FuzzFormula, prelude::*};
use serde::{Serialize, de::DeserializeOwned};

fn parse<M: FromStr>(candidate: &str) -> Option<M> {
    let start_time = std::time::Instant::now();
    let result = M::from_str(candidate);
    let elapsed = start_time.elapsed();

    if let Ok(formula) = result {
        // If the parsing took more than 0.5 second, we raise an error
        // so to turn a timeout into a panic
        if elapsed.as_secs_f64() > 0.5 {
            panic!(
                "Parsing candidate `{candidate}` type `{}` took too long: {} seconds",
                stringify!(M),
                elapsed.as_secs_f64()
            );
        }
        Some(formula)
    } else {
        None
    }
}

fn round_trip<M: Display + FromStr<Err: Display> + Eq + Debug + Serialize + DeserializeOwned>(
    candidate: &str,
    formula: &M,
) {
    // We check that the display works without panicking
    let display_string = formula.to_string();

    // Round-trip test:
    // parsing the display string should result in an equivalent formula.
    match M::from_str(&display_string) {
        Ok(reparsed_formula) => {
            assert_eq!(
                formula, &reparsed_formula,
                "Round trip failed of {candidate}. Original: {formula}, Display: {display_string}, Re-parsed: `{reparsed_formula}`"
            )
        }
        Err(err) => {
            panic!(
                "Failed to re-parse display string of {candidate}. Original: {formula}, Display: {display_string}, Error: {err}"
            )
        }
    }

    // Test Debug formatting
    let _ = format!("{:?}", formula);

    // Test serialization if serde is enabled
    if let Ok(serialized) = serde_json::to_string(formula) {
        let _: Result<M, _> = serde_json::from_str(&serialized);
    }
}

/// Verifies that all common traits are well-behaved.
fn fuzz_common_traits<M: Clone + PartialEq + Debug + Hash>(formula: &M) {
    // Test Clone + Eq
    assert_eq!(formula, &formula.clone(), "Formula should be equal to its clone");

    // Test Hash consistency
    let mut s1 = DefaultHasher::new();
    formula.hash(&mut s1);
    let h1 = s1.finish();

    let mut s2 = DefaultHasher::new();
    formula.clone().hash(&mut s2);
    let h2 = s2.finish();

    assert_eq!(h1, h2, "Hash of clone should match hash of original");
}

/// Verifies that all of the methods of the MolecularFormula trait can be called
/// without panicking.
fn fuzz_molecular_formula<M: MolecularFormula>(formula: &M)
where
    u64: From<M::Count>,
{
    let _ = formula.isotopologue_mass();
    let _ = formula.is_noble_gas_compound();
    let _ = formula.is_hill_sorted();
    let _ = formula.get_element(0);
    // We test a large index to ensure it doesn't panic on OOB or iteration limits
    let _ = formula.get_element(46);
    let _ = formula.get_element_ignore_hydrogens(0);
    let _ = formula.get_element_ignore_hydrogens(46);
    let _ = formula.contains_isotopes();
    let contains_elements = formula.contains_elements();
    let _ = formula.number_of_mixtures();

    // Check elements consistency
    if contains_elements {
        assert!(
            formula.elements().next().is_some(),
            "contains_elements is true but elements() is empty"
        );
    } else {
        assert!(
            formula.elements().next().is_none(),
            "contains_elements is false but elements() is not empty"
        );
    }

    // Test element/isotope queries
    let _ = formula.count_of_element::<u64>(Element::C);
    let _ = formula.count_of_element::<u64>(Element::H);
    if let Ok(iso) = Isotope::try_from((Element::H, 1_u16)) {
        let _ = formula.contains_isotope(iso);
        let _ = formula.count_of_isotope::<u64>(iso);
    }
}

/// Verifies that all of the methods of the ChargedMolecularFormula trait can be
/// called without panicking.
fn fuzz_charged_molecular_formula<M: ChargedMolecularFormula>(formula: &M) {
    let _ = formula.molar_mass();
    let _ = formula.isotopologue_mass_with_charge();
    let _ = formula.isotopologue_mass_over_charge();
    let _ = formula.charge();
}

/// We need to use an `u16` count type to ensure that all possible Isotope
/// values can be represented (some isotopes have mass numbers > 255).
type CountType = u16;
/// We use the smallest possible charge type.
type ChargeType = i8;

/// Fuzz operations specific to ChemicalFormula (e.g. addition)
fn fuzz_chemical_formula_ops(formula: &ChemicalFormula<CountType, ChargeType>) {
    // Test Addition
    let doubled = formula.clone() + formula.clone();

    // Check that counts double for a few common elements
    let elements_to_check = [Element::C, Element::H, Element::O, Element::N];
    for element in elements_to_check {
        let count = formula.count_of_element::<u64>(element);
        let doubled_count = doubled.count_of_element::<u64>(element);
        match (count, doubled_count) {
            (Some(c), Some(dc)) => {
                assert_eq!(c * 2, dc, "Doubling formula should double element count for {element}");
            }
            (None, None) => {}
            _ => panic!("Count overflow or mismatch in addition fuzzing for {element}"),
        }
    }

    // Check charge doubles (approximately, allowing for float precision)
    let charge = formula.charge();
    let doubled_charge = doubled.charge();
    if charge.is_finite() && doubled_charge.is_finite() {
        let diff = (charge * 2.0 - doubled_charge).abs();
        assert!(diff < 1e-4, "Charge addition mismatch: {charge} * 2 != {doubled_charge}");
    }

    // Check mass comparison
    if formula.contains_elements() {
        // Molar mass should be roughly doubling
        let mass = formula.molar_mass();
        let doubled_mass = doubled.molar_mass();
        if mass.is_finite() && doubled_mass.is_finite() {
            assert!(
                doubled_mass >= mass,
                "Doubled mass {doubled_mass} should be >= original mass {mass}"
            );
        }
    }
}

fn main() {
    loop {
        fuzz!(|data: FuzzFormula<CountType, ChargeType, Residual>| {
            if let Some(formula) = parse::<ChemicalFormula<CountType, ChargeType>>(&data.as_ref()) {
                round_trip(&data.as_ref(), &formula);
                fuzz_common_traits(&formula);
                fuzz_molecular_formula(&formula);
                fuzz_charged_molecular_formula(&formula);
                fuzz_chemical_formula_ops(&formula);
            }

            if let Some(formula) = parse::<MineralFormula<CountType, ChargeType>>(&data.as_ref()) {
                round_trip(&data.as_ref(), &formula);
                fuzz_common_traits(&formula);
                fuzz_molecular_formula(&formula);
                fuzz_charged_molecular_formula(&formula);
            }

            if let Some(formula) = parse::<InChIFormula<CountType>>(&data.as_ref()) {
                round_trip(&data.as_ref(), &formula);
                fuzz_common_traits(&formula);
                fuzz_molecular_formula(&formula);
            }

            // Fuzz ResidualFormula - Has subset of methods
            if let Some(formula) = parse::<ResidualFormula<CountType, ChargeType>>(&data.as_ref()) {
                round_trip(&data.as_ref(), &formula);
                fuzz_common_traits(&formula);
                // Specific methods
                let _ = formula.contains_residuals();
            }
        });
    }
}
