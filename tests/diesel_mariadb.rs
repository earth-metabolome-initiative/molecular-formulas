//! Live MariaDB formula storage using `MARIADB_DATABASE_URL`.

use diesel::{mariadb::MariadbConnection, prelude::*};

mod support;

#[test]
#[ignore = "requires a live MariaDB server and MARIADB_DATABASE_URL"]
fn mariadb_stores_and_loads_formula_text() {
    let database_url = std::env::var("MARIADB_DATABASE_URL").expect("set MARIADB_DATABASE_URL");
    let mut connection = MariadbConnection::establish(&database_url).unwrap();
    let version: String = diesel::select(support::version()).get_result(&mut connection).unwrap();
    assert!(version.to_ascii_lowercase().contains("mariadb"), "expected MariaDB, got {version}");
    support::check_formula_storage!(&mut connection);
}
