//! Submodule for fuzzing molecular formulas from strings.

use std::{
    fmt::{Debug, Display},
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
    let _ = formula.get_element(46);
    let _ = formula.get_element_ignore_hydrogens(0);
    let _ = formula.get_element_ignore_hydrogens(46);
    let _ = formula.contains_isotopes();
    let _ = formula.contains_elements();
    let _ = formula.number_of_mixtures();
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

fn main() {
    loop {
        fuzz!(|data: FuzzFormula<CountType, ChargeType, Residual>| {
            if let Some(formula) = parse::<ChemicalFormula<CountType, ChargeType>>(&data.as_ref()) {
                round_trip(&data.as_ref(), &formula);
                fuzz_molecular_formula(&formula);
                fuzz_charged_molecular_formula(&formula);
            }

            if let Some(formula) = parse::<MineralFormula<CountType, ChargeType>>(&data.as_ref()) {
                round_trip(&data.as_ref(), &formula);
                fuzz_molecular_formula(&formula);
                fuzz_charged_molecular_formula(&formula);
            }

            if let Some(formula) = parse::<InChIFormula<CountType>>(&data.as_ref()) {
                round_trip(&data.as_ref(), &formula);
                fuzz_molecular_formula(&formula);
            }

            // Fuzz ResidualFormula - Has subset of methods
            if let Some(formula) = parse::<ResidualFormula<CountType, ChargeType>>(&data.as_ref()) {
                round_trip(&data.as_ref(), &formula);
                // Specific methods
                let _ = formula.contains_residuals();
            }
        });
    }
}
