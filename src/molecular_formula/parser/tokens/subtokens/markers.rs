//! Submodule defining struct markers for token iteration and associated traits.

mod sign_markers;

pub use sign_markers::*;

use super::TypeSetting;

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
