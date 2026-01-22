//! Submodule defining an enumeration of the characters which are allowed
//! in a molecular formula.

use core::str::Chars;
use std::fmt::Display;

mod greek_letters;
pub use greek_letters::GreekLetter;
mod allowed_lowercase_letter;
pub use allowed_lowercase_letter::{AllowedLowercaseLetter, AllowedLowercaseLetterError};
mod allowed_uppercase_letter;
pub use allowed_uppercase_letter::{AllowedUppercaseLetter, AllowedUppercaseLetterError};
mod brackets;
mod digits;
pub use brackets::Bracket;
pub(super) use digits::Digits;
pub use digits::{CountLike, Digit, NumberLike, subscript_digits_ltr, superscript_digits_ltr};

mod radicals;
pub use radicals::Radical;

use super::{BaselineMinus, BaselinePlus, CharacterMarker};

/// Enumeration of allowed characters in a molecular formula.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AllowedCharacter {
    /// An uppercase letter.
    UppercaseLetter(AllowedUppercaseLetter),
    /// A lowercase letter.
    LowercaseLetter(AllowedLowercaseLetter),
    /// A greek letter.
    GreekLetter(GreekLetter),
    /// A baseline plus sign.
    BaselinePlus,
    /// A baseline minus sign.
    BaselineMinus,
    /// A superscript plus sign.
    SuperscriptPlus,
    /// A superscript minus sign.
    SuperscriptMinus,
    /// A radical marker.
    Radical,
    /// A Superscript digit.
    SuperscriptDigit(Digit),
    /// A Subscript digit.
    SubscriptDigit(Digit),
    /// A baseline digit.
    BaselineDigit(Digit),
    /// An open bracket.
    OpenBracket(Bracket),
    /// A group terminator.
    Terminator(Terminator),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a terminator in a molecular formula.
pub enum Terminator {
    /// A close bracket.
    CloseBracket(Bracket),
    /// A dot character.
    Dot,
}

impl From<Terminator> for AllowedCharacter {
    fn from(terminator: Terminator) -> Self {
        match terminator {
            Terminator::CloseBracket(bracket) => {
                match bracket {
                    Bracket::Round => {
                        AllowedCharacter::Terminator(Terminator::CloseBracket(Bracket::Round))
                    }
                    Bracket::Square => {
                        AllowedCharacter::Terminator(Terminator::CloseBracket(Bracket::Square))
                    }
                }
            }
            Terminator::Dot => AllowedCharacter::Terminator(Terminator::Dot),
        }
    }
}

impl From<Terminator> for char {
    fn from(terminator: Terminator) -> Self {
        match terminator {
            Terminator::CloseBracket(bracket) => {
                match bracket {
                    Bracket::Round => ')',
                    Bracket::Square => ']',
                }
            }
            Terminator::Dot => '.',
        }
    }
}

impl Display for Terminator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Terminator::CloseBracket(bracket) => {
                write!(
                    f,
                    "{}",
                    match bracket {
                        Bracket::Round => ')',
                        Bracket::Square => ']',
                    }
                )
            }
            Terminator::Dot => write!(f, "."),
        }
    }
}

impl Display for AllowedCharacter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

