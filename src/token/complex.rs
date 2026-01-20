//! Submodules for tokens representing complex groups.

use std::str::FromStr;

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

impl FromStr for Complex {
    type Err = crate::errors::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Me" => Ok(Complex::Methyl),
            "Et" => Ok(Complex::Ethyl),
            "Bu" => Ok(Complex::Butyl),
            "Ph" => Ok(Complex::Phenyl),
            "Bn" => Ok(Complex::Benzyl),
            "Cy" => Ok(Complex::Cyclohexyl),
            "Cp" => Ok(Complex::Cyclopentadienyl),
            _ => Err(crate::errors::Error::InvalidComplexGroupFragment(s.to_string())),
        }
    }
}

impl TryFrom<[char; 2]> for Complex {
    type Error = crate::errors::Error;

    fn try_from(value: [char; 2]) -> Result<Self, Self::Error> {
        match value {
            ['M', 'e'] => Ok(Complex::Methyl),
            ['E', 't'] => Ok(Complex::Ethyl),
            ['B', 'u'] => Ok(Complex::Butyl),
            ['P', 'h'] => Ok(Complex::Phenyl),
            ['B', 'n'] => Ok(Complex::Benzyl),
            ['C', 'y'] => Ok(Complex::Cyclohexyl),
            ['C', 'p'] => Ok(Complex::Cyclopentadienyl),
            _ => Err(crate::errors::Error::InvalidComplexGroupFragment(value.iter().collect())),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_complex_from_str() {
        use std::str::FromStr;

        use super::Complex;

        assert_eq!(Complex::from_str("Me").unwrap(), Complex::Methyl);
        assert_eq!(Complex::from_str("Et").unwrap(), Complex::Ethyl);
        assert_eq!(Complex::from_str("Bu").unwrap(), Complex::Butyl);
        assert_eq!(Complex::from_str("Ph").unwrap(), Complex::Phenyl);
        assert_eq!(Complex::from_str("Bn").unwrap(), Complex::Benzyl);
        assert_eq!(Complex::from_str("Cy").unwrap(), Complex::Cyclohexyl);
        assert_eq!(Complex::from_str("Cp").unwrap(), Complex::Cyclopentadienyl);
        assert!(Complex::from_str("Xx").is_err());
    }

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
}
