//! Element and isotope presence on PostgreSQL with the extension installed.

use diesel::{connection::SimpleConnection, prelude::*};
use molecular_formulas::{formula_index::POSTGRES_SCHEMA, prelude::Element};

#[path = "presence/cases.rs"]
mod cases;

type TestConnection = PgConnection;

#[test]
#[ignore = "requires PostgreSQL with the molecular_formulas extension and DATABASE_URL"]
fn searches_and_maintains_presence_across_search_paths() {
    let database_url = std::env::var("DATABASE_URL").expect("set DATABASE_URL for PostgreSQL");
    let connection = &mut PgConnection::establish(&database_url).unwrap();
    connection.begin_test_transaction().unwrap();
    connection
        .batch_execute(
            "CREATE EXTENSION IF NOT EXISTS molecular_formulas;
             CREATE SCHEMA mf_presence_test;
             SET LOCAL search_path TO mf_presence_test, public",
        )
        .unwrap();
    connection.batch_execute(POSTGRES_SCHEMA).unwrap();
    cases::check_presence(connection);
    connection
        .batch_execute(
            "CREATE TEMP TABLE mf_postings (decoy integer);
             SET LOCAL search_path TO pg_temp, pg_catalog;
             UPDATE mf_presence_test.mf_documents SET formula = 'F' WHERE id = 40;
             SET LOCAL search_path TO mf_presence_test, public, pg_temp",
        )
        .unwrap();
    assert_eq!(cases::element(connection, Element::F), [40]);
    assert!(cases::element(connection, Element::Fe).is_empty());
    connection
        .batch_execute(
            "SET LOCAL search_path TO pg_temp, pg_catalog;
             TRUNCATE mf_presence_test.mf_documents;
             SET LOCAL search_path TO mf_presence_test, public, pg_temp",
        )
        .unwrap();
    assert!(cases::element(connection, Element::C).is_empty());
    assert!(cases::element(connection, Element::F).is_empty());
}