/// Error enumeration when converting a character to an `AllowedCharacter`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AllowedCharacterError {
    /// The character is not an allowed character.
    #[error("The character '{0}' is not an allowed character.")]
    NotAllowedCharacter(char),
    /// The character is not an allowed Uppercase letter.
    #[error("The character '{0}' is not an allowed uppercase letter.")]
    UppercaseLetter(#[from] AllowedUppercaseLetterError),
    /// The character is not an allowed lowercase letter.
    #[error("The character '{0}' is not an allowed lowercase letter.")]
    LowercaseLetter(#[from] AllowedLowercaseLetterError),
    /// Unexpected end of input.
    #[error("Unexpected end of input while parsing allowed character.")]
    UnexpectedEndOfInputWhileParsingAllowedCharacter,
    /// Greek letters must be followed by a hyphen.
    #[error("Greek letter `{0}` must be followed by a hyphen.")]
    GreekLetterMustBeFollowedByHyphen(GreekLetter),
}

impl From<AllowedUppercaseLetter> for AllowedCharacter {
    fn from(letter: AllowedUppercaseLetter) -> Self {
        Self::UppercaseLetter(letter)
    }
}

impl From<AllowedLowercaseLetter> for AllowedCharacter {
    fn from(letter: AllowedLowercaseLetter) -> Self {
        Self::LowercaseLetter(letter)
    }
}

impl TryFrom<char> for AllowedCharacter {
    type Error = AllowedCharacterError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        if let Some(digit) = Digit::from_superscript_char(c) {
            return Ok(AllowedCharacter::SuperscriptDigit(digit));
        }
        if let Some(digit) = Digit::from_subscript_char(c) {
            return Ok(AllowedCharacter::SubscriptDigit(digit));
        }
        if let Ok(greek_letter) = GreekLetter::try_from(c) {
            return Ok(AllowedCharacter::GreekLetter(greek_letter));
        }

        match c {
            _ if BaselinePlus::matches(c) => Ok(AllowedCharacter::BaselinePlus),
            _ if BaselineMinus::matches(c) => Ok(AllowedCharacter::BaselineMinus),
            _ if Radical::matches(c) => Ok(AllowedCharacter::Radical),
            '⁺' => Ok(AllowedCharacter::SuperscriptPlus),
            '⁻' => Ok(AllowedCharacter::SuperscriptMinus),
            '0'..='9' => {
                Ok(AllowedCharacter::BaselineDigit(((c as u8) - b'0').try_into().unwrap()))
            }
            '(' => Ok(AllowedCharacter::OpenBracket(Bracket::Round)),
            ')' => Ok(Terminator::CloseBracket(Bracket::Round).into()),
            '[' => Ok(AllowedCharacter::OpenBracket(Bracket::Square)),
            ']' => Ok(Terminator::CloseBracket(Bracket::Square).into()),
            '.' | '•' | '⋅' | '·' => Ok(Terminator::Dot.into()),
            _ if c.is_ascii_uppercase() => Ok(AllowedUppercaseLetter::try_from(c)?.into()),
            _ if c.is_ascii_lowercase() => Ok(AllowedLowercaseLetter::try_from(c)?.into()),
            _ => Err(AllowedCharacterError::NotAllowedCharacter(c)),
        }
    }
}

/// Iterator over allowed characters in a molecular formula.
pub(super) struct AllowedCharacters<'a> {
    /// The underlying character iterator.
    chars: Chars<'a>,
}

impl<'a> From<&'a str> for AllowedCharacters<'a> {
    fn from(s: &'a str) -> Self {
        Self { chars: s.chars() }
    }
}

/// Returns true if the character is a hyphen or similar.
fn is_hyphen(c: char) -> bool {
    c == '-'
        || c == '\u{2010}'
        || c == '\u{2011}'
        || c == '\u{2012}'
        || c == '\u{2013}'
        || c == '—'
        || c == '―'
}

impl Iterator for AllowedCharacters<'_> {
    type Item = Result<AllowedCharacter, AllowedCharacterError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next().map(|c| {
            let allowed_char = AllowedCharacter::try_from(c)?;

            if let AllowedCharacter::GreekLetter(letter) = allowed_char {
                // Greek letters must be followed by a hyphen
                let Some(next_char) = self.chars.next() else {
                    return Err(AllowedCharacterError::GreekLetterMustBeFollowedByHyphen(letter));
                };

                if !is_hyphen(next_char) {
                    return Err(AllowedCharacterError::GreekLetterMustBeFollowedByHyphen(letter));
                }
            }

            Ok(allowed_char)
        })
    }
}

impl From<AllowedCharacter> for char {
    fn from(letter: AllowedCharacter) -> Self {
        match letter {
            AllowedCharacter::UppercaseLetter(letter) => char::from(letter),
            AllowedCharacter::LowercaseLetter(letter) => char::from(letter),
            AllowedCharacter::GreekLetter(letter) => char::from(letter),
            AllowedCharacter::Radical => char::from(Radical),
            AllowedCharacter::BaselinePlus => '+',
            AllowedCharacter::BaselineMinus => '-',
            AllowedCharacter::SuperscriptPlus => '⁺',
            AllowedCharacter::SuperscriptMinus => '⁻',
            AllowedCharacter::SuperscriptDigit(digit) => digit.to_superscript_char(),
            AllowedCharacter::SubscriptDigit(digit) => digit.to_subscript_char(),
            AllowedCharacter::BaselineDigit(digit) => char::from(digit),
            AllowedCharacter::OpenBracket(Bracket::Round) => '(',
            AllowedCharacter::OpenBracket(Bracket::Square) => '[',
            AllowedCharacter::Terminator(terminator) => terminator.into(),
        }
    }
}
