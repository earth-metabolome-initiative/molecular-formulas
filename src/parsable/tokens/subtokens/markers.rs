//! Markers for plus and minus signs.

use core::fmt::Display;

use crate::{Baseline, BaselineDigit, Superscript, SuperscriptDigit, TypeSetting};

/// Marker trait.
pub trait CharacterMarker {
    /// The character representing the marker.
    const CANONICAL: char;
    /// The type setting of the character.
    type TS: TypeSetting;
    /// Returns whether the provided character matches the marker,
    /// include non-canonical representations.
    #[must_use]
    fn matches(c: char) -> bool {
        c == Self::CANONICAL
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enumeration of sign markers.
pub enum SignCharacter {
    /// Marker for the baseline plus sign.
    BaselinePlus,
    /// Marker for the baseline minus sign.
    BaselineMinus,
    /// Marker for the superscript plus sign.
    SuperscriptPlus,
    /// Marker for the superscript minus sign.
    SuperscriptMinus,
}

/// A trait defining a sign marker.
pub trait SignMarker: CharacterMarker {
    /// Whether the sign is positive.
    const POSITIVE: bool;
    /// The digit type associated with the sign.
    type Digit: TryFrom<char>;
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
    type Digit = BaselineDigit;
}

/// Marker for superscript plus sign.
pub struct SuperscriptPlus;

impl Display for SuperscriptPlus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", Self::CANONICAL)
    }
}

impl CharacterMarker for SuperscriptPlus {
    const CANONICAL: char = '⁺';
    type TS = Superscript;
}
impl SignMarker for SuperscriptPlus {
    const POSITIVE: bool = true;
    type Digit = SuperscriptDigit;
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
    type Digit = BaselineDigit;
}

/// Marker for superscript minus sign.
pub struct SuperscriptMinus;

impl Display for SuperscriptMinus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", Self::CANONICAL)
    }
}

impl CharacterMarker for SuperscriptMinus {
    const CANONICAL: char = '⁻';
    type TS = Superscript;
}

impl SignMarker for SuperscriptMinus {
    const POSITIVE: bool = false;
    type Digit = SuperscriptDigit;
}

/// Marker for the dot character.
pub struct Dot;

impl CharacterMarker for Dot {
    const CANONICAL: char = '.';
    type TS = Baseline;
    fn matches(c: char) -> bool {
        matches!(c, '.' | '\u{06d4}' | '\u{ff0e}' | '｡')
    }
}
