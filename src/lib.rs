#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "fuzzing"), no_std)]

/// The crate is solely dependent on the alloc crate,
/// not the standard library.
extern crate alloc;

pub mod errors;
pub mod molecular_formula;
pub(crate) mod molecular_tree;
pub mod nodes;
pub mod parsable;
mod serde_impl;
mod utils;
pub use molecular_formula::*;
pub use molecular_tree::*;
pub use nodes::*;
pub use parsable::*;
pub(crate) use utils::{display_charge, display_isotope};
pub mod fuzzing;

/// Prelude module re-exporting commonly used items.
pub mod prelude {
    /// Re-exports from the elements_rs crate.
    pub use elements_rs::{Element, ElementVariant, Isotope, MassNumber};

    pub use crate::{molecular_formula::*, molecular_tree::*, nodes::*, parsable::*};
}
