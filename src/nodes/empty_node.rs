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

#[cfg(test)]
mod tests {
    use alloc::format;

    use super::*;

    #[test]
    fn test_empty_display() {
        assert_eq!(format!("{Empty}"), "");
    }

    #[test]
    fn test_empty_try_from_char() {
        assert_eq!(Empty::try_from('a'), Err(()));
    }

    #[test]
    fn test_empty_traits() {
        let e1 = Empty;
        let e2 = Empty;
        assert_eq!(e1, e2);
        assert_eq!(e1.clone(), e2);
        assert_eq!(format!("{e1:?}"), "Empty");
    }

    #[cfg(feature = "fuzzing")]
    #[test]
    fn test_empty_arbitrary() {
        use arbitrary::{Arbitrary, Unstructured};

        let raw_data = [0u8; 10];
        let mut u = Unstructured::new(&raw_data);
        let e = Empty::arbitrary(&mut u).unwrap();
        assert_eq!(e, Empty);
    }
}
