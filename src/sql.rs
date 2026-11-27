//! Typed Diesel queries for formula validation on SQLite and PostgreSQL.

use diesel::sql_types::{Bool, Nullable, Text};

#[diesel::declare_sql_function]
extern "SQL" {
    /// Returns true for valid formula text and SQL `NULL` for missing input.
    /// Malformed text raises a database error.
    fn must_be_molecular_formula(formula: Nullable<Text>) -> Nullable<Bool>;
}
