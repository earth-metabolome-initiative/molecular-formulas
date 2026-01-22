//! Submodule defining the `Residual` struct marker.

use core::fmt::Display;

use super::{Baseline, CharacterMarker};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Marker for a residual group in a molecular formula.
pub struct Residual;

impl From<Residual> for char {
    fn from(_: Residual) -> Self {
        Residual::CANONICAL
    }
}

impl CharacterMarker for Residual {
    type TS = Baseline;
    const CANONICAL: char = 'R';
}

impl Display for Residual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as CharacterMarker>::CANONICAL)
    }
}
