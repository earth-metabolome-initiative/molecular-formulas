//! Connection-local SQLite functions for formula validation and search.

#![forbid(unsafe_code)]

use alloc::string::String;

use diesel::{
    deserialize::{self, FromSql},
    result::QueryResult,
    serialize::{self, Output, ToSql},
    sql_types::{Bool, Nullable, Text},
    sqlite::{Sqlite, SqliteConnection, SqliteFunctionBehavior, SqliteType, SqliteValue},
};

use crate::{
    errors::ParserError, sql::must_be_molecular_formula_utils, validate_molecular_formula,
};

#[diesel::declare_sql_function]
extern "SQL" {
    /// Projects a formula to distinct element and explicit isotope pairs.
    fn mf_presence(formula: Nullable<Text>) -> Nullable<Text>;
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
        Ok(Self(validate_molecular_formula(value.read_text())))
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

#[derive(Debug, diesel::deserialize::FromSqlRow)]
struct Presence(Result<String, ParserError>);

impl FromSql<Text, Sqlite> for Presence {
    fn from_sql(mut value: SqliteValue<'_, '_, '_>) -> deserialize::Result<Self> {
        if !matches!(value.value_type(), Some(SqliteType::Text)) {
            return Err("expected TEXT".into());
        }
        Ok(Self(crate::formula_index::presence_json(value.read_text())))
    }
}

impl ToSql<Text, Sqlite> for Presence {
    fn to_sql<'borrow>(
        &'borrow self,
        output: &mut Output<'borrow, '_, Sqlite>,
    ) -> serialize::Result {
        <String as ToSql<Text, Sqlite>>::to_sql(self.0.as_ref().map_err(|error| *error)?, output)
    }
}

/// Registers formula validation and presence projection on this connection.
///
/// # Errors
///
/// Returns the database error if SQLite refuses registration.
pub fn register(connection: &mut SqliteConnection) -> QueryResult<()> {
    let behavior = SqliteFunctionBehavior::DETERMINISTIC.union(SqliteFunctionBehavior::INNOCUOUS);
    must_be_molecular_formula_utils::register_impl_with_behavior(
        connection,
        behavior,
        core::convert::identity::<Option<FormulaValidation>>,
    )?;
    mf_presence_utils::register_impl_with_behavior(
        connection,
        behavior,
        core::convert::identity::<Option<Presence>>,
    )
}
