//! Module defining brackets as allowed characters in a molecular formula.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
/// Represents the kind of brackets used in a molecular formula.
pub enum Bracket {
    /// Round brackets: ()
    Round,
    /// Square brackets: []
    Square,
}

impl Bracket {
    /// Returns the opening bracket character.
    #[must_use]
    pub const fn opening(&self) -> char {
        match self {
            Self::Round => '(',
            Self::Square => '[',
        }
    }

    /// Returns the closing bracket character.
    #[must_use]
    pub const fn closing(&self) -> char {
        match self {
            Self::Round => ')',
            Self::Square => ']',
        }
    }
}
