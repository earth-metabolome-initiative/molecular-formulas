//! Module defining brackets as allowed characters in a molecular formula.

use core::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents the kind of brackets used in a molecular formula.
pub enum Bracket {
    /// Round brackets: ()
    Round,
    /// Square brackets: []
    Square,
}

impl Display for Bracket {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Round => write!(f, "()"),
            Self::Square => write!(f, "[]"),
        }
    }
}

impl Bracket {
    /// Returns the opening bracket character.
    pub const fn opening(&self) -> char {
        match self {
            Self::Round => '(',
            Self::Square => '[',
        }
    }

    /// Returns the closing bracket character.
    pub const fn closing(&self) -> char {
        match self {
            Self::Round => ')',
            Self::Square => ']',
        }
    }
}

#[cfg(feature = "fuzzing")]
impl<'a> arbitrary::Arbitrary<'a> for Bracket {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        if u.ratio(1, 2)? {
            Ok(Bracket::Round)
        } else {
            Ok(Bracket::Square)
        }
    }
}
