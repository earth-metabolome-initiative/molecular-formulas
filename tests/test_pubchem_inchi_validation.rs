//! Validate molecular formulas in the uncompressed PubChem CID-InChI-Key
//! export.
use core::{num::ParseIntError, str::FromStr};
use std::{fs::File, io::Read, path::Path};

use csv::{ReaderBuilder, StringRecord};
use molecular_formulas::prelude::InChIFormula;
use thiserror::Error;

#[derive(Debug, Error)]
enum ValidationError {
    #[error("cannot read record {record}: {source}")]
    Record { record: u64, source: csv::Error },
    #[error("record {record} must have three tab-separated fields and a nonempty InChIKey")]
    Fields { record: u64 },
    #[error("invalid CID in record {record}: {source}")]
    CompoundId { record: u64, source: ParseIntError },
    #[error("invalid or missing InChI formula layer for CID {cid}: {inchi:?}")]
    Layer { cid: u64, inchi: String },
    #[error("invalid formula {formula:?} for CID {cid}: {source}")]
    Formula { cid: u64, formula: String, source: <InChIFormula as FromStr>::Err },
    #[error("input contains no usable InChI formulas")]
    NoFormulas,
}

fn validate_pubchem_inchi(input: impl Read) -> Result<u64, ValidationError> {
    let mut reader = ReaderBuilder::new().delimiter(b'\t').has_headers(false).from_reader(input);
    let mut fields = StringRecord::new();
    let mut parsed = 0;
    for record in 1.. {
        if !reader
            .read_record(&mut fields)
            .map_err(|source| ValidationError::Record { record, source })?
        {
            break;
        }
        if fields.len() != 3 || fields[2].is_empty() {
            return Err(ValidationError::Fields { record });
        }
        let cid =
            fields[0].parse().map_err(|source| ValidationError::CompoundId { record, source })?;
        let inchi = &fields[1];
        let layer_error = || ValidationError::Layer { cid, inchi: inchi.to_owned() };
        let layers = inchi.strip_prefix("InChI=1S/").ok_or_else(layer_error)?;
        let mut parts = layers.split('/');
        let formula = parts.next().filter(|layer| !layer.is_empty()).ok_or_else(layer_error)?;
        if let Some(count) = formula.strip_prefix("p+").or_else(|| formula.strip_prefix("p-"))
            && !count.is_empty()
            && count.bytes().all(|digit| digit.is_ascii_digit())
            && count.bytes().any(|digit| digit != b'0')
        {
            continue;
        }
        formula.parse::<InChIFormula>().map_err(|source| {
            ValidationError::Formula { cid, formula: formula.to_owned(), source }
        })?;
        parsed += 1;
    }
    if parsed == 0 {
        return Err(ValidationError::NoFormulas);
    }
    Ok(parsed)
}

#[test]
#[ignore = "requires the full uncompressed PubChem CID-InChI-Key export"]
fn test_pubchem_validation() {
    let path = Path::new("CID-InChI-Key");
    let file = File::open(path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    let parsed =
        validate_pubchem_inchi(file).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    println!("Validated {parsed} PubChem InChI formulas");
}

#[test]
fn validates_formula_alongside_proton_only_record() {
    let records = "962\tInChI=1S/H2O/h1H2\tXLYOFNOQVPJJNP-UHFFFAOYSA-N\n\
                   1038\tInChI=1S/p+1\tGPRLSGONYQIRFK-UHFFFAOYSA-N\n";
    assert_eq!(validate_pubchem_inchi(records.as_bytes()).unwrap(), 1);
}

#[test]
fn rejects_empty_and_proton_only_inputs() {
    for records in ["", "1038\tInChI=1S/p+1\tGPRLSGONYQIRFK-UHFFFAOYSA-N\n"] {
        assert!(matches!(
            validate_pubchem_inchi(records.as_bytes()),
            Err(ValidationError::NoFormulas)
        ));
    }
}

#[test]
fn reports_malformed_records_and_formulas() {
    let valid = "962\tInChI=1S/H2O/h1H2\tXLYOFNOQVPJJNP-UHFFFAOYSA-N\n";
    for (invalid, diagnostic) in [
        ("1038\tmissing-key\n", "record 2"),
        ("1038\tH2O\tkey\n", "CID 1038"),
        ("1038\tInChI=1S/potato\tkey\n", "CID 1038"),
        ("1038\tInChI=1S/\tkey\n", "CID 1038"),
    ] {
        let records = format!("{valid}{invalid}");
        let error = validate_pubchem_inchi(records.as_bytes()).unwrap_err();
        assert!(error.to_string().contains(diagnostic), "{error}");
    }
}
