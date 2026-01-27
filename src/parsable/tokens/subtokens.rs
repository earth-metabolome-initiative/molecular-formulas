//! Submodule creating the `TokenIter` struct, which is an iterator over
//! the `Token`s found in a provided string.

use core::{fmt::Display, iter::Peekable};

use elements_rs::{Element, isotopes::HydrogenIsotope};
use num_traits::{CheckedAdd, CheckedNeg, ConstOne, One, Signed};

mod complex;
pub use complex::Complex;
mod typesetting;
pub use typesetting::{Baseline, Subscript, Superscript, TypeSetting};

use crate::{
    ChargedMolecularFormulaMetadata, display_charge, display_isotope,
    errors::{NumericError, ParserError},
    parsable::tokens::inchi_tokens::InchiToken,
    prelude::Radical,
};

/// Marker trait for typesettings that support charge notation.
pub trait ChargeLike: NumberLike + Signed + CheckedNeg + Into<i32> + TryFrom<i64> {}
impl<T> ChargeLike for T where T: NumberLike + Signed + CheckedNeg + Into<i32> + TryFrom<i64> {}

mod brackets;
mod digits;
pub use brackets::Bracket;
pub use digits::*;
mod markers;
pub use markers::{
    BaselineMinus, BaselinePlus, CharacterMarker, Dot, SignCharacter, SignMarker, SuperscriptMinus,
    SuperscriptPlus,
};

/// Enumeration of allowed characters in a molecular formula.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum SubToken<Count: CountLike, Charge: ChargeLike, Extension> {
    /// Enumeration of characters allowed in InChI strings.
    Inchi(InchiToken<Count>),
    /// Hydrogen isotope marker. No other isotopes can be represented
    /// with a single character.
    HydrogenIsotope(HydrogenIsotope),
    /// A radical marker.
    Radical,
    /// A charge sign.
    Charge(Charge),
    /// A complex group.
    Complex(Complex),
    /// A Superscript digit.
    SuperscriptDigit(Count),
    /// An open bracket.
    OpenBracket(Bracket),
    /// A closed bracket.
    CloseBracket(Bracket),
    /// An extension token.
    Extension(Extension),
}

impl<Count: CountLike, Charge: ChargeLike, Extension> Display for SubToken<Count, Charge, Extension>
where
    Extension: Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SubToken::Inchi(token) => write!(f, "{token}"),
            SubToken::HydrogenIsotope(isotope) => display_isotope((*isotope).into(), f),
            SubToken::Radical => write!(f, "•"),
            SubToken::Charge(charge) => display_charge(*charge, f),
            SubToken::Complex(complex) => write!(f, "{complex}"),
            SubToken::SuperscriptDigit(count) => {
                for digit_char in superscript_digits_ltr(*count) {
                    write!(f, "{digit_char}")?;
                }
                Ok(())
            }
            SubToken::OpenBracket(bracket) => write!(f, "{}", bracket.opening()),
            SubToken::CloseBracket(bracket) => write!(f, "{}", bracket.closing()),
            SubToken::Extension(extension) => write!(f, "{extension}"),
        }
    }
}

impl<Count: CountLike, Charge: ChargeLike, Extension> From<Complex>
    for SubToken<Count, Charge, Extension>
{
    fn from(complex: Complex) -> Self {
        SubToken::Complex(complex)
    }
}

impl<Count: CountLike, Charge: ChargeLike, Extension> From<InchiToken<Count>>
    for SubToken<Count, Charge, Extension>
{
    fn from(token: InchiToken<Count>) -> Self {
        SubToken::Inchi(token)
    }
}

impl<Count: CountLike, Charge: ChargeLike, Extension> From<Dot>
    for SubToken<Count, Charge, Extension>
{
    fn from(_: Dot) -> Self {
        InchiToken::from(Dot).into()
    }
}

impl<Count: CountLike, Charge: ChargeLike, Extension> From<Element>
    for SubToken<Count, Charge, Extension>
{
    fn from(element: Element) -> Self {
        InchiToken::from(element).into()
    }
}

impl<Count: CountLike, Charge: ChargeLike, Extension> From<HydrogenIsotope>
    for SubToken<Count, Charge, Extension>
{
    fn from(hydrogen_isotope: HydrogenIsotope) -> Self {
        SubToken::HydrogenIsotope(hydrogen_isotope)
    }
}

impl<Count: CountLike, Charge: ChargeLike, Extension> From<Radical>
    for SubToken<Count, Charge, Extension>
{
    fn from(_: Radical) -> Self {
        SubToken::Radical
    }
}

/// Iterator over the `Token`s found in a provided string.
pub struct SubTokens<I: Iterator<Item = char>, M, Extension> {
    /// A peekable iterator over the allowed characters.
    stream: core::iter::Peekable<I>,
    /// Phantom data for the molecular formula type.
    _marker: core::marker::PhantomData<M>,
    /// Phantom data for the extension type.
    _extension: core::marker::PhantomData<Extension>,
}

