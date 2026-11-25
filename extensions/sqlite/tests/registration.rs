//! Exercises the registered function through SQLite's SQL interface.
#![cfg(not(feature = "loadable-extension"))]

use diesel::{
    connection::SimpleConnection,
    dsl::sql,
    prelude::*,
    sql_types::{Bool, Nullable, Text},
};
use molecular_formulas_sqlite::register_on_every_new_connection;

diesel::table! {
    samples (identifier) {
        identifier -> Integer,
        formula -> Nullable<Text>,
    }
}

#[diesel::declare_sql_function]
extern "SQL" {
    fn must_be_molecular_formula(formula: Nullable<Text>) -> Nullable<Bool>;
}

#[test]
fn validates_on_an_embedded_connection() {
    register_on_every_new_connection().unwrap();
    let connection = &mut SqliteConnection::establish(":memory:").unwrap();
    connection.set_trusted_schema(false).unwrap();
    connection
        .batch_execute(
            "CREATE TABLE samples (
                 identifier INTEGER PRIMARY KEY, formula TEXT CHECK (must_be_molecular_formula(formula))
             )",
        )
        .unwrap();
    for formula in [Some("H2O"), None] {
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
    assert_eq!(rows, [(Some("H2O".into()), Some(true)), (None, None)]);

    let error = diesel::insert_into(samples::table)
        .values(samples::formula.eq("·"))
        .execute(connection)
        .unwrap_err();
    assert_eq!(error.to_string(), "The molecular tree is empty.");
}

#[test]
fn rejects_invalid_sqlite_values() {
    register_on_every_new_connection().unwrap();
    let connection = &mut SqliteConnection::establish(":memory:").unwrap();
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
