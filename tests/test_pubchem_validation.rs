//! Test suite for validating molecular formula parsing and mass calculations
//! against PubChem CID-Mass data.
//!
//! # Overview
//!
//! This test module validates the molecular formula parser and mass calculation
//! methods against the PubChem CID-Mass dataset.
//!
//! # Running Tests
//!
//! To run this test (validates all compounds):
//!
//! ```bash
//! cargo test --release --test test_pubchem_validation -- --ignored --nocapture
//! ```
use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fs::File,
    io::{BufReader, Write},
    path::Path,
};

use comfy_table::{Table, presets};
use csv::ReaderBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use molecular_formulas::MolecularFormula;
use serde::Deserialize;

/// Entry for a mass mismatch to be stored in the top-k heap
#[derive(Debug)]
struct MismatchEntry {
    cid: u64,
    formula: String,
    pubchem_mass: f64,
    calculated_mass: f64,
    error: f64,
}

impl PartialEq for MismatchEntry {
    fn eq(&self, other: &Self) -> bool {
        self.error == other.error
    }
}

impl Eq for MismatchEntry {}

impl PartialOrd for MismatchEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MismatchEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.error.total_cmp(&other.error)
    }
}

/// Structure representing a PubChem compound entry from the CID-Mass file.
#[derive(Debug, Deserialize)]
struct PubChemCompound {
    #[allow(dead_code)]
    /// PubChem Compound ID
    cid: u64,
    /// Molecular formula string
    formula: MolecularFormula,
    /// Monoisotopic mass from PubChem
    monoisotopic_mass: f64,
    /// Exact mass from PubChem (should be same as monoisotopic)
    #[allow(dead_code)]
    exact_mass: f64,
}

/// Statistics for validation results
#[derive(Debug, Default)]
struct ValidationStats {
    total_processed: usize,
    mass_matches: usize,
    mass_mismatches: usize,
    mass_tolerance_failures: usize,
    ion_mass_mismatches: usize,
    neutral_mass_mismatches: usize,
    tollerance: f64,
    time_required: f64,
    top_mismatches: BinaryHeap<Reverse<MismatchEntry>>,
    k: usize,
}

impl ValidationStats {
    fn write_mismatches_report(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;

        writeln!(file, "# Top {} Mass Mismatches\n", self.k)?;

        // Extract and sort entries (largest error first)
        let mut entries: Vec<_> = self.top_mismatches.iter().map(|Reverse(e)| e).collect();
        entries.sort_by(|a, b| b.error.partial_cmp(&a.error).unwrap());

        let mut table = Table::new();
        table.load_preset(presets::ASCII_MARKDOWN).set_header(vec![
            "CID",
            "Formula",
            "PubChem Mass",
            "Calc. Mass",
        ]);

        for entry in entries {
            table.add_row(vec![
                format!("[cid](https://pubchem.ncbi.nlm.nih.gov/compound/{cid})", cid = entry.cid),
                format!("`{}`", entry.formula),
                format!("{:.5}", entry.pubchem_mass),
                format!("{:.5}", entry.calculated_mass),
            ]);
        }

        writeln!(file, "{table}")?;
        println!("Report written to {filename}");
        Ok(())
    }

    fn print_summary(&self) {
        #[allow(clippy::cast_precision_loss)]
        let speed = self.total_processed as f64 / self.time_required;
        #[allow(clippy::cast_precision_loss)]
        let accuracy = ((self.mass_matches + self.mass_tolerance_failures) as f64
            / self.total_processed as f64)
            * 100.0;

        let mut table = Table::new();
        table.load_preset(presets::ASCII_MARKDOWN).set_header(vec!["Metric", "Value"]);

        table.add_row(vec!["Total processed".to_string(), self.total_processed.to_string()]);
        table.add_row(vec![
            "Total time required".to_string(),
            format!("{:.2} s", self.time_required),
        ]);
        table.add_row(vec!["Processing speed".to_string(), format!("{:.2} cmp/s", speed)]);
        table.add_row(vec!["Exact matches".to_string(), self.mass_matches.to_string()]);
        table.add_row(vec![
            "Within tolerance".to_string(),
            self.mass_tolerance_failures.to_string(),
        ]);
        table.add_row(vec!["Mismatches".to_string(), self.mass_mismatches.to_string()]);
        table.add_row(vec!["  - Ion mismatches".to_string(), self.ion_mass_mismatches.to_string()]);
        table.add_row(vec![
            "  - Neutral mismatches".to_string(),
            self.neutral_mass_mismatches.to_string(),
        ]);
        table.add_row(vec![
            format!("Mass accuracy (within {})", self.tollerance),
            format!("{:.2}%", accuracy),
        ]);

        println!("\n=== PubChem Validation Summary ===");
        println!("{table}");
    }
}

