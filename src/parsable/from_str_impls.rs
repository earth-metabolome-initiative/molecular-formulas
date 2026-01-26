//! Implementations of the `FromStr` trait for various parsable molecular
//! formula formats.

use core::str::FromStr;

use elements_rs::Isotope;

use crate::{
    ChargeLike, ChemicalFormula, CountLike, InChIFormula, MineralFormula, ResidualFormula,
    errors::ParserError, parsable::MoleculeParser,
};

impl<Count: CountLike> FromStr for InChIFormula<Count> {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        MoleculeParser::new(s.chars())?.parse_formula()
    }
}

impl<Count: CountLike, Charge: ChargeLike> TryFrom<&str> for ChemicalFormula<Count, Charge>
where
    Isotope: TryFrom<(elements_rs::Element, Count), Error = elements_rs::errors::Error>,
    Charge: TryFrom<Count>,
{
    type Error = ParserError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        MoleculeParser::new(s.chars())?.parse_formula()
    }
}

impl<Count: CountLike, Charge: ChargeLike> FromStr for ChemicalFormula<Count, Charge>
where
    Isotope: TryFrom<(elements_rs::Element, Count), Error = elements_rs::errors::Error>,
    Charge: TryFrom<Count>,
{
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        MoleculeParser::new(s.chars())?.parse_formula()
    }
}

impl<Count: CountLike, Charge: ChargeLike> TryFrom<&str> for MineralFormula<Count, Charge>
where
    Isotope: TryFrom<(elements_rs::Element, Count), Error = elements_rs::errors::Error>,
    Charge: TryFrom<Count>,
{
    type Error = ParserError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        MoleculeParser::new(s.chars())?.parse_formula()
    }
}

impl<Count: CountLike, Charge: ChargeLike> FromStr for MineralFormula<Count, Charge>
where
    Isotope: TryFrom<(elements_rs::Element, Count), Error = elements_rs::errors::Error>,
    Charge: TryFrom<Count>,
{
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        MoleculeParser::new(s.chars())?.parse_formula()
    }
}

impl<Count: CountLike, Charge: ChargeLike> TryFrom<&str> for ResidualFormula<Count, Charge>
where
    Isotope: TryFrom<(elements_rs::Element, Count), Error = elements_rs::errors::Error>,
    Charge: TryFrom<Count>,
{
    type Error = ParserError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        MoleculeParser::new(s.chars())?.parse_formula()
    }
}

impl<Count: CountLike, Charge: ChargeLike> FromStr for ResidualFormula<Count, Charge>
where
    Isotope: TryFrom<(elements_rs::Element, Count), Error = elements_rs::errors::Error>,
    Charge: TryFrom<Count>,
{
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        MoleculeParser::new(s.chars())?.parse_formula()
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_inchi_formula_from_str() {
        let input = "C2H5";
        let expected = "1C2H5"; // InChI formula explicitly prints count
        let formula = InChIFormula::<u32>::from_str(input).unwrap();
        assert_eq!(formula.to_string(), expected);
    }

    #[test]
    fn test_chemical_formula_try_from() {
        let input = "H2O";
        let expected = "H₂O"; // Chemical formula uses subscripts
        let formula = ChemicalFormula::<u32, i32>::try_from(input).unwrap();
        assert_eq!(formula.to_string(), expected);
    }

    #[test]
    fn test_chemical_formula_from_str() {
        let input = "H2O";
        let expected = "H₂O";
        let formula = ChemicalFormula::<u32, i32>::from_str(input).unwrap();
        assert_eq!(formula.to_string(), expected);
    }

    #[test]
    fn test_mineral_formula_try_from() {
        let input = "SiO2";
        let expected = "SiO₂"; // Mineral formula uses subscripts
        let formula = MineralFormula::<u32, i32>::try_from(input).unwrap();
        assert_eq!(formula.to_string(), expected);
    }

    #[test]
    fn test_mineral_formula_from_str() {
        let input = "SiO2";
        let expected = "SiO₂";
        let formula = MineralFormula::<u32, i32>::from_str(input).unwrap();
        assert_eq!(formula.to_string(), expected);
    }

    #[test]
    fn test_residual_formula_try_from() {
        let input = "C6H12O6";
        let expected = "C₆H₁₂O₆"; // Residual formula uses subscripts
        let formula = ResidualFormula::<u32, i32>::try_from(input).unwrap();
        assert_eq!(formula.to_string(), expected);
    }

    #[test]
    fn test_residual_formula_from_str() {
        let input = "C6H12O6";
        let expected = "C₆H₁₂O₆";
        let formula = ResidualFormula::<u32, i32>::from_str(input).unwrap();
        assert_eq!(formula.to_string(), expected);
    }
}
