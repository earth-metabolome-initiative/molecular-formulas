//! Submodule creating the `TokenIter` struct, which is an iterator over
//! the `Token`s found in a provided string.

use std::fmt::Display;

use elements_rs::{Element, Isotope};
use num_traits::{CheckedNeg, Signed};

mod allowed_characters;
mod complex;
pub use complex::Complex;
mod typesetting;
pub use allowed_characters::{
    AllowedCharacter, AllowedCharacterError, AllowedLowercaseLetter, AllowedUppercaseLetter,
    CountLike, Digit, GreekLetter, NumberLike, Radical, Terminator, subscript_digits_ltr,
    superscript_digits_ltr,
};
pub use typesetting::{Baseline, Subscript, Superscript, TypeSetting};
mod markers;
pub use allowed_characters::Bracket;
use allowed_characters::{AllowedCharacters, Digits};
mod residual;
pub use markers::{
    BaselineMinus, BaselinePlus, CharacterMarker, SignMarker, SuperscriptMinus, SuperscriptPlus,
};
pub use residual::Residual;

/// Marker trait for typesettings that support charge notation.
pub trait ChargeLike: NumberLike + Signed + CheckedNeg {}
impl<T> ChargeLike for T where T: NumberLike + Signed + CheckedNeg {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Tokens aggregating allowed characters.
pub enum SubToken<Signed, Unsigned> {
    /// An element token.
    Element(Element),
    /// A single-lettered isotope token.
    Isotope(Isotope),
    /// A complex token.
    Complex(Complex),
    /// A greek letter token.
    GreekLetter(GreekLetter),
    /// A baseline number token.
    BaselineNumber(Unsigned),
    /// A subscript number token.
    SubscriptNumber(Unsigned),
    /// A superscript number token.
    SuperscriptNumber(Unsigned),
    /// A baseline charge token.
    Charge(Signed),
    /// An open bracket '(' or '['.
    OpenBracket(Bracket),
    /// A residual marker.
    Residual,
    /// A radical marker.
    Radical,
    /// A terminator token.
    Terminator(Terminator),
}

impl<Signed: ChargeLike, Unsigned: CountLike> Display for SubToken<Signed, Unsigned> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Element(element) => write!(f, "{element}"),
            Self::Isotope(isotope) => write!(f, "{isotope}"),
            Self::Complex(complex) => write!(f, "{complex}"),
            Self::GreekLetter(gl) => write!(f, "{gl}"),
            Self::BaselineNumber(n) | Self::SubscriptNumber(n) | Self::SuperscriptNumber(n) => {
                write!(f, "{n}")
            }
            Self::Charge(c) => write!(f, "{c}"),
            Self::OpenBracket(bracket) => {
                match bracket {
                    Bracket::Round => write!(f, "("),
                    Bracket::Square => write!(f, "["),
                }
            }
            Self::Residual => write!(f, "{Residual}"),
            Self::Radical => write!(f, "{Radical}"),
            Self::Terminator(terminator) => write!(f, "{terminator}"),
        }
    }
}