impl<I: Iterator<Item = char>, M, Extension> From<Peekable<I>> for SubTokens<I, M, Extension> {
    fn from(iter: Peekable<I>) -> Self {
        Self {
            stream: iter,
            _marker: core::marker::PhantomData,
            _extension: core::marker::PhantomData,
        }
    }
}

impl<I: Iterator<Item = char>, M: ChargedMolecularFormulaMetadata, Extension>
    SubTokens<I, M, Extension>
{
    /// Parses a charge.
    fn parse_charge<CS: SignMarker>(&mut self) -> Result<M::Charge, NumericError>
    where
        M::Charge: From<CS::Digit>,
    {
        // There might be one of more signs in some notations.
        let mut sign_count: M::Charge = <M::Charge as ConstOne>::ONE;
        while self.stream.peek().copied().is_some_and(|c| CS::matches(c)) {
            sign_count = sign_count
                .checked_add(&<M::Charge as ConstOne>::ONE)
                .ok_or(NumericError::PositiveOverflow)?;
            self.stream.next();
        }

        // If the sign count is, in absolute value, equal to one, it may be followed
        // by an optional number.
        if sign_count.abs().is_one()
            && let Some(count) = try_fold_number::<M::Charge, CS::Digit, _>(&mut self.stream)
        {
            sign_count = count?;
        }

        // We adjust the sign of the charge according to the sign marker.
        if !CS::POSITIVE {
            sign_count = sign_count.checked_neg().ok_or(NumericError::NegativeOverflow)?;
        }

        Ok(sign_count)
    }

    fn parse_charge_token<CS: SignMarker>(
        &mut self,
    ) -> Result<SubToken<M::Count, M::Charge, Extension>, ParserError>
    where
        M::Charge: From<CS::Digit>,
    {
        let charge = self.parse_charge::<CS>()?;
        // Charges cannot be immediately followed by another charge or digit.
        if self.parse_any_illegal_charge_successor() {
            return Err(ParserError::UnexpectedCharacter(self.stream.next().unwrap()));
        }
        Ok(SubToken::Charge(charge))
    }

    /// Returns whether any charge or superscript digit can be parsed next.
    fn parse_any_illegal_charge_successor(&mut self) -> bool {
        if let Some(c) = self.stream.peek().copied() {
            SuperscriptMinus::matches(c)
                || SuperscriptPlus::matches(c)
                || BaselinePlus::matches(c)
                || BaselineMinus::matches(c)
                || SuperscriptDigit::try_from(c).is_ok()
        } else {
            false
        }
    }
}

impl<I: Iterator<Item = char>, M: ChargedMolecularFormulaMetadata, Extension> Iterator
    for SubTokens<I, M, Extension>
