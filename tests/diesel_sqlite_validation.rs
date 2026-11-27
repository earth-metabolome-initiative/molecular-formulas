//! Exercises the registered function through SQLite's SQL interface.

use diesel::{
    connection::SimpleConnection,
    dsl::sql,
    prelude::*,
    sql_types::{Nullable, Text},
};
use molecular_formulas::{sql::must_be_molecular_formula, sqlite::register};

diesel::table! {
    /// Samples with a formula constraint.
    samples (identifier) {
        /// Sample identifier.
        identifier -> Integer,
        /// Submitted molecular formula text.
        formula -> Nullable<Text>,
    }
}

#[test]
fn validates_on_an_embedded_connection() {
    let connection = &mut SqliteConnection::establish(":memory:").unwrap();
    let unregistered = &mut SqliteConnection::establish(":memory:").unwrap();
    register(connection).unwrap();
    let error = diesel::select(must_be_molecular_formula(Some("H2O")))
        .get_result::<Option<bool>>(unregistered)
        .unwrap_err();
    assert!(error.to_string().contains("no such function"));
    connection.set_trusted_schema(false).unwrap();
    connection
        .batch_execute(
            "CREATE TABLE samples (
                 identifier INTEGER PRIMARY KEY, formula TEXT CHECK (must_be_molecular_formula(formula))
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

    let overflow = diesel::update(samples::table)
        .set(samples::formula.eq("C65536"))
        .execute(connection)
        .unwrap_err();
    assert!(overflow.to_string().to_lowercase().contains("overflow"), "{overflow}");
    let stored = samples::table
        .order(samples::identifier)
        .select(samples::formula)
        .load::<Option<String>>(connection)
        .unwrap();
    assert_eq!(stored, formulas.map(|formula| formula.map(String::from)));
    connection
        .batch_execute("CREATE TABLE required (formula TEXT NOT NULL CHECK (must_be_molecular_formula(formula)))")
        .unwrap();
    assert!(connection.batch_execute("INSERT INTO required VALUES (NULL)").is_err());

    let error = diesel::insert_into(samples::table)
        .values(samples::formula.eq("·"))
        .execute(connection)
        .unwrap_err();
    assert_eq!(error.to_string(), "The molecular tree is empty.");
}

#[test]
fn validates_in_an_untrusted_generated_column() {
    let connection = &mut SqliteConnection::establish(":memory:").unwrap();
    connection.set_trusted_schema(false).unwrap();
    register(connection).unwrap();
    connection
        .batch_execute(
            "CREATE TABLE formulas (
                 formula TEXT, valid INTEGER GENERATED ALWAYS AS (must_be_molecular_formula(formula))
             );
             INSERT INTO formulas (formula) VALUES ('H2O')",
        )
        .unwrap();
    let error = connection.batch_execute("UPDATE formulas SET formula = '·'").unwrap_err();
    assert_eq!(error.to_string(), "The molecular tree is empty.");
}

#[test]
fn preserves_argument_encoding_errors() {
    #[derive(Debug, diesel::expression::AsExpression)]
    #[diesel(sql_type = Text)]
    struct FormulaBytes<'input>(&'input [u8]);

    impl diesel::serialize::ToSql<Text, diesel::sqlite::Sqlite> for FormulaBytes<'_> {
        fn to_sql<'bind>(
            &'bind self,
            output: &mut diesel::serialize::Output<'bind, '_, diesel::sqlite::Sqlite>,
        ) -> diesel::serialize::Result {
            output.set_value(std::str::from_utf8(self.0)?);
            Ok(diesel::serialize::IsNull::No)
        }
    }

    let connection = &mut SqliteConnection::establish(":memory:").unwrap();
    register(connection).unwrap();
    let error = diesel::select(must_be_molecular_formula(Some(FormulaBytes(b"H\xffO"))))
        .get_result::<Option<bool>>(connection)
        .unwrap_err();
    assert!(
        matches!(error, diesel::result::Error::SerializationError(source) if source.is::<std::str::Utf8Error>())
    );
}

#[test]
fn rejects_invalid_sqlite_values() {
    let connection = &mut SqliteConnection::establish(":memory:").unwrap();
    register(connection).unwrap();
    // SQL literals exercise storage classes that a typed text argument cannot
    // bind.
    for expression in
        ["x'48324f'", "1", "1.0", "CAST(x'ff' AS TEXT)", "CAST(x'48324f0021' AS TEXT)"]
    {
        let result = diesel::select(must_be_molecular_formula(sql::<Nullable<Text>>(expression)))
            .get_result::<Option<bool>>(connection);
        assert!(result.is_err(), "unexpected result for {expression}: {result:?}");
    }
}
