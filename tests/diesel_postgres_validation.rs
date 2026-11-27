//! Exercises formula constraints on an installed PostgreSQL extension.

use diesel::{
    connection::SimpleConnection,
    prelude::*,
    result::{DatabaseErrorKind, Error},
};
use molecular_formulas::sql::must_be_molecular_formula;

diesel::table! {
    /// Samples with a formula constraint.
    samples (identifier) {
        /// Preserves insertion order for the round-trip assertion.
        identifier -> Integer,
        /// Submitted formula text.
        formula -> Nullable<Text>,
    }
}

#[test]
#[ignore = "requires DATABASE_URL and the molecular_formulas PostgreSQL extension"]
fn validates_constrained_writes() {
    let database_url = std::env::var("DATABASE_URL").expect("set DATABASE_URL for PostgreSQL");
    let connection = &mut PgConnection::establish(&database_url).unwrap();
    connection
        .batch_execute(
            "CREATE EXTENSION IF NOT EXISTS molecular_formulas;
             CREATE TEMP TABLE samples (
                 identifier INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
                 formula TEXT CHECK (must_be_molecular_formula(formula))
             )",
        )
        .unwrap();

    let formulas = [
        Some("CuSO4.5H2O"),
        Some("SO₄²⁻"),
        Some("[13C]H4"),
        Some("[Co(NH3)5Cl]Cl2"),
        Some("C65535"),
        None,
    ];
    for formula in formulas {
        diesel::insert_into(samples::table)
            .values(samples::formula.eq(formula))
            .execute(connection)
            .unwrap();
    }
    for malformed in ["", "not a formula", "(OH", "C65536"] {
        let result = diesel::insert_into(samples::table)
            .values(samples::formula.eq(malformed))
            .execute(connection);
        assert!(matches!(result, Err(Error::DatabaseError(..))), "{malformed:?}: {result:?}");
    }
    let overflow = diesel::update(samples::table)
        .set(samples::formula.eq("C65536"))
        .execute(connection)
        .unwrap_err();
    assert!(overflow.to_string().to_lowercase().contains("overflow"), "{overflow}");

    let rows = samples::table
        .order(samples::identifier)
        .select((samples::formula, must_be_molecular_formula(samples::formula)))
        .load::<(Option<String>, Option<bool>)>(connection)
        .unwrap();
    assert_eq!(rows, formulas.map(|formula| (formula.map(String::from), formula.map(|_| true))));
    let error = diesel::select(must_be_molecular_formula(Some("C65536")))
        .get_result::<Option<bool>>(connection)
        .unwrap_err();
    assert!(error.to_string().to_lowercase().contains("overflow"), "{error}");

    connection
        .batch_execute(
            "CREATE TEMP TABLE required_samples (
                 formula TEXT NOT NULL CHECK (must_be_molecular_formula(formula))
             )",
        )
        .unwrap();
    let result = connection.batch_execute("INSERT INTO required_samples VALUES (NULL)");
    assert!(matches!(result, Err(Error::DatabaseError(DatabaseErrorKind::NotNullViolation, _))));
}
