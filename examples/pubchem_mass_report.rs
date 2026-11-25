//! Summarize agreement with PubChem `CID-Mass` and write the largest
//! discrepancies to Markdown.

extern crate alloc;

use alloc::collections::BinaryHeap;
use core::cmp::{Ordering, Reverse};
use std::{
    env,
    fs::File,
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
    process::ExitCode,
    time::{Duration, Instant},
};

use comfy_table::{Table, presets};
use indicatif::ProgressBar;
use molecular_formulas::prelude::*;
use num_traits::Zero;

#[path = "../tests/support/pubchem_mass.rs"]
mod pubchem_mass;

use pubchem_mass::{MassComparison, MassDataError, TOLERANCE, compare_masses, open_mass_file};

const TOP_MISMATCHES: usize = 30;

#[derive(Debug, thiserror::Error)]
enum ReportError {
    #[error("usage: pubchem_mass_report <CID-Mass file> <new report path>")]
    Usage,
    #[error(transparent)]
    Input(#[from] MassDataError),
    #[error("could not write {}: {source}", path.display())]
    Output { path: PathBuf, source: io::Error },
    #[error("could not print summary: {0}")]
    Summary(io::Error),
}

struct Mismatch {
    comparison: MassComparison,
    difference: f64,
}

impl Ord for Mismatch {
    fn cmp(&self, other: &Self) -> Ordering {
        self.difference
            .total_cmp(&other.difference)
            .then_with(|| self.comparison.compound_id.cmp(&other.comparison.compound_id))
    }
}

impl PartialOrd for Mismatch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Mismatch {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for Mismatch {}

#[derive(Default)]
struct Report {
    exact: u64,
    within_tolerance: u64,
    ion_mismatches: u64,
    neutral_mismatches: u64,
    largest: BinaryHeap<Reverse<Mismatch>>,
}

impl Report {
    fn record(&mut self, comparison: MassComparison) {
        let difference = (comparison.calculated_mass - comparison.expected_mass).abs();
        if difference < 1e-10 {
            self.exact += 1;
        } else if comparison.agrees() {
            self.within_tolerance += 1;
        } else {
            if comparison.formula.charge().is_zero() {
                self.neutral_mismatches += 1;
            } else {
                self.ion_mismatches += 1;
            }
            let entry = Reverse(Mismatch { comparison, difference });
            if self.largest.len() < TOP_MISMATCHES {
                self.largest.push(entry);
            } else if let Some(mut smallest) = self.largest.peek_mut()
                && entry < *smallest
            {
                *smallest = entry;
            }
        }
    }

    #[expect(clippy::cast_precision_loss, reason = "Throughput and percentage are display values")]
    fn write_summary(&self, mut output: impl Write, elapsed: Duration) -> io::Result<()> {
        let matches = self.exact + self.within_tolerance;
        let mismatches = self.ion_mismatches + self.neutral_mismatches;
        let total = matches + mismatches;
        let seconds = elapsed.as_secs_f64();
        writeln!(
            output,
            "Processed {total} compounds in {seconds:.2} s ({:.2} compounds/s)",
            total as f64 / seconds
        )?;
        writeln!(output, "Exact matches (difference < 1e-10 Da): {}", self.exact)?;
        writeln!(output, "Other matches within {TOLERANCE} Da: {}", self.within_tolerance)?;
        writeln!(
            output,
            "Mismatches: {mismatches} ({} ion, {} neutral)",
            self.ion_mismatches, self.neutral_mismatches
        )?;
        writeln!(
            output,
            "Agreement within {TOLERANCE} Da: {:.2}%",
            100.0 * matches as f64 / total as f64
        )
    }

    fn write_mismatches(self, path: &Path) -> io::Result<()> {
        let mut output = BufWriter::new(File::create_new(path)?);
        writeln!(output, "# Top {TOP_MISMATCHES} Mass Mismatches\n")?;
        let mut table = Table::new();
        table.load_preset(presets::ASCII_MARKDOWN).set_header([
            "CID",
            "Formula",
            "PubChem Mass",
            "Calc. Mass",
        ]);
        for Reverse(entry) in self.largest.into_sorted_vec() {
            let row = entry.comparison;
            table.add_row([
                format!(
                    "[{compound_id}](https://pubchem.ncbi.nlm.nih.gov/compound/{compound_id})",
                    compound_id = row.compound_id
                ),
                format!("`{}`", row.formula),
                format!("{:.5}", row.expected_mass),
                format!("{:.5}", row.calculated_mass),
            ]);
        }
        writeln!(output, "{table}")?;
        output.flush()
    }
}

fn run() -> Result<(), ReportError> {
    let mut arguments = env::args_os().skip(1);
    let (Some(input), Some(output), None) = (arguments.next(), arguments.next(), arguments.next())
    else {
        return Err(ReportError::Usage);
    };
    let file = open_mass_file(Path::new(&input))?;
    let mut report = Report::default();
    let progress = ProgressBar::new_spinner();
    let start = Instant::now();
    compare_masses(file, |comparison| {
        report.record(comparison);
        progress.inc(1);
        Ok::<_, MassDataError>(())
    })?;
    let elapsed = start.elapsed();
    progress.finish_and_clear();
    report.write_summary(io::stdout().lock(), elapsed).map_err(ReportError::Summary)?;
    let path = PathBuf::from(output);
    report.write_mismatches(&path).map_err(|source| ReportError::Output { path, source })
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
