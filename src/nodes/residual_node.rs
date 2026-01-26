//! Submodule providing a struct and implementation of the `ExtensionTree` trait
//! for molecular formulas that can contain residuals such as `R`.

use core::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Marker for a residual group in a molecular formula.
pub struct Residual;

#[cfg(feature = "fuzzing")]
impl<'a> arbitrary::Arbitrary<'a> for Residual {
    fn arbitrary(_: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Residual)
    }
}

impl Display for Residual {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "R")
    }
}

impl From<Residual> for char {
    fn from(_: Residual) -> Self {
        'R'
    }
}

impl TryFrom<char> for Residual {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'R' => Ok(Residual),
            _ => Err(()),
        }
    }
}
