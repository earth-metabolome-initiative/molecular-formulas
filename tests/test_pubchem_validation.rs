//! Strict mass agreement checks against headerless PubChem `CID-Mass` records.

use std::{io::Read, path::Path};

#[path = "support/pubchem_mass.rs"]
mod pubchem_mass;

use pubchem_mass::{MassComparison, MassDataError, compare_masses, open_mass_file};

#[derive(Debug, thiserror::Error)]
enum ValidationError {
    #[error(transparent)]
    Input(#[from] MassDataError),
    #[error(
        "CID {} ({}) expected {} Da, calculated {} Da (tolerance {} Da)",
        .0.compound_id, .0.formula, .0.expected_mass, .0.calculated_mass, pubchem_mass::TOLERANCE
    )]
    Disagreement(MassComparison),
}

fn validate_masses(input: impl Read) -> Result<(), ValidationError> {
    compare_masses(input, |comparison| {
        if comparison.agrees() { Ok(()) } else { Err(ValidationError::Disagreement(comparison)) }
    })
}

fn validate_mass_file(path: &Path) -> Result<(), ValidationError> {
    validate_masses(open_mass_file(path)?)
}

#[test]
fn pinned_pubchem_masses_agree() {
    validate_masses(include_bytes!("fixtures/pubchem-masses.tsv").as_slice())
        .unwrap_or_else(|error| panic!("{error}"));
}

#[test]
#[ignore = "requires the full CID-Mass file in the crate root"]
fn test_pubchem_validation() {
    validate_mass_file(Path::new("CID-Mass")).unwrap_or_else(|error| panic!("{error}"));
}

#[test]
fn wrong_reference_mass_identifies_the_compound_and_both_masses() {
    let input = b"1\tC\t12\t12\n2\tC2\t25\t24\n";
    let error = validate_masses(input.as_slice()).unwrap_err().to_string();
    assert!(error.contains("CID 2"), "{error}");
    assert!(error.contains("expected 25 Da"), "{error}");
    assert!(error.contains("calculated 24 Da"), "{error}");
}

#[test]
fn tolerance_includes_both_endpoints() {
    validate_masses(b"1\tC2\t24.001\t24\n2\tC2\t23.999\t24\n".as_slice()).unwrap();
    for mass in ["24.00101", "23.99899"] {
        let input = format!("1\tC2\t{mass}\t24\n");
        assert!(matches!(validate_masses(input.as_bytes()), Err(ValidationError::Disagreement(_))));
    }
}

#[test]
fn empty_mass_input_is_rejected() {
    assert!(matches!(
        validate_masses(b"".as_slice()),
        Err(ValidationError::Input(MassDataError::Empty))
    ));
}

#[test]
fn invalid_mass_records_are_rejected() {
    for invalid_row in [
        "bad\tC\t12\t12",
        "2\t!\t12\t12",
        "2\tC\tbad\t12",
        "2\tC\t12\tbad",
        "2\tC\tNaN\t12",
        "2\tC\t12\tinf",
        "2\tC\t12",
        "2\tC\t12\t12\textra",
    ] {
        assert!(
            matches!(validate_masses(invalid_row.as_bytes()), Err(ValidationError::Input(_))),
            "accepted invalid row {invalid_row:?}"
        );
    }
}

#[test]
fn missing_mass_file_is_rejected() {
    let path = Path::new("tests/fixtures/missing-CID-Mass");
    let error = validate_mass_file(path).unwrap_err();
    assert!(
        matches!(&error, ValidationError::Input(MassDataError::Open { source, .. }) if source.kind() == std::io::ErrorKind::NotFound),
        "{error}"
    );
}
