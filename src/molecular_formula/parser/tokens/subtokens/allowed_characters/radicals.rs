//! Submodule defining the `Radical` struct marker.

use core::fmt::Display;

use super::super::{Baseline, CharacterMarker};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Marker for a Radical group in a molecular formula.
pub struct Radical;

impl From<Radical> for char {
    fn from(_: Radical) -> Self {
        Radical::CANONICAL
    }
}

impl CharacterMarker for Radical {
    type TS = Baseline;
    const CANONICAL: char = '•';
    fn matches(c: char) -> bool {
        c == Self::CANONICAL || c == '\u{00B7}' // middle dot
        || c == '\u{2219}' // bullet operator
        || c == '\u{2022}' // bullet
        || c == '\u{30FB}' // katakana middle dot
        || c == '\u{FF65}' // halfwidth katakana middle dot
        || c == '\u{0387}' // greek ano teleia
        || c == '\u{2027}' // hyphenation point
    }
}

impl Display for Radical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as CharacterMarker>::CANONICAL)
    }
}
