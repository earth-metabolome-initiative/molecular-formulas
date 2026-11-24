//! Diesel `TEXT` storage for molecular formulas in SQLite.
#![cfg(feature = "diesel-sqlite")]

use diesel::{Connection, SqliteConnection};

mod support;

#[test]
fn sqlite_stores_and_loads_formula_text() {
    let mut connection = SqliteConnection::establish(":memory:").unwrap();
    support::check_formula_storage!(&mut connection);
}
