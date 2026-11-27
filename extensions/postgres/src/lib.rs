//! PostgreSQL validation of molecular formula text.

pub use formula_parser::errors::ParserError as Error;
use pgrx_validation_derive::validation;

pgrx::pg_module_magic!(name, version);

#[validation]
/// Validates the input with the default chemical formula parser.
///
/// # Errors
///
/// Returns the parser error when the input is malformed.
pub fn must_be_molecular_formula(formula: &str) -> Result<(), crate::Error> {
    formula_parser::validate_molecular_formula(formula)
}
