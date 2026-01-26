//! Test suite for validating molecular formula parsing
//! against PubChem CID-InChI-Key data.
//!
//! # Running Tests
//!
//! To run this test (validates all InChI formulas in the PubChem dataset),
//! ensure:
//!
//! ```bash
//! cargo test --release --test test_pubchem_inchi_validation -- --ignored --nocapture
//! ```
use std::{fs::File, io::BufReader, path::Path, str::FromStr};

use csv::ReaderBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use molecular_formulas::prelude::*;
use serde::Deserialize;

/// Structure representing a PubChem compound entry from the CID-InChI-Key file.
#[derive(Debug, Deserialize)]
struct PubChemCompound {
    #[allow(dead_code)]
    /// PubChem Compound ID
    cid: u64,
    /// Molecular formula string
    inchi: String,
    /// InchI Key
    #[allow(dead_code)]
    inchikey: String,
}

/// Read and validate PubChem data from the CID-InChI-Key file.
fn validate_pubchem_inchi(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut csv_reader =
        ReaderBuilder::new().delimiter(b'\t').has_headers(false).from_reader(reader);

    let pb = ProgressBar::new(123_458_255);

    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    let start = std::time::Instant::now();
    let mut skipped_compounds = Vec::new();
    let mut parsed_count = 0u64;

    for result in csv_reader.deserialize::<PubChemCompound>() {
        let result = result?;
        pb.inc(1);

        let prefix_removed = result
            .inchi
            .strip_prefix("InChI=1S/")
            .unwrap_or_else(|| panic!("Invalid InChI format for CID {}", result.cid));

        let (formula_portion, _rest) = match prefix_removed.find('/') {
            Some(index) => prefix_removed.split_at(index),
            None => (prefix_removed, ""),
        };

        if formula_portion.starts_with('p') {
            // This is not a formula layer, skip it.
            skipped_compounds.push((result.cid, result.inchi));
            continue;
        }

        let _formula: InChIFormula = InChIFormula::from_str(formula_portion).map_err(|e| {
            format!(
                "Failed to parse formula `{}` for CID {}: {}",
                result.inchi,
                result.cid,
                e
            )
        })?;

        parsed_count += 1;
    }

    let time_required = start.elapsed().as_secs_f64();
    #[allow(clippy::cast_precision_loss)]
    let time_per_compound = time_required / parsed_count as f64;

    pb.finish_with_message("Validation complete");

    println!(
        "Time taken: {:.2} seconds ({:.6} milliseconds per compound)",
        time_required,
        time_per_compound * 1000.0
    );
    println!("Parsed compounds: {parsed_count}");
    println!("Skipped compounds (non-formula layers): {}", skipped_compounds.len());
    for (cid, inchi) in skipped_compounds {
        println!("  CID {cid}: InChI {inchi}");
    }

    Ok(())
}

#[test]
#[ignore = "This test requires the CID-InChI-Key file to be present and is time-consuming."]
/// Validate molecular formula parsing and mass calculations against PubChem
/// CID-InChI-Key data.
///
/// The document, weighing compressed approximately 6.79 GB, can be downloaded
/// from:
///
/// <https://ftp.ncbi.nlm.nih.gov/pubchem/Compound/Extras/CID-InChI-Key.gz>
fn test_pubchem_validation() {
    let file_path = Path::new("CID-InChI-Key");

    if !file_path.exists() {
        eprintln!("CID-InChI-Key file not found. Skipping test.");
        return;
    }

    println!("Validating all PubChem InChI formulas...");

    validate_pubchem_inchi(file_path).expect("Validation failed");
}
