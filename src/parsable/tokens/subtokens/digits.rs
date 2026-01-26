//! Module defining digits as allowed characters in a molecular formula.

use core::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{DivAssign, MulAssign, RemAssign},
};

use num_traits::{CheckedAdd, CheckedMul, CheckedSub, ConstOne, ConstZero, Num, Unsigned};

use crate::errors::NumericError;

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
    + From<BaselineDigit>
    + From<SubscriptDigit>
    + From<SuperscriptDigit>
    + ConstZero
    + DivAssign
    + PartialOrd
    + MulAssign
    + ConstOne
    + Eq
    + Hash
    + PartialEq
    + Display
    + Into<f64>
    + Into<i64>
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
pub trait CountLike: NumberLike + Unsigned + Into<usize> {}

impl<T> CountLike for T where T: NumberLike + Unsigned + Into<usize> {}

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

impl Digit {
    /// Returns true if the digit is zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        matches!(self, Digit::Zero)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a baseline digit in a molecular formula.
pub struct BaselineDigit(Digit);

impl BaselineDigit {
    /// Returns true if the digit is zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a subscript digit in a molecular formula.
pub struct SubscriptDigit(Digit);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a superscript digit in a molecular formula.
pub struct SuperscriptDigit(Digit);

impl From<BaselineDigit> for char {
    fn from(digit: BaselineDigit) -> Self {
        match digit.0 {
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

impl TryFrom<char> for BaselineDigit {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(BaselineDigit(Digit::Zero)),
            '1' => Ok(BaselineDigit(Digit::One)),
            '2' => Ok(BaselineDigit(Digit::Two)),
            '3' => Ok(BaselineDigit(Digit::Three)),
            '4' => Ok(BaselineDigit(Digit::Four)),
            '5' => Ok(BaselineDigit(Digit::Five)),
            '6' => Ok(BaselineDigit(Digit::Six)),
            '7' => Ok(BaselineDigit(Digit::Seven)),
            '8' => Ok(BaselineDigit(Digit::Eight)),
            '9' => Ok(BaselineDigit(Digit::Nine)),
            _ => Err(()),
        }
    }
}

impl From<SubscriptDigit> for char {
    fn from(digit: SubscriptDigit) -> Self {
        match digit.0 {
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
}

impl TryFrom<char> for SubscriptDigit {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '₀' => Ok(SubscriptDigit(Digit::Zero)),
            '₁' => Ok(SubscriptDigit(Digit::One)),
            '₂' => Ok(SubscriptDigit(Digit::Two)),
            '₃' => Ok(SubscriptDigit(Digit::Three)),
            '₄' => Ok(SubscriptDigit(Digit::Four)),
            '₅' => Ok(SubscriptDigit(Digit::Five)),
            '₆' => Ok(SubscriptDigit(Digit::Six)),
            '₇' => Ok(SubscriptDigit(Digit::Seven)),
            '₈' => Ok(SubscriptDigit(Digit::Eight)),
            '₉' => Ok(SubscriptDigit(Digit::Nine)),
            _ => Err(()),
        }
    }
}

impl From<SuperscriptDigit> for char {
    fn from(digit: SuperscriptDigit) -> Self {
        match digit.0 {
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

impl TryFrom<char> for SuperscriptDigit {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '⁰' => Ok(SuperscriptDigit(Digit::Zero)),
            '¹' => Ok(SuperscriptDigit(Digit::One)),
            '²' => Ok(SuperscriptDigit(Digit::Two)),
            '³' => Ok(SuperscriptDigit(Digit::Three)),
            '⁴' => Ok(SuperscriptDigit(Digit::Four)),
            '⁵' => Ok(SuperscriptDigit(Digit::Five)),
            '⁶' => Ok(SuperscriptDigit(Digit::Six)),
            '⁷' => Ok(SuperscriptDigit(Digit::Seven)),
            '⁸' => Ok(SuperscriptDigit(Digit::Eight)),
            '⁹' => Ok(SuperscriptDigit(Digit::Nine)),
            _ => Err(()),
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
            impl From<BaselineDigit> for $t {
                #[inline]
                fn from(digit: BaselineDigit) -> Self {
                    Self::from(digit.0)
                }
            }
            impl From<SubscriptDigit> for $t {
                #[inline]
                fn from(digit: SubscriptDigit) -> Self {
                    Self::from(digit.0)
                }
            }
            impl From<SuperscriptDigit> for $t {
                #[inline]
                fn from(digit: SuperscriptDigit) -> Self {
                    Self::from(digit.0)
                }
            }
        )*
    };
}

impl_digit_to_numeric!(u8, u16, u32, i8, i16, i32);

impl Display for SubscriptDigit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

impl Display for SuperscriptDigit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

impl Display for BaselineDigit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

/// Tries to fold the stream of characters into the provided number type.
pub fn try_fold_number<D, C, I>(
    stream: &mut core::iter::Peekable<I>,
) -> Option<Result<D, NumericError>>
where
    D: NumberLike + From<C>,
    I: Iterator<Item = char>,
    C: TryFrom<char>,
{
    let mut amount = if let Some(next_char) = stream.peek().copied()
        && let Ok(digit) = C::try_from(next_char)
    {
        // We consume the peeked character as we have used it to form the digit.
        stream.next();

        // We convert the parsed digit into the provided number type.
        let digit: D = D::from(digit);

        // We check that the digit is not zero to avoid leading zeros in counts.
        // TODO: change how this is handled if fractional counts are allowed.
        if digit.is_zero() {
            return Some(Err(NumericError::LeadingZero));
        }

        digit
    } else {
        return None;
    };

    while let Some(next_char) = stream.peek().copied()
        && let Ok(digit) = C::try_from(next_char)
    {
        // We consume the peeked character as we have used it to form the digit.
        stream.next();

        // We convert the parsed digit into the provided number type.
        let digit: D = D::from(digit);

        // We update the amount by multiplying the current amount by 10
        // and adding the new digit, checking for overflows.
        if let Some(updated_amount) =
            amount.checked_mul(&D::TEN).and_then(|v| v.checked_add(&digit))
        {
            amount = updated_amount;
        } else {
            return Some(Err(NumericError::PositiveOverflow));
        }
    }

    Some(Ok(amount))
}

/// Returns an iterator over the digits of the provided number in
/// left-to-right order.
///
/// # Examples
///
/// ```
/// use molecular_formulas::{Digit, digits_ltr};
///
/// let digits: Vec<Digit> = digits_ltr(123).collect();
/// assert_eq!(digits, vec![Digit::One, Digit::Two, Digit::Three]);
///
/// let digits: Vec<Digit> = digits_ltr(0).collect();
/// assert_eq!(digits, vec![Digit::Zero]);
///
/// let digits: Vec<Digit> = digits_ltr(10).collect();
/// assert_eq!(digits, vec![Digit::One, Digit::Zero]);
/// ```
pub fn digits_ltr<D: Into<i64>>(number: D) -> impl Iterator<Item = Digit> {
    let mut number: i64 = number.into();
    number = number.abs();

    let mut div = 1;
    while number / 10 >= div {
        div *= 10;
    }

    core::iter::from_fn(move || {
        if div == 0 {
            None
        } else {
            let d = number / div;
            number %= div;
            div /= 10;
            Some(if d == 0 {
                Digit::Zero
            } else if d == 1 {
                Digit::One
            } else if d == 2 {
                Digit::Two
            } else if d == 3 {
                Digit::Three
            } else if d == 4 {
                Digit::Four
            } else if d == 5 {
                Digit::Five
            } else if d == 6 {
                Digit::Six
            } else if d == 7 {
                Digit::Seven
            } else if d == 8 {
                Digit::Eight
            } else {
                Digit::Nine
            })
        }
    })
}

/// Returns an iterator over the characters of the provided number in
/// left-to-right order, in superscript form.
///
/// # Examples
///
/// ```
/// use molecular_formulas::superscript_digits_ltr;
///
/// let digits: String = superscript_digits_ltr(123).collect();
/// assert_eq!(digits, "¹²³");
///
/// let digits: String = superscript_digits_ltr(0).collect();
/// assert_eq!(digits, "⁰");
///
/// let digits: String = superscript_digits_ltr(105).collect();
/// assert_eq!(digits, "¹⁰⁵");
/// ```
pub fn superscript_digits_ltr<D: Into<i64>>(number: D) -> impl Iterator<Item = char> {
    digits_ltr(number).map(|d| SuperscriptDigit(d).into())
}

/// Returns an iterator over the characters of the provided number in
/// left-to-right order, in subscript form.
///
/// # Examples
///
/// ```
/// use molecular_formulas::subscript_digits_ltr;
///
/// let digits: String = subscript_digits_ltr(123).collect();
/// assert_eq!(digits, "₁₂₃");
///
/// let digits: String = subscript_digits_ltr(0).collect();
/// assert_eq!(digits, "₀");
///
/// let digits: String = subscript_digits_ltr(109).collect();
/// assert_eq!(digits, "₁₀₉");
/// ```
pub fn subscript_digits_ltr<D: Into<i64>>(number: D) -> impl Iterator<Item = char> {
    digits_ltr(number).map(|d| SubscriptDigit(d).into())
}
