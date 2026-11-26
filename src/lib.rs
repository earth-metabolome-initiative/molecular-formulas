#![doc = include_str!("../README.md")]
#![cfg_attr(
    not(any(
        feature = "fuzzing",
        feature = "mem_dbg",
        feature = "diesel-postgres",
        feature = "diesel-mariadb",
        feature = "diesel-mysql"
    )),
    no_std
)]

/// Heap allocation in `no_std` builds.
extern crate alloc;

#[cfg(any(
    feature = "diesel-sqlite",
    feature = "diesel-postgres",
    feature = "diesel-mariadb",
    feature = "diesel-mysql"
))]
mod diesel_sql;
pub mod errors;
#[cfg(feature = "formula-index")]
pub mod formula_index;
#[cfg(any(feature = "diesel-sqlite", feature = "diesel-postgres"))]
pub mod index_queries;
pub mod molecular_formula;
pub(crate) mod molecular_tree;
pub mod nodes;
pub mod parsable;
mod serde_impl;
#[cfg(any(feature = "diesel-postgres", feature = "diesel-sqlite"))]
pub mod sql;
#[cfg(feature = "diesel-sqlite")]
pub mod sqlite;
mod utils;
pub use molecular_formula::*;
pub use molecular_tree::*;
pub use nodes::*;
pub use parsable::*;
pub(crate) use utils::{display_charge, display_isotope};
pub mod fuzzing;

/// Validates text using the default [`ChemicalFormula`] count and charge types.
///
/// # Errors
///
/// Returns the parser error for invalid syntax, counts, or charges.
pub fn validate_molecular_formula(formula: &str) -> Result<(), errors::ParserError> {
    formula.parse::<ChemicalFormula>().map(drop)
}

/// Prelude module re-exporting commonly used items.
pub mod prelude {
    /// Re-exports from the elements_rs crate.
    pub use elements_rs::{Element, ElementVariant, Isotope, MassNumber};

    pub use crate::{molecular_formula::*, molecular_tree::*, nodes::*, parsable::*};
}
