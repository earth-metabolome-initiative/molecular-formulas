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
