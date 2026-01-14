//! Submodule for fuzzing the execution of the Hopcroft-Karp algorithm.

use std::str::FromStr;

use honggfuzz::fuzz;
use molecular_formulas::MolecularFormula;

fn main() {
    loop {
        fuzz!(|candidate: &str| {
            // If the candidate has more than 10 characters, skip it
            if candidate.len() > 10 {
                return;
            }

            let start_time = std::time::Instant::now();
            let Ok(formula) = MolecularFormula::from_str(candidate) else {
                // If the candidate is not a valid formula, we can skip it
                return;
            };
            let elapsed = start_time.elapsed();

            // If the parsing took more than 0.5 second, we raise an error
            // so to turn a timeout into a panic
            if elapsed.as_secs_f64() > 0.5 {
                panic!(
                    "Parsing candidate `{candidate}` took too long: {} seconds",
                    elapsed.as_secs_f64()
                );
            }

            println!("Parsed formula `{}`", candidate);

            // We check that the display works without panicking
            let _ = formula.to_string();

            // Test various property methods that should not panic
            let _ = formula.contains_residual();
            let _ = formula.contains_mixture();
            let _ = formula.contains_isotope();
            let _ = formula.contains_elements();
            let _ = formula.is_countable();
            let _ = formula.number_of_mixtures();

            // Test methods that may return errors but should not panic
            let _ = formula.molar_mass();
            let _ = formula.isotopologue_mass_with_charge();
            let _ = formula.isotopologue_mass_without_charge();
            let _ = formula.isotopologue_mass_over_charge();
            let _ = formula.charge();
            let _ = formula.number_of_elements();
            let _ = formula.is_diatomic();
            let _ = formula.is_homonuclear();
            let _ = formula.is_noble_gas_compound();
            let _ = formula.oxidation_states();

            // Test iterator methods
            for _mixture in formula.mixtures() {
                // Just iterate to ensure no panics
            }

            // Test cloning and equality
            let cloned = formula.clone();
            let _ = formula == cloned;
            let _ = formula.partial_cmp(&cloned);

            // Test Debug formatting
            let _ = format!("{:?}", formula);

            // Test serialization if serde is enabled
            if let Ok(serialized) = serde_json::to_string(&formula) {
                let _: Result<MolecularFormula, _> = serde_json::from_str(&serialized);
            }
        });
    }
}