impl<S, U> From<Residual> for SubToken<S, U> {
    fn from(_: Residual) -> Self {
        SubToken::Residual
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Error enumeration for `SubTokens`.
pub enum SubTokenError {
    /// Error indicating that an invalid character was encountered.
    #[error("Invalid allowed character: {0}")]
    AllowedCharacter(#[from] AllowedCharacterError),
    /// Unknown combination of uppercase and lowercase letters.
    #[error("Unknown element, isotope or complex symbol: {0}{1}")]
    UnknownLetterCombination(AllowedUppercaseLetter, AllowedLowercaseLetter),
    /// Unknown singleton uppercase letter.
    #[error("Unknown element or isotope: {0}")]
    UnknownUppercaseLetter(AllowedUppercaseLetter),
    /// Unknown singleton lowercase letter.
    #[error("Unknown element or isotope: {0}")]
    UnknownLowercaseLetter(AllowedLowercaseLetter),
    /// Numeric overflow error.
    #[error("Numeric overflow")]
    NumericOverflow,
    /// Unexpected end of input while parsing allowed character.
    #[error("Unexpected end of input while parsing allowed character.")]
    UnexpectedEndOfInputWhileParsingAllowedCharacter,
    /// Invalid repeated token.
    #[error("Invalid repeated character: '{0}'.")]
    InvalidRepeatedCharacter(AllowedCharacter),
    /// Leading zero in number.
    #[error("Leading zero in number is not allowed.")]
    LeadingZeroInNumber,
}

/// Iterator over the `Token`s found in a provided string.
pub struct SubTokens<'a, Signed, Unsigned> {
    /// A peekable iterator over the allowed characters.
    allowed_chars: core::iter::Peekable<AllowedCharacters<'a>>,
    /// Phantom data for signed types.
    _signed: core::marker::PhantomData<Signed>,
    /// Phantom data for unsigned types.
    _unsigned: core::marker::PhantomData<Unsigned>,
}

impl<'a, Signed, Unsigned> From<&'a str> for SubTokens<'a, Signed, Unsigned> {
    fn from(s: &'a str) -> Self {
        SubTokens {
            allowed_chars: AllowedCharacters::from(s).peekable(),
            _signed: core::marker::PhantomData,
            _unsigned: core::marker::PhantomData,
        }
    }
}

impl<'token, Signed: ChargeLike + TryFrom<Unsigned>, Unsigned: NumberLike>
    SubTokens<'token, Signed, Unsigned>
{
    /// Iterates over the subscript digits it enconters.
    fn iter_subscript_digits(&mut self) -> Digits<'_, 'token, Subscript> {
        (&mut self.allowed_chars).into()
    }

    /// Iterates over the superscript digits it enconters.
    fn iter_superscript_digits(&mut self) -> Digits<'_, 'token, Superscript> {
        (&mut self.allowed_chars).into()
    }

    /// Iterates over the baseline digits it enconters.
    fn iter_baseline_digits(&mut self) -> Digits<'_, 'token, Baseline> {
        (&mut self.allowed_chars).into()
    }

    /// Parses a charge.
    fn parse_charge<'a, CS: SignMarker>(&'a mut self) -> Result<Signed, SubTokenError>
    where
        Digits<'a, 'token, CS::TS>: Iterator<Item = Result<Digit, AllowedCharacterError>>,
    {
        // There might be one of more signs in some notations.
        let mut sign_count: Signed = Signed::zero();
        while matches!(self.allowed_chars.peek().copied(), Some(Ok(c)) if CS::matches(c.into())) {
            sign_count =
                sign_count.checked_add(&Signed::ONE).ok_or(SubTokenError::NumericOverflow)?;
            self.allowed_chars.next();
        }

        assert_ne!(sign_count, Signed::ZERO);

        // If the sign count is, in absolute value, equal to one, it may be followed
        // by an optional number.
        if sign_count.abs() == Signed::ONE {
            let mut digits: Digits<'a, 'token, CS::TS> = (&mut self.allowed_chars).into();
            match digits.collect_number::<Signed>() {
                Ok(n) => {
                    sign_count = n;
                }
                Err(SubTokenError::UnexpectedEndOfInputWhileParsingAllowedCharacter) => {
                    // No digits found, we just return the sign count of +/- 1.
                }
                Err(e) => return Err(e),
            }
        }

        // We adjust the sign of the charge according to the sign marker.
        if !CS::POSITIVE {
            sign_count = sign_count.checked_neg().ok_or(SubTokenError::NumericOverflow)?;
        }

        Ok(sign_count)
    }

