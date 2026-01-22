//! Submodules for tokens representing complex groups.

use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents complex group fragments in molecular formulas.
pub enum Complex {
    /// Methyl group (Me) - CH3
    Methyl,
    /// Ethyl group (Et) - C2H5
    Ethyl,
    /// Butyl group (Bu) - C4H9
    Butyl,
    /// Phenyl group (Ph) - C6H5
    Phenyl,
    /// Benzyl group (Bn) - C7H7
    Benzyl,
    /// Cyclohexyl group (Cy) - C6H11
    Cyclohexyl,
    /// Cyclopentadienyl group (Cp) - `C5H5-`
    Cyclopentadienyl,
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Complex::Methyl => "Me",
            Complex::Ethyl => "Et",
            Complex::Butyl => "Bu",
            Complex::Phenyl => "Ph",
            Complex::Benzyl => "Bn",
            Complex::Cyclohexyl => "Cy",
            Complex::Cyclopentadienyl => "Cp",
        };
        write!(f, "{s}")
    }
}

impl TryFrom<[char; 2]> for Complex {
    type Error = ();

    fn try_from(value: [char; 2]) -> Result<Self, Self::Error> {
        match value {
            ['M', 'e'] => Ok(Complex::Methyl),
            ['E', 't'] => Ok(Complex::Ethyl),
            ['B', 'u'] => Ok(Complex::Butyl),
            ['P', 'h'] => Ok(Complex::Phenyl),
            ['B', 'n'] => Ok(Complex::Benzyl),
            ['C', 'y'] => Ok(Complex::Cyclohexyl),
            ['C', 'p'] => Ok(Complex::Cyclopentadienyl),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_complex_try_from_char_array() {
        use std::convert::TryFrom;

        use super::Complex;

        assert_eq!(Complex::try_from(['M', 'e']).unwrap(), Complex::Methyl);
        assert_eq!(Complex::try_from(['E', 't']).unwrap(), Complex::Ethyl);
        assert_eq!(Complex::try_from(['B', 'u']).unwrap(), Complex::Butyl);
        assert_eq!(Complex::try_from(['P', 'h']).unwrap(), Complex::Phenyl);
        assert_eq!(Complex::try_from(['B', 'n']).unwrap(), Complex::Benzyl);
        assert_eq!(Complex::try_from(['C', 'y']).unwrap(), Complex::Cyclohexyl);
        assert_eq!(Complex::try_from(['C', 'p']).unwrap(), Complex::Cyclopentadienyl);
        assert!(Complex::try_from(['X', 'x']).is_err());
    }

    #[test]
    fn test_complex_display() {
        use super::Complex;

        assert_eq!(Complex::Methyl.to_string(), "Me");
        assert_eq!(Complex::Ethyl.to_string(), "Et");
        assert_eq!(Complex::Butyl.to_string(), "Bu");
        assert_eq!(Complex::Phenyl.to_string(), "Ph");
        assert_eq!(Complex::Benzyl.to_string(), "Bn");
        assert_eq!(Complex::Cyclohexyl.to_string(), "Cy");
        assert_eq!(Complex::Cyclopentadienyl.to_string(), "Cp");
    }
}
