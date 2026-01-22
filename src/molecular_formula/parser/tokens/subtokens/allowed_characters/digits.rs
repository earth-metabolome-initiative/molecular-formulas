//! Module defining digits as allowed characters in a molecular formula.

use core::{
    fmt::{Debug, Display},
    hash::Hash,
};
use std::ops::{DivAssign, MulAssign, RemAssign};

use num_traits::{CheckedAdd, CheckedMul, CheckedSub, ConstOne, ConstZero, Num, Unsigned};

use super::super::{
    AllowedCharacter, AllowedCharacters, Baseline, SubTokenError, Subscript, Superscript,
    TypeSetting, allowed_characters::AllowedCharacterError,
};

/// Trait for number-like types that can represent 0-10.
pub trait NumberLike:
    Num
    + Copy
    + Debug
    + CheckedMul
    + CheckedAdd
    + RemAssign
    + CheckedSub
    + From<Digit>
    + ConstZero
    + DivAssign
    + PartialOrd
    + Ord
    + MulAssign
    + ConstOne
    + PartialEq
    + Eq
    + Display
    + Into<f64>
    + Hash
{
    /// Constant for the `Two` value of the type.
    const TWO: Self;
    /// Constant for the `Three` value of the type.
    const THREE: Self;
    /// Constant for the `Four` value of the type.
    const FOUR: Self;
    /// Constant for the `Five` value of the type.
    const FIVE: Self;
    /// Constant for the `Six` value of the type.
    const SIX: Self;
    /// Constant for the `Seven` value of the type.
    const SEVEN: Self;
    /// Constant for the `Eight` value of the type.
    const EIGHT: Self;
    /// Constant for the `Nine` value of the type.
    const NINE: Self;
    /// Constant for the `Ten` value of the type.
    const TEN: Self;
    /// Constant for the `Eleven` value of the type.
    const ELEVEN: Self;
}

/// Trait for count-like types.
pub trait CountLike: NumberLike + Unsigned + Into<u64> {}

impl<T> CountLike for T where T: NumberLike + Unsigned + Into<u64> {}

/// Macro to implement the `NumberLike` trait for a given numeric type.
macro_rules! impl_number_like {
    ($t:ty) => {
        impl NumberLike for $t {
            const TWO: Self = 2;
            const THREE: Self = 3;
            const FOUR: Self = 4;
            const FIVE: Self = 5;
            const SIX: Self = 6;
            const SEVEN: Self = 7;
            const EIGHT: Self = 8;
            const NINE: Self = 9;
            const TEN: Self = 10;
            const ELEVEN: Self = 11;
        }
    };
}

