//! SQLite registration for the `must_be_molecular_formula` scalar function.

#![forbid(unsafe_code)]

use diesel::{
    deserialize::{self, FromSql},
    result::QueryResult,
    serialize::{self, Output, ToSql},
    sql_types::{Bool, Nullable, Text},
    sqlite::{Sqlite, SqliteConnection, SqliteFunctionBehavior, SqliteType, SqliteValue},
};

use crate::{ChemicalFormula, errors::ParserError};

#[diesel::declare_sql_function]
extern "SQL" {
    /// Validates formula text, propagating missing values and parser errors.
    fn must_be_molecular_formula(formula: Nullable<Text>) -> Nullable<Bool>;
}

/// Reports parser errors when Diesel writes the SQL result so callers see the
/// cause.
#[derive(Debug, diesel::deserialize::FromSqlRow)]
struct FormulaValidation(Result<(), ParserError>);

impl FromSql<Text, Sqlite> for FormulaValidation {
    fn from_sql(mut value: SqliteValue<'_, '_, '_>) -> deserialize::Result<Self> {
        if !matches!(value.value_type(), Some(SqliteType::Text)) {
            return Err("expected TEXT".into());
        }
        Ok(Self(value.read_text().parse::<ChemicalFormula>().map(drop)))
    }
}

impl ToSql<Bool, Sqlite> for FormulaValidation {
    fn to_sql<'borrow>(
        &'borrow self,
        output: &mut Output<'borrow, '_, Sqlite>,
    ) -> serialize::Result {
        self.0?;
        <bool as ToSql<Bool, Sqlite>>::to_sql(&true, output)
    }
}

/// Registers `must_be_molecular_formula` on this connection.
///
/// # Errors
///
/// Returns the database error if SQLite refuses registration.
pub fn register(connection: &mut SqliteConnection) -> QueryResult<()> {
    must_be_molecular_formula_utils::register_impl_with_behavior(
        connection,
        SqliteFunctionBehavior::DETERMINISTIC.union(SqliteFunctionBehavior::INNOCUOUS),
        core::convert::identity::<Option<FormulaValidation>>,
    )
}
