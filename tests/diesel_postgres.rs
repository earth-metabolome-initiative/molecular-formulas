//! Live PostgreSQL formula storage using `DATABASE_URL`.

use diesel::{Connection, PgConnection};

mod support;

#[test]
#[ignore = "requires a live PostgreSQL server and DATABASE_URL"]
fn postgres_stores_and_loads_formula_text() {
    let database_url = std::env::var("DATABASE_URL").expect("set DATABASE_URL for PostgreSQL");
    let mut connection = PgConnection::establish(&database_url).unwrap();
    support::check_formula_storage!(&mut connection);
}