impl_number_like!(u8);
impl_number_like!(u16);
impl_number_like!(u32);
impl_number_like!(i8);
impl_number_like!(i16);
impl_number_like!(i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enumeration of digits.
pub enum Digit {
    /// Zero
    Zero,
    /// One
    One,
    /// Two
    Two,
    /// Three
    Three,
    /// Four
    Four,
    /// Five
    Five,
    /// Six
    Six,
    /// Seven
    Seven,
    /// Eight
    Eight,
    /// Nine
    Nine,
}

impl From<Digit> for char {
    fn from(digit: Digit) -> Self {
        match digit {
            Digit::Zero => '0',
            Digit::One => '1',
            Digit::Two => '2',
            Digit::Three => '3',
            Digit::Four => '4',
            Digit::Five => '5',
            Digit::Six => '6',
            Digit::Seven => '7',
            Digit::Eight => '8',
            Digit::Nine => '9',
        }
    }
}

impl Digit {
    /// Attempts to create a digit from the provided subscript character.
    #[must_use]
    pub fn from_subscript_char(c: char) -> Option<Self> {
        match c {
            '₀' => Some(Digit::Zero),
            '₁' => Some(Digit::One),
            '₂' => Some(Digit::Two),
            '₃' => Some(Digit::Three),
            '₄' => Some(Digit::Four),
            '₅' => Some(Digit::Five),
            '₆' => Some(Digit::Six),
            '₇' => Some(Digit::Seven),
            '₈' => Some(Digit::Eight),
            '₉' => Some(Digit::Nine),
            _ => None,
        }
    }

    /// Attempts to create a digit from the provided superscript character.
    #[must_use]
    pub fn from_superscript_char(c: char) -> Option<Self> {
        match c {
            '⁰' => Some(Digit::Zero),
            '¹' => Some(Digit::One),
            '²' => Some(Digit::Two),
            '³' => Some(Digit::Three),
            '⁴' => Some(Digit::Four),
            '⁵' => Some(Digit::Five),
            '⁶' => Some(Digit::Six),
            '⁷' => Some(Digit::Seven),
            '⁸' => Some(Digit::Eight),
            '⁹' => Some(Digit::Nine),
            _ => None,
        }
    }

    /// Converts the digit to its corresponding subscript character.
    #[must_use]
    pub fn to_subscript_char(&self) -> char {
        match self {
            Digit::Zero => '₀',
            Digit::One => '₁',
            Digit::Two => '₂',
            Digit::Three => '₃',
            Digit::Four => '₄',
            Digit::Five => '₅',
            Digit::Six => '₆',
            Digit::Seven => '₇',
            Digit::Eight => '₈',
            Digit::Nine => '₉',
        }
    }

    /// Converts the digit to its corresponding superscript character.
    #[must_use]
    pub fn to_superscript_char(&self) -> char {
        match self {
            Digit::Zero => '⁰',
            Digit::One => '¹',
            Digit::Two => '²',
            Digit::Three => '³',
            Digit::Four => '⁴',
            Digit::Five => '⁵',
            Digit::Six => '⁶',
            Digit::Seven => '⁷',
            Digit::Eight => '⁸',
            Digit::Nine => '⁹',
        }
    }
}

/// Macro implementing the conversion from Digit to several numeric types.
macro_rules! impl_digit_to_numeric {
    ($($t:ty),*) => {
        $(
            impl From<Digit> for $t {
                fn from(digit: Digit) -> Self {
                    match digit {
                        Digit::Zero => 0,
                        Digit::One => 1,
                        Digit::Two => 2,
                        Digit::Three => 3,
                        Digit::Four => 4,
                        Digit::Five => 5,
                        Digit::Six => 6,
                        Digit::Seven => 7,
                        Digit::Eight => 8,
                        Digit::Nine => 9,
                    }
                }
            }
        )*
    };
}

impl_digit_to_numeric!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

impl TryFrom<u8> for Digit {
    type Error = ();

    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            0 => Ok(Digit::Zero),
            1 => Ok(Digit::One),
            2 => Ok(Digit::Two),
            3 => Ok(Digit::Three),
            4 => Ok(Digit::Four),
            5 => Ok(Digit::Five),
            6 => Ok(Digit::Six),
            7 => Ok(Digit::Seven),
            8 => Ok(Digit::Eight),
            9 => Ok(Digit::Nine),
            _ => Err(()),
        }
    }
}

pub(crate) struct Digits<'iter, 'token, TS: TypeSetting> {
    allowed_chars: &'iter mut core::iter::Peekable<AllowedCharacters<'token>>,
    _typesetting: core::marker::PhantomData<TS>,
}

