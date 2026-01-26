//! Marker type for an empty node and its trait implementations.

use core::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Marker type for an empty node.
pub struct Empty;

impl Display for Empty {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "")
    }
}

#[cfg(feature = "fuzzing")]
impl<'a> arbitrary::Arbitrary<'a> for Empty {
    fn arbitrary(_: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Empty)
    }
}

impl TryFrom<char> for Empty {
    type Error = ();

    fn try_from(_value: char) -> Result<Self, Self::Error> {
        Err(())
    }
}