where
    Extension: TryFrom<char>,
{
    type Item = Result<SubToken<M::Count, M::Charge, Extension>, ParserError>;

    #[allow(clippy::too_many_lines)]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(count) = try_fold_number::<M::Count, BaselineDigit, _>(&mut self.stream) {
            // If we have found a baseline number, we return it as a count token.
            // But first, we check that it is not further followed by a subscript digit,
            // which would indicate an incorrect formula.
            if self.stream.peek().copied().is_some_and(|c| SubscriptDigit::try_from(c).is_ok()) {
                return Some(Err(ParserError::UnexpectedCharacter(self.stream.next().unwrap())));
            }

            return Some(count.map(|c| InchiToken::Count(c).into()).map_err(Into::into));
        }
        if let Some(count) = try_fold_number::<M::Count, SubscriptDigit, _>(&mut self.stream) {
            // If we have found a subscript number, we return it as a count token.
            // But first, we check that it is not further followed by a baseline digit,
            // which would indicate an incorrect formula.
            if self.stream.peek().copied().is_some_and(|c| BaselineDigit::try_from(c).is_ok()) {
                return Some(Err(ParserError::UnexpectedCharacter(self.stream.next().unwrap())));
            }

            return Some(count.map(|c| InchiToken::Count(c).into()).map_err(Into::into));
        }
        if let Some(count) = try_fold_number::<M::Count, SuperscriptDigit, _>(&mut self.stream) {
            let count = match count {
                Ok(c) => c,
                Err(e) => return Some(Err(e.into())),
            };
            return Some(match self.stream.peek().copied() {
                Some(c) if SuperscriptMinus::matches(c) => {
                    self.stream.next();

                    // We check that no further charge or digit follows.
                    if self.parse_any_illegal_charge_successor() {
                        return Some(Err(ParserError::UnexpectedCharacter(
                            self.stream.next().unwrap(),
                        )));
                    }

                    let mut padded_count: i64 = count.into();
                    // Should not be possible to overflow here.
                    padded_count = -padded_count;
                    M::Charge::try_from(padded_count)
                        .map_err(|_| NumericError::NegativeOverflow.into())
                        .map(|ch| SubToken::Charge(ch))
                }
                Some(c) if SuperscriptPlus::matches(c) => {
                    self.stream.next();

                    // We check that no further charge or digit follows.
                    if self.parse_any_illegal_charge_successor() {
                        return Some(Err(ParserError::UnexpectedCharacter(
                            self.stream.next().unwrap(),
                        )));
                    }

                    M::Charge::try_from(count)
                        .map_err(|_| NumericError::PositiveOverflow.into())
                        .map(|ch| SubToken::Charge(ch))
                }
                _ => Ok(SubToken::SuperscriptDigit(count)),
            });
        }

        let next_char = self.stream.next()?;

        if let Some(peaked) = self.stream.peek().copied() {
            if let Ok(complex) = Complex::try_from([next_char, peaked]) {
                self.stream.next();
                return Some(Ok(complex.into()));
            }
            if let Ok(element) = Element::try_from([next_char, peaked]) {
                self.stream.next();
                return Some(Ok(element.into()));
            }
        }

        if let Ok(element) = Element::try_from(next_char) {
            return Some(Ok(element.into()));
        }

        if Dot::matches(next_char) {
            return Some(Ok(Dot.into()));
        }

        if Radical::matches(next_char) {
            // We check that the radical is not repeated.
            if self.stream.peek().copied().is_some_and(Radical::matches) {
                return Some(Err(ParserError::UnexpectedCharacter(self.stream.next().unwrap())));
            }

            return Some(Ok(Radical.into()));
        }

        if SuperscriptMinus::matches(next_char) {
            return Some(self.parse_charge_token::<SuperscriptMinus>());
        }

        if SuperscriptPlus::matches(next_char) {
            return Some(self.parse_charge_token::<SuperscriptPlus>());
        }

        if BaselinePlus::matches(next_char) {
            return Some(self.parse_charge_token::<BaselinePlus>());
        }

        if BaselineMinus::matches(next_char) {
            return Some(self.parse_charge_token::<BaselineMinus>());
        }

        if let Ok(extension) = Extension::try_from(next_char) {
            return Some(Ok(SubToken::Extension(extension)));
        }

        // All remaining single-character cases.
        match next_char {
            'T' => Some(Ok(HydrogenIsotope::T.into())),
            'D' => Some(Ok(HydrogenIsotope::D.into())),
            '[' => {
                // We check that it is not immediately followed by a closed bracket.
                if self.stream.peek().copied() == Some(']') {
                    return Some(Err(ParserError::UnexpectedCharacter(
                        self.stream.next().unwrap(),
                    )));
                }
                Some(Ok(SubToken::OpenBracket(Bracket::Square)))
            }
            ']' => Some(Ok(SubToken::CloseBracket(Bracket::Square))),
            '(' => {
                // We check that it is not immediately followed by a closed bracket.
                if self.stream.peek().copied() == Some(')') {
                    return Some(Err(ParserError::UnexpectedCharacter(
                        self.stream.next().unwrap(),
                    )));
                }
                Some(Ok(SubToken::OpenBracket(Bracket::Round)))
            }
            ')' => Some(Ok(SubToken::CloseBracket(Bracket::Round))),
            _ => Some(Err(ParserError::UnexpectedCharacter(next_char))),
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use elements_rs::{Element, isotopes::HydrogenIsotope};

    use super::*;
    use crate::parsable::tokens::inchi_tokens::InchiToken;

    #[test]
    fn test_display() {
        assert_eq!(
            SubToken::<u32, i32, char>::Inchi(InchiToken::from(Element::C)).to_string(),
            "C"
        );
        assert_eq!(SubToken::<u32, i32, char>::Inchi(InchiToken::Count(42)).to_string(), "42");
        assert_eq!(
            SubToken::<u32, i32, char>::HydrogenIsotope(HydrogenIsotope::D).to_string(),
            "[²H]"
        );
        assert_eq!(SubToken::<u32, i32, char>::Radical.to_string(), "•");
        assert_eq!(SubToken::<u32, i32, char>::Charge(1).to_string(), "⁺");
        assert_eq!(SubToken::<u32, i32, char>::Charge(-1).to_string(), "⁻");
        assert_eq!(SubToken::<u32, i32, char>::Charge(2).to_string(), "²⁺");
        assert_eq!(SubToken::<u32, i32, char>::Charge(-2).to_string(), "²⁻");
        assert_eq!(SubToken::<u32, i32, char>::Complex(Complex::Methyl).to_string(), "Me");
        assert_eq!(SubToken::<u32, i32, char>::SuperscriptDigit(5).to_string(), "⁵");
        assert_eq!(SubToken::<u32, i32, char>::OpenBracket(Bracket::Round).to_string(), "(");
        assert_eq!(SubToken::<u32, i32, char>::CloseBracket(Bracket::Square).to_string(), "]");
        assert_eq!(SubToken::<u32, i32, char>::Extension('x').to_string(), "x");
    }
}
