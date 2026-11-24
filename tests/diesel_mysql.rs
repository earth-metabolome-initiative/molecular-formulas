//! Live MySQL formula storage using `MYSQL_DATABASE_URL`.

use diesel::{mysql::MysqlConnection, prelude::*};

mod support;

#[test]
#[ignore = "requires a live MySQL server and MYSQL_DATABASE_URL"]
fn mysql_stores_and_loads_formula_text() {
    let database_url = std::env::var("MYSQL_DATABASE_URL").expect("set MYSQL_DATABASE_URL");
    let mut connection = MysqlConnection::establish(&database_url).unwrap();
    let version: String = diesel::select(support::version()).get_result(&mut connection).unwrap();
    assert!(!version.to_ascii_lowercase().contains("mariadb"), "expected MySQL, got {version}");
    support::check_formula_storage!(&mut connection);
}
