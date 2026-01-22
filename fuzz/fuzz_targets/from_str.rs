//! Submodule for fuzzing the execution of the Hopcroft-Karp algorithm.

use std::str::FromStr;

use honggfuzz::fuzz;
use molecular_formulas::{DefaultTree, MolecularFormula, ResidualFormula};

macro_rules! fuzz_headers {
    ($type:ty, $candidate:expr) => {{
        let start_time = std::time::Instant::now();
        let result = <$type>::from_str($candidate);
        let elapsed = start_time.elapsed();

        if let Ok(formula) = result {
            // If the parsing took more than 0.5 second, we raise an error
            // so to turn a timeout into a panic
            if elapsed.as_secs_f64() > 0.5 {
                panic!(
                    "Parsing candidate `{}` type `{}` took too long: {} seconds",
                    $candidate,
                    stringify!($type),
                    elapsed.as_secs_f64()
                );
            }
            Some(formula)
        } else {
            None
        }
    }};
}

macro_rules! fuzz_common {
    ($type:ty, $candidate:expr, $formula:expr) => {{
        // We check that the display works without panicking
        let display_string = $formula.to_string();

        // Round-trip test:
        // parsing the display string should result in an equivalent formula.
        match <$type>::from_str(&display_string) {
            Ok(reparsed_formula) => {
                assert_eq!(
                    $formula, reparsed_formula,
                    "Round trip failed. Original: {}, Display: {}, Re-parsed: `{}`",
                    $candidate, display_string, reparsed_formula
                )
            }
            Err(err) => {
                panic!(
                    "Failed to re-parse display string. Original: {:?}, Display: {:?}, Error: {:?}",
                    $candidate, display_string, err
                )
            }
        }

        // Test various property methods that should not panic
        let _ = $formula.contains_isotopes();
        let _ = $formula.contains_elements();
        let _ = $formula.number_of_mixtures();

        // Test iterator methods
        for _mixture in $formula.mixtures().take(10) {
            // Just iterate to ensure no panics
        }

        // Test cloning and equality
        let cloned = $formula.clone();
        let _ = $formula == cloned;

        // Test mixing with itself (should double the counts)
        let _ = $formula.clone().mix(cloned);

        // Test Debug formatting
        let _ = format!("{:?}", $formula);

        // Test serialization if serde is enabled
        if let Ok(serialized) = serde_json::to_string(&$formula) {
            let _: Result<$type, _> = serde_json::from_str(&serialized);
        }
    }};
}

fn main() {
    loop {
        fuzz!(|candidate: &str| {
            // Start the timer for the entire test case
            let total_start = std::time::Instant::now();

            // If the candidate has more than 200 characters, skip it
            if candidate.len() > 200 {
                return;
            }

            // Fuzz MolecularFormula (DefaultTree) - Has all methods
            if let Some(formula) = fuzz_headers!(MolecularFormula<DefaultTree>, candidate) {
                fuzz_common!(MolecularFormula<DefaultTree>, candidate, formula);

                // Methods specific to NoResidualsTree
                let _ = formula.molar_mass();
                let _ = formula.isotopologue_mass_with_charge();
                let _ = formula.isotopologue_mass_without_charge();
                let _ = formula.isotopologue_mass_over_charge();
                let _ = formula.charge();
                let _ = formula.is_noble_gas_compound();
            }

            // Fuzz ResidualFormula - Has subset of methods
            if let Some(formula) = fuzz_headers!(ResidualFormula, candidate) {
                fuzz_common!(ResidualFormula, candidate, formula);
                // No extra methods available
            }

            // Check if the total execution time exceeded 0.7 seconds
            if total_start.elapsed().as_secs_f64() > 0.7 {
                panic!("Test case took too long (> 0.7s) for candidate: {:?}", candidate);
            }
        });
    }
}
