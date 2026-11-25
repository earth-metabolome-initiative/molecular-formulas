//! Decode PubChem `CID-Mass` records and compare their monoisotopic masses.

use std::{
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
};

use csv::{ReaderBuilder, StringRecord};
use molecular_formulas::{errors::ParserError, prelude::*};

pub(super) const TOLERANCE: f64 = 0.001;

#[derive(Debug, thiserror::Error)]
pub(super) enum MassDataError {
    #[error("could not open {}: {source}", path.display())]
    Open { path: PathBuf, source: io::Error },
    #[error("invalid CID-Mass record: {0}")]
    Record(#[from] csv::Error),
    #[error("CID-Mass record {record} has {fields} fields, expected four")]
    Fields { record: u64, fields: usize },
    #[error("CID {compound_id} has an invalid formula: {source}")]
    Formula { compound_id: u64, source: ParserError },
    #[error(
        "CID {compound_id} has a non-finite mass (expected {expected_mass} Da, calculated {calculated_mass} Da, exact {exact_mass} Da)"
    )]
    NonFinite { compound_id: u64, expected_mass: f64, calculated_mass: f64, exact_mass: f64 },
    #[error("CID-Mass input contains no records")]
    Empty,
}

pub(super) fn open_mass_file(path: &Path) -> Result<File, MassDataError> {
    File::open(path).map_err(|source| MassDataError::Open { path: path.to_owned(), source })
}

#[derive(Debug)]
pub(super) struct MassComparison {
    pub(super) compound_id: u64,
    pub(super) formula: ChemicalFormula,
    pub(super) expected_mass: f64,
    pub(super) calculated_mass: f64,
}

impl MassComparison {
    pub(super) fn agrees(&self) -> bool {
        (self.calculated_mass - TOLERANCE..=self.calculated_mass + TOLERANCE)
            .contains(&self.expected_mass)
    }
}

fn decode_mass(record: &StringRecord, record_number: u64) -> Result<MassComparison, MassDataError> {
    if record.len() != 4 {
        return Err(MassDataError::Fields { record: record_number, fields: record.len() });
    }
    let (compound_id, formula, expected_mass, exact_mass): (u64, &str, f64, f64) =
        record.deserialize(None)?;
    let formula: ChemicalFormula =
        formula.parse().map_err(|source| MassDataError::Formula { compound_id, source })?;
    let calculated_mass = formula.isotopologue_mass();
    if !expected_mass.is_finite() || !exact_mass.is_finite() || !calculated_mass.is_finite() {
        return Err(MassDataError::NonFinite {
            compound_id,
            expected_mass,
            calculated_mass,
            exact_mass,
        });
    }
    Ok(MassComparison { compound_id, formula, expected_mass, calculated_mass })
}

pub(super) fn compare_masses<Error: From<MassDataError>>(
    input: impl Read,
    mut visit: impl FnMut(MassComparison) -> Result<(), Error>,
) -> Result<(), Error> {
    let mut reader =
        ReaderBuilder::new().has_headers(false).delimiter(b'\t').quoting(false).from_reader(input);
    let mut record = StringRecord::new();
    let mut records = 0;
    while reader.read_record(&mut record).map_err(MassDataError::from)? {
        records += 1;
        visit(decode_mass(&record, records)?)?;
    }
    if records == 0 {
        return Err(MassDataError::Empty.into());
    }
    Ok(())
}
