//! Markers for plus and minus signs.

use super::{
    super::{Baseline, Superscript},
    CharacterMarker,
};

/// A trait defining a sign marker.
pub trait SignMarker: CharacterMarker {
    const POSITIVE: bool;
}

/// Marker for Baseline plus sign.
pub struct BaselinePlus;

impl CharacterMarker for BaselinePlus {
    const CANONICAL: char = '+';
    type TS = Baseline;
    fn matches(c: char) -> bool {
        c == Self::CANONICAL || c == '\u{FF0B}' // Fullwidth plus sign
    }
}

impl SignMarker for BaselinePlus {
    const POSITIVE: bool = true;
}

/// Marker for superscript plus sign.
pub struct SuperscriptPlus;

impl CharacterMarker for SuperscriptPlus {
    const CANONICAL: char = '⁺';
    type TS = Superscript;
}
impl SignMarker for SuperscriptPlus {
    const POSITIVE: bool = true;
}

/// Marker for Baseline minus sign.
pub struct BaselineMinus;

impl CharacterMarker for BaselineMinus {
    const CANONICAL: char = '-';
    type TS = Baseline;
    fn matches(c: char) -> bool {
        c == Self::CANONICAL
            || c == '\u{2212}' // Unicode minus sign
            || c == '\u{FF0D}' // Fullwidth minus sign
            || c == '\u{2010}' // Hyphen
            || c == '\u{2011}' // Non-breaking hyphen
            || c == '\u{2012}' // Figure dash
            || c == '\u{2013}' // En dash
            || c == '\u{2014}' // Em dash
            || c == '\u{2015}' // Horizontal bar
            || c == '\u{FE63}' // Small hyphen-minus
    }
}

impl SignMarker for BaselineMinus {
    const POSITIVE: bool = false;
}

/// Marker for superscript minus sign.
pub struct SuperscriptMinus;

impl CharacterMarker for SuperscriptMinus {
    const CANONICAL: char = '⁻';
    type TS = Superscript;
}

impl SignMarker for SuperscriptMinus {
    const POSITIVE: bool = false;
}