    /// Parses a number which might be followed by a charge.
    fn parse_superscript_number_or_charge(
        &mut self,
    ) -> Result<SubToken<Signed, Unsigned>, SubTokenError> {
        let number = self.iter_superscript_digits().collect_number()?;
        if matches!(self.allowed_chars.peek().copied(), Some(Ok(AllowedCharacter::SuperscriptPlus)))
        {
            // Consume the plus sign.
            self.allowed_chars.next();
            // We convert the number to a charge.
            let charge = Signed::try_from(number).map_err(|_| SubTokenError::NumericOverflow)?;
            Ok(SubToken::Charge(charge))
        } else if matches!(
            self.allowed_chars.peek().copied(),
            Some(Ok(AllowedCharacter::SuperscriptMinus))
        ) {
            // Consume the minus sign.
            self.allowed_chars.next();
            // We convert the number to a charge and negate it.
            let mut charge =
                Signed::try_from(number).map_err(|_| SubTokenError::NumericOverflow)?;
            charge = charge.checked_neg().ok_or(SubTokenError::NumericOverflow)?;
            Ok(SubToken::Charge(charge))
        } else {
            // No charge following the number.
            Ok(SubToken::SuperscriptNumber(number))
        }
    }
}

impl<Signed: ChargeLike + TryFrom<Unsigned>, Unsigned: NumberLike> Iterator
    for SubTokens<'_, Signed, Unsigned>
{
    type Item = Result<SubToken<Signed, Unsigned>, SubTokenError>;

    #[allow(clippy::too_many_lines)]
    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.allowed_chars.peek().copied() {
            Some(Ok(c)) => c,
            Some(Err(e)) => return Some(Err(e.into())),
            None => return None,
        };

        Some(Ok(match next {
            AllowedCharacter::Radical => {
                self.allowed_chars.next();

                if self.allowed_chars.peek().copied() == Some(Ok(AllowedCharacter::Radical)) {
                    // Two radicals in a row is not allowed.
                    return Some(Err(SubTokenError::InvalidRepeatedCharacter(
                        AllowedCharacter::Radical,
                    )));
                }

                SubToken::Radical
            }
            AllowedCharacter::Terminator(terminator) => {
                self.allowed_chars.next();
                SubToken::Terminator(terminator)
            }
            AllowedCharacter::OpenBracket(bracket) => {
                self.allowed_chars.next();
                SubToken::OpenBracket(bracket)
            }
            AllowedCharacter::GreekLetter(gl) => {
                self.allowed_chars.next();
                SubToken::GreekLetter(gl)
            }
            AllowedCharacter::SubscriptDigit(_) => {
                // When we find a subscript digit, we can only form a number
                // composited of several subscript digits.
                match self.iter_subscript_digits().collect_number() {
                    Ok(n) => SubToken::SubscriptNumber(n),
                    Err(e) => return Some(Err(e)),
                }
            }
            AllowedCharacter::SuperscriptMinus => {
                // When we find a superscript minus, we can only form a charge.
                match self.parse_charge::<SuperscriptMinus>() {
                    Ok(c) => SubToken::Charge(c),
                    Err(e) => return Some(Err(e)),
                }
            }
            AllowedCharacter::SuperscriptPlus => {
                // When we find a superscript plus, we can only form a charge.
                match self.parse_charge::<SuperscriptPlus>() {
                    Ok(c) => SubToken::Charge(c),
                    Err(e) => return Some(Err(e)),
                }
            }
            AllowedCharacter::BaselinePlus => {
                // When we find a baseline plus, we can only form a charge.
                match self.parse_charge::<BaselinePlus>() {
                    Ok(c) => SubToken::Charge(c),
                    Err(e) => return Some(Err(e)),
                }
            }
            AllowedCharacter::BaselineMinus => {
                // When we find a baseline minus, we can only form a charge.
                match self.parse_charge::<BaselineMinus>() {
                    Ok(c) => SubToken::Charge(c),
                    Err(e) => return Some(Err(e)),
                }
            }
            AllowedCharacter::BaselineDigit(_) => {
                // When we find a baseline digit, we can only form a number
                // composited of several baseline digits.
                match self.iter_baseline_digits().collect_number() {
                    Ok(n) => SubToken::BaselineNumber(n),
                    Err(e) => return Some(Err(e)),
                }
            }
            AllowedCharacter::SuperscriptDigit(_) => {
                // When we find a baseline digit, it could lead to either a number
                // or a number followed by a charge.
                match self.parse_superscript_number_or_charge() {
                    Ok(token) => token,
                    Err(e) => return Some(Err(e)),
                }
            }
            AllowedCharacter::LowercaseLetter(letter) => {
                return Some(Err(SubTokenError::UnknownLowercaseLetter(letter)));
            }
            AllowedCharacter::UppercaseLetter(letter) => {
                self.allowed_chars.next();
                // If the next character is a lowercase letter, it could be composing
                // an element symbol or a complex.
                if let Some(Ok(AllowedCharacter::LowercaseLetter(next_letter))) =
                    self.allowed_chars.peek().copied()
                {
                    if let Ok(element) = Element::try_from([letter.into(), next_letter.into()]) {
                        // It's an element.
                        self.allowed_chars.next(); // Consume the lowercase letter.
                        SubToken::Element(element)
                    } else if let Ok(complex) =
                        Complex::try_from([letter.into(), next_letter.into()])
                    {
                        // It's a complex.
                        self.allowed_chars.next(); // Consume the lowercase letter.
                        SubToken::Complex(complex)
                    } else {
                        return Some(Err(SubTokenError::UnknownLetterCombination(
                            letter,
                            next_letter,
                        )));
                    }
                } else if let Ok(element) = Element::try_from(letter) {
                    // It's a single-letter element.
                    SubToken::Element(element)
                } else if let Ok(isotope) = Isotope::try_from(letter) {
                    // It's a single-letter isotope.
                    SubToken::Isotope(isotope)
                } else if letter == AllowedUppercaseLetter::R {
                    // It's a residual marker.
                    Residual.into()
                } else {
                    return Some(Err(SubTokenError::UnknownUppercaseLetter(letter)));
                }
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use elements_rs::{Element, MassNumber};

    use super::*;

    #[test]
    fn test_subtoken_display() {
        // We use i32 and u32 as our Signed and Unsigned types.
        type Token = SubToken<i32, u32>;

        // Element
        let token: Token = SubToken::Element(Element::C);
        assert_eq!(token.to_string(), "C");

        // Isotope
        // Use into_iter to get &Isotope in find
        if let Some(iso) = Element::C.isotopes().into_iter().find(|i| i.mass_number() == 13) {
            let token: Token = SubToken::Isotope(iso);
            assert!(!token.to_string().is_empty());
        }

        // Complex
        let token: Token = SubToken::Complex(Complex::Methyl);
        assert_eq!(token.to_string(), "Me");

        // GreekLetter
        let token: Token = SubToken::GreekLetter(GreekLetter::Alpha);
        assert_eq!(token.to_string(), "α");

        // BaselineNumber
        let token: Token = SubToken::BaselineNumber(42);
        assert_eq!(token.to_string(), "42");

        // SubscriptNumber
        let token: Token = SubToken::SubscriptNumber(5);
        assert_eq!(token.to_string(), "5");

        // SuperscriptNumber
        let token: Token = SubToken::SuperscriptNumber(9);
        assert_eq!(token.to_string(), "9");

        // Charge
        let token: Token = SubToken::Charge(-1);
        assert_eq!(token.to_string(), "-1");
        let token: Token = SubToken::Charge(3);
        assert_eq!(token.to_string(), "3");

        // OpenBracket
        let token: Token = SubToken::OpenBracket(Bracket::Round);
        assert_eq!(token.to_string(), "(");
        let token: Token = SubToken::OpenBracket(Bracket::Square);
        assert_eq!(token.to_string(), "[");

        // Residual
        let token: Token = SubToken::Residual;
        assert_eq!(token.to_string(), "R");

        // Radical
        let token: Token = SubToken::Radical;
        assert_eq!(token.to_string(), "•");

        // Terminator
        let token: Token = SubToken::Terminator(Terminator::Dot);
        assert_eq!(token.to_string(), ".");

        let token: Token = SubToken::Terminator(Terminator::CloseBracket(Bracket::Round));
        assert_eq!(token.to_string(), ")");
    }
}
