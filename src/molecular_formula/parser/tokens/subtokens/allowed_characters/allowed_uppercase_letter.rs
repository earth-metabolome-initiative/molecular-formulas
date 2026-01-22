//! Module defining allowed uppercase letters in a molecular formula.

use std::fmt::Display;

use elements_rs::{Element, Isotope};

/// Enumeration of allowed uppercase letters in a molecular formula.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AllowedUppercaseLetter {
    /// The letter 'A'
    A,
    /// The letter 'B'
    B,
    /// The letter 'C'
    C,
    /// The letter 'D'
    D,
    /// The letter 'E'
    E,
    /// The letter 'F'
    F,
    /// The letter 'H'
    H,
    /// The letter 'G'
    G,
    /// The letter 'I'
    I,
    /// The letter 'K'
    K,
    /// The letter 'L'
    L,
    /// The letter 'M'
    M,
    /// The letter 'N'
    N,
    /// The letter 'O'
    O,
    /// The letter 'P'
    P,
    /// The letter 'R'
    R,
    /// The letter 'S'
    S,
    /// The letter 'T'
    T,
    /// The letter 'U'
    U,
    /// The letter 'V'
    V,
    /// The letter 'W'
    W,
    /// The letter 'X'
    X,
    /// The letter 'Y'
    Y,
    /// The letter 'Z'
    Z,
}

impl From<AllowedUppercaseLetter> for char {
    fn from(letter: AllowedUppercaseLetter) -> Self {
        match letter {
            AllowedUppercaseLetter::A => 'A',
            AllowedUppercaseLetter::B => 'B',
            AllowedUppercaseLetter::C => 'C',
            AllowedUppercaseLetter::D => 'D',
            AllowedUppercaseLetter::E => 'E',
            AllowedUppercaseLetter::F => 'F',
            AllowedUppercaseLetter::H => 'H',
            AllowedUppercaseLetter::G => 'G',
            AllowedUppercaseLetter::I => 'I',
            AllowedUppercaseLetter::K => 'K',
            AllowedUppercaseLetter::L => 'L',
            AllowedUppercaseLetter::M => 'M',
            AllowedUppercaseLetter::N => 'N',
            AllowedUppercaseLetter::O => 'O',
            AllowedUppercaseLetter::P => 'P',
            AllowedUppercaseLetter::R => 'R',
            AllowedUppercaseLetter::S => 'S',
            AllowedUppercaseLetter::T => 'T',
            AllowedUppercaseLetter::U => 'U',
            AllowedUppercaseLetter::V => 'V',
            AllowedUppercaseLetter::W => 'W',
            AllowedUppercaseLetter::X => 'X',
            AllowedUppercaseLetter::Y => 'Y',
            AllowedUppercaseLetter::Z => 'Z',
        }
    }
}

impl Display for AllowedUppercaseLetter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

impl TryFrom<AllowedUppercaseLetter> for Element {
    type Error = elements_rs::errors::Error;

    fn try_from(letter: AllowedUppercaseLetter) -> Result<Self, Self::Error> {
        let character: char = letter.into();
        Element::try_from(character)
    }
}

impl TryFrom<AllowedUppercaseLetter> for Isotope {
    type Error = elements_rs::errors::Error;

    fn try_from(letter: AllowedUppercaseLetter) -> Result<Self, Self::Error> {
        let character: char = letter.into();
        Isotope::try_from(character)
    }
}

/// Error enumeration when converting a character to an
/// `AllowedUppercaseLetter`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AllowedUppercaseLetterError {
    /// The character is not an allowed uppercase letter.
    #[error("The character '{0}' is not an allowed uppercase letter.")]
    NotAllowedUppercaseLetter(char),
}

impl TryFrom<char> for AllowedUppercaseLetter {
    type Error = AllowedUppercaseLetterError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Self::A),
            'B' => Ok(Self::B),
            'C' => Ok(Self::C),
            'D' => Ok(Self::D),
            'E' => Ok(Self::E),
            'F' => Ok(Self::F),
            'H' => Ok(Self::H),
            'G' => Ok(Self::G),
            'I' => Ok(Self::I),
            'K' => Ok(Self::K),
            'L' => Ok(Self::L),
            'M' => Ok(Self::M),
            'N' => Ok(Self::N),
            'O' => Ok(Self::O),
            'P' => Ok(Self::P),
            'R' => Ok(Self::R),
            'S' => Ok(Self::S),
            'T' => Ok(Self::T),
            'U' => Ok(Self::U),
            'V' => Ok(Self::V),
            'W' => Ok(Self::W),
            'X' => Ok(Self::X),
            'Y' => Ok(Self::Y),
            'Z' => Ok(Self::Z),
            _ => Err(AllowedUppercaseLetterError::NotAllowedUppercaseLetter(c)),
        }
    }
}
