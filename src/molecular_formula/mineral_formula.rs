//! Submodule providing the `MineralFormula` struct to represent molecular
//! formulas of minerals.

use alloc::vec::Vec;
use core::{fmt::Display, iter::Peekable};

use elements_rs::{Element, Isotope};

use crate::{
    BaselineMinus, ChargeLike, ChargedMolecularFormulaMetadata, ChemicalTree, CountLike, Empty,
    MolecularFormula, MolecularFormulaMetadata, ParsableFormula, errors::ParserError,
    parsable::CharacterMarker, prelude::ChemicalFormula,
};

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a greek letter in a molecular formula.
///
/// These are not ALL of the greek letters, but only those which are used in
/// mineral formulas, i.e. `'α'`, `'β'`, `'γ'`, `'δ'`, `'φ'`, `'ω'`, `'λ'`,
/// `'μ'`, and `'π'`.
pub enum PolymorphPrefix {
    /// Alpha
    Alpha,
    /// Beta
    Beta,
    /// Gamma
    Gamma,
    /// Delta
    Delta,
    /// Phi
    Phi,
    /// Omega
    Omega,
    /// Lambda
    Lambda,
    /// Mu
    Mu,
    /// Pi
    Pi,
}

impl From<PolymorphPrefix> for char {
    fn from(letter: PolymorphPrefix) -> Self {
        match letter {
            PolymorphPrefix::Alpha => '\u{03b1}',
            PolymorphPrefix::Beta => 'β',
            PolymorphPrefix::Gamma => '\u{03b3}',
            PolymorphPrefix::Delta => 'δ',
            PolymorphPrefix::Phi => 'φ',
            PolymorphPrefix::Omega => 'ω',
            PolymorphPrefix::Lambda => 'λ',
            PolymorphPrefix::Mu => 'μ',
            PolymorphPrefix::Pi => 'π',
        }
    }
}

impl Display for PolymorphPrefix {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

impl TryFrom<char> for PolymorphPrefix {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '\u{03b1}' => Ok(PolymorphPrefix::Alpha),
            'β' => Ok(PolymorphPrefix::Beta),
            '\u{03b3}' => Ok(PolymorphPrefix::Gamma),
            'δ' => Ok(PolymorphPrefix::Delta),
            'φ' => Ok(PolymorphPrefix::Phi),
            'ω' => Ok(PolymorphPrefix::Omega),
            'λ' => Ok(PolymorphPrefix::Lambda),
            'μ' => Ok(PolymorphPrefix::Mu),
            'π' => Ok(PolymorphPrefix::Pi),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord, Hash)]
/// Struct representing a mineral formula, potentially with a greek letter
/// prefix.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
///
/// use molecular_formulas::prelude::*;
///
/// let quartz = MineralFormula::<u32, i32>::from_str("α-SiO2").unwrap();
/// assert_eq!(quartz.to_string(), "α-SiO₂");
/// ```
pub struct MineralFormula<Count: CountLike = u16, Charge: ChargeLike = i16> {
    /// Optional greek letter prefix for the mineral formula.
    polymorph_prefix: Option<PolymorphPrefix>,
    /// The rest of the chemical formula.
    formula: ChemicalFormula<Count, Charge>,
}

impl<Count: CountLike, Charge: ChargeLike> MolecularFormulaMetadata
    for MineralFormula<Count, Charge>
{
    type Count = Count;
}

impl<Count: CountLike, Charge: ChargeLike> MolecularFormula for MineralFormula<Count, Charge> {
    type Tree = ChemicalTree<Count, Charge, Empty>;

    fn counted_mixtures(&self) -> impl Iterator<Item = (Self::Count, &Self::Tree)> {
        self.formula.counted_mixtures()
    }
}

impl<Count: CountLike, Charge: ChargeLike> From<Element> for MineralFormula<Count, Charge> {
    fn from(element: Element) -> Self {
        Self { polymorph_prefix: None, formula: ChemicalFormula::from(element) }
    }
}

impl<Count: CountLike, Charge: ChargeLike> From<Isotope> for MineralFormula<Count, Charge> {
    fn from(isotope: Isotope) -> Self {
        Self { polymorph_prefix: None, formula: ChemicalFormula::from(isotope) }
    }
}

impl<Count: CountLike, Charge: ChargeLike> ChargedMolecularFormulaMetadata
    for MineralFormula<Count, Charge>
