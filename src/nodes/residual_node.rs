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

impl TryFrom<char> for Residual {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'R' => Ok(Residual),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use super::*;

    #[test]
    fn test_residual_display() {
        assert_eq!(format!("{Residual}"), "R");
    }

    #[test]
    fn test_residual_try_from_char() {
        assert_eq!(Residual::try_from('R'), Ok(Residual));
        assert_eq!(Residual::try_from('A'), Err(()));
    }

    #[test]
    fn test_residual_traits() {
        let r1 = Residual;
        let r2 = Residual;
        assert_eq!(r1, r2);
        assert_eq!(r1.clone(), r2);
        assert_eq!(format!("{r1:?}"), "Residual");
    }

    #[cfg(feature = "fuzzing")]
    #[test]
    fn test_residual_arbitrary() {
        use arbitrary::{Arbitrary, Unstructured};

        let raw_data = [0u8; 10];
        let mut u = Unstructured::new(&raw_data);
        let r = Residual::arbitrary(&mut u).unwrap();
        assert_eq!(r, Residual);
    }
}
