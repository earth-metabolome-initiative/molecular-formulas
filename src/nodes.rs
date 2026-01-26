//! Submodule defining an `ExtensionTree` trait, including its associated types
//! regarding parsing. For instance, the `ChargeExtensionTree` trait defines the
//! tokens, subtokens and allowed characters for formulas which can contain
//! charges, which is common in several contexts but forbidden in InChI strings.

mod bracket_node;
mod charge_node;
mod element_node;
mod empty_node;
mod isotope_node;
mod radical_node;
mod repeat_node;
mod residual_node;
mod sequence_node;

pub(crate) use bracket_node::BracketNode;
pub(crate) use charge_node::ChargeNode;
pub(crate) use radical_node::{Radical, RadicalNode};
pub(crate) use repeat_node::RepeatNode;
pub use residual_node::Residual;
pub(crate) use sequence_node::SequenceNode;
pub(crate) use empty_node::Empty;

/// Trait defining an extension tree for molecular formulas.
pub trait Node: Sized {}

/// Marker trait indicating that a node supports a given extension.
pub trait Supports<N: Node> {}
