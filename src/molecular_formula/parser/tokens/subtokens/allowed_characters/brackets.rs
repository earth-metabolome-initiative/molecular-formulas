//! Module defining brackets as allowed characters in a molecular formula.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents the kind of brackets used in a molecular formula.
pub enum Bracket {
    /// Round brackets: ()
    Round,
    /// Square brackets: []
    Square,
}
