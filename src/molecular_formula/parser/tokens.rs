//! Submodule creating the `Tokens` struct, which is an iterator over
//! the `Token`s found in a provided string.

use elements_rs::{Element, Isotope};

mod subtokens;
pub use subtokens::{
    AllowedCharacter, AllowedCharacterError, Bracket, CharacterMarker, ChargeLike, Complex,
    CountLike, Digit, GreekLetter, NumberLike, Radical, Residual, SubToken, SubTokenError,
    SubTokens, SuperscriptMinus, SuperscriptPlus, Terminator, subscript_digits_ltr,
    superscript_digits_ltr,
};

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a token in a molecular formula. The primary difference
/// between a `Token` and a `SubToken` is that in a `SubToken` the handling
/// of the Isotopes is not yet fully completes, as some `SubToken::Element`
/// may be prefixed by some `SubToken::BaselineNumber` tokens. Still, this
/// parsing will not take into account corner cases like `[13]C` or `C[13]`,
/// where the parsing would require a memory of the previous or next token to
/// correctly parse the isotope, and this aspect is taken care of in the upper
/// parsing layer still.
pub enum Token<Signed, Unsigned> {
    /// An element
    Element(Element),
    /// An isotope
    Isotope(Isotope),
    /// A group fragment, like "Me" for Methyl or "Et" for Ethyl
    Complex(Complex),
    /// A charge
    Charge(Signed),
    /// A subscript number, which may be a count
    Repeat(Unsigned),
    /// A residual group
    Residual,
    /// A radical marker
    Radical,
    /// An open bracket
    OpenBracket(Bracket),
    /// A terminator: either a close bracket or a dot
    Terminator(Terminator),
    /// A greek letter
    Greek(GreekLetter),
}

/// Enumeration of errors which may occur while parsing tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TokenError<Signed, Unsigned> {
    /// Error associated with subtokens.
    #[error("Subtoken error: {0}")]
    SubToken(#[from] SubTokenError),
    /// An error occurred when trying to create an isotope from
    /// an isotopic number and an element.
    #[error("Isotope error: {0}")]
    Isotope(#[from] elements_rs::errors::Error),
    /// Unable to assign an atomic count to a token.
    #[error("Unable to assign an isotopic count {0} to a token.")]
    UnableToAssignIsotopicNumber(Unsigned),
    /// Unexpected end of input while parsing tokens.
    #[error("Unexpected end of input while parsing tokens.")]
    UnexpectedEndOfInputWhileParsingTokens,
    /// Unexpected terminator while parsing tokens.
    #[error("Unexpected terminator `{0}` while parsing tokens.")]
    UnexpectedTerminatorWhileParsingTokens(Terminator),
    /// Invalid subsequent tokens
    #[error("The subtoken `{0}` cannot be followed by the subtoken `{1}`.")]
    InvalidSuccessor(SubToken<Signed, Unsigned>, SubToken<Signed, Unsigned>),
}

/// Iterator over the `Token`s found in a provided string.
pub struct Tokens<'a, Signed: ChargeLike + TryFrom<Unsigned>, Unsigned: NumberLike> {
    /// A peekable iterator over the allowed characters.
    chars: core::iter::Peekable<SubTokens<'a, Signed, Unsigned>>,
}

impl<'a, Signed: ChargeLike + TryFrom<Unsigned>, Unsigned: NumberLike> From<&'a str>
    for Tokens<'a, Signed, Unsigned>
{
    fn from(s: &'a str) -> Self {
        Tokens { chars: SubTokens::from(s).peekable() }
    }
}

impl<Signed: ChargeLike + TryFrom<Unsigned>, Unsigned: NumberLike> Iterator
    for Tokens<'_, Signed, Unsigned>
where
    Isotope: TryFrom<(Element, Unsigned), Error = elements_rs::errors::Error>,
{
    type Item = Result<Token<Signed, Unsigned>, TokenError<Signed, Unsigned>>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.chars.next() {
            Some(Ok(subtoken)) => subtoken,
            Some(Err(e)) => return Some(Err(e.into())),
            None => return None,
        };

        Some(Ok(match next {
            SubToken::Radical => Token::Radical,
            SubToken::Element(element) => Token::Element(element),
            SubToken::SuperscriptNumber(isotopic_number) => {
                // A superscript number must be followed by an element to be valid,
                // and be the isotopic number of that element.
                let next = match self.chars.next() {
                    Some(Ok(subtoken)) => subtoken,
                    Some(Err(e)) => return Some(Err(e.into())),
                    None => {
                        return Some(Err(TokenError::UnexpectedEndOfInputWhileParsingTokens));
                    }
                };
                if let SubToken::Element(element) = next {
                    match Isotope::try_from((element, isotopic_number)) {
                        Ok(isotope) => Token::Isotope(isotope),
                        Err(e) => return Some(Err(e.into())),
                    }
                } else {
                    return Some(Err(TokenError::UnableToAssignIsotopicNumber(isotopic_number)));
                }
            }
            SubToken::Charge(charge) => {
                // A charge cannot be followed by another charge
                if let Some(Ok(
                    token @ (SubToken::Charge(_)
                    | SubToken::BaselineNumber(_)
                    | SubToken::SuperscriptNumber(_)),
                )) = self.chars.peek().copied()
                {
                    return Some(Err(TokenError::InvalidSuccessor(
                        SubToken::Charge(charge),
                        token,
                    )));
                }
                Token::Charge(charge)
            }
            count_token @ (SubToken::SubscriptNumber(count) | SubToken::BaselineNumber(count)) => {
                if let Some(Ok(
                    token @ (SubToken::BaselineNumber(_) | SubToken::SubscriptNumber(_)),
                )) = self.chars.peek().copied()
                {
                    return Some(Err(TokenError::InvalidSuccessor(count_token, token)));
                }

                Token::Repeat(count)
            }
            SubToken::Isotope(isotope) => Token::Isotope(isotope),
            SubToken::Complex(complex) => Token::Complex(complex),
            SubToken::Residual => Token::Residual,
            SubToken::OpenBracket(bracket) => Token::OpenBracket(bracket),
            SubToken::Terminator(terminator) => Token::Terminator(terminator),
            SubToken::GreekLetter(greek_letter) => Token::Greek(greek_letter),
        }))
    }
}
