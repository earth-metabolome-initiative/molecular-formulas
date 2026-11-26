//! Element and isotope presence on an embedded SQLite connection.

use diesel::{connection::SimpleConnection, prelude::*};
use molecular_formulas::{formula_index::SQLITE_SCHEMA, prelude::Element, sqlite::register};

#[path = "presence/cases.rs"]
mod cases;

type TestConnection = SqliteConnection;

fn database() -> SqliteConnection {
    let mut connection = SqliteConnection::establish(":memory:").unwrap();
    register(&mut connection).unwrap();
    connection.set_trusted_schema(false).unwrap();
    connection.batch_execute("PRAGMA foreign_keys = OFF; PRAGMA recursive_triggers = OFF").unwrap();
    connection.batch_execute(SQLITE_SCHEMA).unwrap();
    connection
}

#[test]
fn searches_and_maintains_presence() {
    cases::check_presence(&mut database());
}

#[test]
fn replacement_clears_displaced_postings_without_recursive_triggers() {
    let connection = &mut database();
    connection
        .batch_execute(
            "INSERT INTO mf_documents VALUES (1, 'C'), (2, 'Fe');
             INSERT OR REPLACE INTO mf_documents VALUES (1, 'F')",
        )
        .unwrap();
    assert!(cases::element(connection, Element::C).is_empty());
    assert_eq!(cases::element(connection, Element::F), [1]);
    assert!(
        connection.batch_execute("INSERT OR REPLACE INTO mf_documents VALUES (1, x'48')").is_err()
    );
    assert_eq!(cases::element(connection, Element::F), [1]);
    connection.batch_execute("UPDATE OR REPLACE mf_documents SET id = 1 WHERE id = 2").unwrap();
    assert_eq!(cases::element(connection, Element::Fe), [1]);
    assert!(cases::element(connection, Element::F).is_empty());
}