where
    Charge: TryFrom<Count>,
{
    type Charge = Charge;
}

impl<Count: CountLike, Charge: ChargeLike> ParsableFormula for MineralFormula<Count, Charge>
where
    Isotope: TryFrom<(elements_rs::Element, Count), Error = elements_rs::errors::Error>,
    Charge: TryFrom<Count>,
{
    type StartOutput = Option<PolymorphPrefix>;
    type Tree = ChemicalTree<Count, Charge, Empty>;

    fn on_start<J>(
        chars: &mut Peekable<J>,
    ) -> Result<<MineralFormula<Count, Charge> as crate::ParsableFormula>::StartOutput, ParserError>
    where
        J: Iterator<Item = char>,
    {
        let first_character = chars.peek().ok_or(ParserError::UnexpectedEndOfInput)?;
        if let Ok(polymorph_prefix) = PolymorphPrefix::try_from(*first_character) {
            chars.next();
            // The polymorph prefix was found, but it must be followed by either an
            // hyphen or something which an OCR would mistake for a hyphen. To have
            // OCR-resilient parsing, we accept anything that looks like a "minus" sign.
            let next_character = chars.next().ok_or(ParserError::UnexpectedEndOfInput)?;
            if BaselineMinus::matches(next_character) {
                Ok(Some(polymorph_prefix))
            } else {
                Err(ParserError::UnexpectedCharacter(next_character))
            }
        } else {
            Ok(None)
        }
    }

    fn from_parsed(
        start_output: Self::StartOutput,
        mixtures: Vec<(Count, Self::Tree)>,
    ) -> Result<Self, crate::errors::ParserError> {
        let formula = ChemicalFormula::from_parsed((), mixtures)?;
        Ok(MineralFormula { polymorph_prefix: start_output, formula })
    }
}

impl<Count: CountLike, Charge: ChargeLike> Display for MineralFormula<Count, Charge> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(prefix) = &self.polymorph_prefix {
            write!(f, "{prefix}")?;
            write!(f, "-")?;
        }
        write!(f, "{}", self.formula)
    }
}

#[cfg(test)]
mod tests {
    use alloc::{format, string::ToString};
    use core::str::FromStr;

    use super::*;

    #[test]
    fn test_all_prefixes_roundtrip() {
        let cases = [
            (PolymorphPrefix::Alpha, "α"),
            (PolymorphPrefix::Beta, "β"),
            (PolymorphPrefix::Gamma, "γ"),
            (PolymorphPrefix::Delta, "δ"),
            (PolymorphPrefix::Phi, "φ"),
            (PolymorphPrefix::Omega, "ω"),
            (PolymorphPrefix::Lambda, "λ"),
            (PolymorphPrefix::Mu, "μ"),
            (PolymorphPrefix::Pi, "π"),
        ];

        // Testing with simple "SiO2".
        // Note: The Display implementation for ChemicalFormula uses subscripts (SiO₂).
        // To ensure roundtrip equality (string -> parse -> string == string), we use
        // the subscript version as input.
        let formula_part = "SiO₂";

        for (prefix, char_representation) in cases {
            let input = format!("{char_representation}-{formula_part}");
            let parsed = MineralFormula::<u32, i32>::from_str(&input).expect("Should parse");

            assert_eq!(parsed.polymorph_prefix, Some(prefix), "Prefix mismatch for {input}");
            assert_eq!(parsed.to_string(), input, "Roundtrip mismatch for {input}");
        }
    }

    #[test]
    fn test_illegal_cases() {
        // defined prefix character but missing hyphen
        assert_eq!(
            MineralFormula::<u32, i32>::from_str("αSiO2"),
            Err(ParserError::UnexpectedCharacter('S'))
        );

        // defined prefix character but wrong separator (space is not a BaselineMinus)
        assert_eq!(
            MineralFormula::<u32, i32>::from_str("α SiO2"),
            Err(ParserError::UnexpectedCharacter(' '))
        );

        // Just the prefix (UnexpectedEndOfInput looking for hyphen)
        assert_eq!(
            MineralFormula::<u32, i32>::from_str("α"),
            Err(ParserError::UnexpectedEndOfInput)
        );

        // Prefix with an hyphen but no formula
        assert_eq!(
            MineralFormula::<u32, i32>::from_str("α-"),
            Err(ParserError::EmptyMolecularTree)
        );
    }
}