/// Read and validate PubChem data from the CID-Mass file
fn validate_pubchem_data(
    file_path: &Path,
    mass_tolerance: f64,
    k_worst: usize,
) -> Result<ValidationStats, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut stats =
        ValidationStats { tollerance: mass_tolerance, k: k_worst, ..Default::default() };

    let mut csv_reader =
        ReaderBuilder::new().delimiter(b'\t').has_headers(false).from_reader(reader);

    let pb = ProgressBar::new(123_455_852);

    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    let start = std::time::Instant::now();

    for result in csv_reader.deserialize::<PubChemCompound>() {
        let result = result?;
        stats.total_processed += 1;
        pb.inc(1);

        let formula = &result.formula;
        let is_ion = formula.charge() != 0.0;
        let calculated_mass = formula.isotopologue_mass_without_charge();
        let mass_diff = (calculated_mass - result.monoisotopic_mass).abs();

        if mass_diff > mass_tolerance {
            let entry = MismatchEntry {
                cid: result.cid,
                formula: formula.to_string(),
                pubchem_mass: result.monoisotopic_mass,
                calculated_mass,
                error: mass_diff,
            };

            // Maintain top-k heap (min-heap of Reverse<MismatchEntry> keeps smallest error
            // of the top-k at top) We want to keep the LARGEST errors.
            // BinaryHeap is a max-heap. Reverse reverts the comparison.
            // So heap.peek() is the SMALLEST error currently in the heap.
            // If new error > heap.peek(), we pop the smallest and push the new one.

            if stats.top_mismatches.len() < k_worst {
                stats.top_mismatches.push(Reverse(entry));
            } else if let Some(Reverse(smallest_top_error)) = stats.top_mismatches.peek()
                && mass_diff > smallest_top_error.error
            {
                stats.top_mismatches.pop();
                stats.top_mismatches.push(Reverse(entry));
            }
        }

        if mass_diff < 1e-10 {
            stats.mass_matches += 1;
        } else if mass_diff <= mass_tolerance {
            stats.mass_tolerance_failures += 1;
        } else {
            stats.mass_mismatches += 1;
            if is_ion {
                stats.ion_mass_mismatches += 1;
            } else {
                stats.neutral_mass_mismatches += 1;
            }
        }
    }

    stats.time_required = start.elapsed().as_secs_f64();

    pb.finish_with_message("Validation complete");
    Ok(stats)
}

#[test]
#[ignore = "This test requires the CID-Mass file to be present and is time-consuming."]
/// Validate molecular formula parsing and mass calculations against PubChem
/// CID-Mass data.
///
/// The document, weighing compressed approximately 1.3 GB, can be downloaded
/// from:
///
/// <https://ftp.ncbi.nlm.nih.gov/pubchem/Compound/Extras/CID-Mass.gz>
fn test_pubchem_validation() {
    let file_path = Path::new("CID-Mass");

    if !file_path.exists() {
        eprintln!("CID-Mass file not found. Skipping test.");
        return;
    }

    println!("Validating all PubChem compounds...");

    match validate_pubchem_data(file_path, 0.001, 30) {
        Ok(stats) => {
            stats.print_summary();
            if let Err(e) = stats.write_mismatches_report("worst_mismatches.md") {
                eprintln!("Failed to write mismatches report: {e}");
            }
        }
        Err(e) => {
            panic!("Error during validation: {e}");
        }
    }
}