impl<TS: TypeSetting> Digits<'_, '_, TS>
where
    Self: Iterator<Item = Result<Digit, AllowedCharacterError>>,
{
    /// Constructs a new number of the provided type consuming the iterator.
    pub(crate) fn collect_number<D>(&mut self) -> Result<D, SubTokenError>
    where
        D: NumberLike,
    {
        let mut number = match self.next() {
            Some(Err(e)) => return Err(e.into()),
            Some(Ok(digit)) => {
                let digit_value: D = D::from(digit);
                digit_value
            }
            None => return Err(SubTokenError::UnexpectedEndOfInputWhileParsingAllowedCharacter),
        };

        if number == D::ZERO {
            return Err(SubTokenError::LeadingZeroInNumber);
        }

        for digit_result in self.by_ref() {
            let digit = digit_result?;
            let digit_value: D = D::from(digit);
            number = number
                .checked_mul(&D::TEN)
                .and_then(|n| n.checked_add(&digit_value))
                .ok_or(SubTokenError::NumericOverflow)?;
        }
        Ok(number)
    }
}

impl<'iter, 'token, TS: TypeSetting>
    From<&'iter mut core::iter::Peekable<AllowedCharacters<'token>>> for Digits<'iter, 'token, TS>
{
    fn from(allowed_chars: &'iter mut core::iter::Peekable<AllowedCharacters<'token>>) -> Self {
        Self { allowed_chars, _typesetting: core::marker::PhantomData }
    }
}

impl Iterator for Digits<'_, '_, Subscript> {
    type Item = Result<Digit, AllowedCharacterError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.allowed_chars.peek().copied() {
            Some(Ok(AllowedCharacter::SubscriptDigit(d))) => {
                self.allowed_chars.next();
                Some(Ok(d))
            }
            Some(Err(e)) => {
                self.allowed_chars.next();
                Some(Err(e))
            }
            Some(Ok(_)) | None => None,
        }
    }
}

impl Iterator for Digits<'_, '_, Superscript> {
    type Item = Result<Digit, AllowedCharacterError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.allowed_chars.peek().copied() {
            Some(Ok(AllowedCharacter::SuperscriptDigit(d))) => {
                self.allowed_chars.next();
                Some(Ok(d))
            }
            Some(Err(e)) => {
                self.allowed_chars.next();
                Some(Err(e))
            }
            Some(Ok(_)) | None => None,
        }
    }
}

impl Iterator for Digits<'_, '_, Baseline> {
    type Item = Result<Digit, AllowedCharacterError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.allowed_chars.peek().copied() {
            Some(Ok(AllowedCharacter::BaselineDigit(d))) => {
                self.allowed_chars.next();
                Some(Ok(d))
            }
            Some(Err(e)) => {
                self.allowed_chars.next();
                Some(Err(e))
            }
            Some(Ok(_)) | None => None,
        }
    }
}

/// Returns an iterator over the digits of the provided number in
/// left-to-right order.
pub fn digits_ltr<D: NumberLike>(mut number: D) -> impl Iterator<Item = Digit> {
    let mut div = D::ONE;
    while number / D::TEN >= div {
        div *= D::TEN;
    }

    core::iter::from_fn(move || {
        if div == D::ZERO {
            None
        } else {
            let d = number / div;
            number %= div;
            div /= D::TEN;
            Some(if d == D::ZERO {
                Digit::Zero
            } else if d == D::ONE {
                Digit::One
            } else if d == D::TWO {
                Digit::Two
            } else if d == D::THREE {
                Digit::Three
            } else if d == D::FOUR {
                Digit::Four
            } else if d == D::FIVE {
                Digit::Five
            } else if d == D::SIX {
                Digit::Six
            } else if d == D::SEVEN {
                Digit::Seven
            } else if d == D::EIGHT {
                Digit::Eight
            } else {
                Digit::Nine
            })
        }
    })
}

/// Returns an iterator over the characters of the provided number in
/// left-to-right order, in superscript form.
pub fn superscript_digits_ltr<D: NumberLike>(number: D) -> impl Iterator<Item = char> {
    digits_ltr(number).map(|d| d.to_superscript_char())
}

/// Returns an iterator over the characters of the provided number in
/// left-to-right order, in subscript form.
pub fn subscript_digits_ltr<D: NumberLike>(number: D) -> impl Iterator<Item = char> {
    digits_ltr(number).map(|d| d.to_subscript_char())
}
