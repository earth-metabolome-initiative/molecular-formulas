//! Submodule providing a parser for the `MolecularFormula` struct

use std::iter::Peekable;

use elements_rs::{Element, Isotope};
use num_traits::{ConstOne, ConstZero};

use crate::{
    MolecularFormula,
    molecular_formula::{Tree, trees::InstantiableTree},
};

mod tokens;
use thiserror::Error;
pub use tokens::{
    AllowedCharacter, AllowedCharacterError, Bracket, CharacterMarker, ChargeLike, Complex,
    CountLike, Digit, GreekLetter, Radical, Residual, SubToken, SubTokenError, SuperscriptMinus,
    SuperscriptPlus, Terminator, Token, TokenError, Tokens,
};
pub(super) use tokens::{subscript_digits_ltr, superscript_digits_ltr};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enumeration of errors which may occur while parsing a molecular formula.
pub enum ParseError<Signed, Unsigned> {
    /// Error regarding the parsing of tokens.
    #[error("Token error: {0}")]
    Token(#[from] TokenError<Signed, Unsigned>),
    /// Unable to continue parsing.
    #[error("Unable to continue parsing, there is more tokens than expected.")]
    UnableToContinueParsing,
    /// Residual not supported in the selected `Tree` implementation.
    #[error("Residual not supported in the selected Tree implementation - change selected tree.")]
    ResidualNotSupportedInCurrentTree,
    /// Empty sequence cannot be parsed into the selected `Tree` implementation.
    #[error("Empty sequence cannot be parsed into the selected Tree implementation.")]
    EmptySequenceNotSupportedInCurrentTree,
    /// Encountered greek letter in unexpected position.
    #[error("Encountered greek letter `{0}` in unexpected position.")]
    UnexpectedGreekLetter(GreekLetter),
    /// Repeat count cannot be zero.
    #[error("Repeat count cannot be zero.")]
    RepeatCountCannotBeZero,
    /// Charge cannot be zero.
    #[error("Charge cannot be zero.")]
    ChargeCannotBeZero,
}

impl<S, U> From<elements_rs::errors::Error> for ParseError<S, U> {
    fn from(e: elements_rs::errors::Error) -> Self {
        ParseError::Token(TokenError::Isotope(e))
    }
}

impl<S, U> From<SubTokenError> for ParseError<S, U> {
    fn from(e: SubTokenError) -> Self {
        ParseError::Token(TokenError::SubToken(e))
    }
}

/// Type alias for the token type used in the parser.
pub type ParserToken<T> = Token<<T as Tree>::Signed, <T as Tree>::Unsigned>;

/// Type alias for the error type used in the parser.
pub type ParserError<T> = ParseError<<T as Tree>::Signed, <T as Tree>::Unsigned>;

/// Parser for the `MolecularFormula` struct
pub struct Parser<'a, T: Tree>
where
    Isotope: TryFrom<(Element, T::Unsigned), Error = elements_rs::errors::Error>,
{
    /// Iterator over the tokens.
    tokens: Peekable<Tokens<'a, T::Signed, T::Unsigned>>,
}

impl<'a, T: Tree> From<&'a str> for Parser<'a, T>
where
    Isotope: TryFrom<(Element, T::Unsigned), Error = elements_rs::errors::Error>,
{
    fn from(s: &'a str) -> Self {
        Parser { tokens: Tokens::from(s).peekable() }
    }
}

impl<T: InstantiableTree> Parser<'_, T>
where
    Isotope: TryFrom<(Element, T::Unsigned), Error = elements_rs::errors::Error>,
{
    pub(super) fn parse(&mut self) -> Result<MolecularFormula<T>, ParserError<T>> {
        // The greek letter descriptor, if any, is at the beginning of the iterator
        let maybe_greek_letter = if let Some(Token::Greek(greek)) = self.peek()? {
            // Consume the greek letter
            self.next()?;
            // Parse the mixture
            Some(greek)
        } else {
            None
        };

        // Next, we start to parse the mixtures, which are separated by dots.
        let mut mixtures = Vec::new();
        while self.peek()?.is_some() {
            mixtures.push(self.parse_mixture()?);
        }

        Ok(MolecularFormula { greek: maybe_greek_letter, mixtures })
    }

    #[inline]
    fn parse_mixture(&mut self) -> Result<(T::Unsigned, T), ParserError<T>> {
        // If there is a mixture multiplier, it appears at the beginning
        // of the mixture
        let mixture_multiplier = if let Some(Token::Repeat(n)) = self.peek()? {
            // Consume the mixture multiplier
            self.next()?;
            n
        } else {
            T::Unsigned::ONE
        };

        let unit = self.parse_unit(Terminator::Dot, None)?;

        Ok((mixture_multiplier, unit))
    }

    #[inline]
    fn consume_open_square_bracket(&mut self) -> Result<bool, ParserError<T>> {
        if let Some(Token::OpenBracket(Bracket::Square)) = self.peek()? {
            // Consume the open square bracket
            self.tokens.next();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[inline]
    fn consume_close_square_bracket(&mut self) -> Result<bool, ParserError<T>> {
        if let Some(Token::Terminator(Terminator::CloseBracket(Bracket::Square))) = self.peek()? {
            // Consume the close square bracket
            self.tokens.next();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[inline]
    fn consume_element(&mut self) -> Result<Option<Element>, ParserError<T>> {
        if let Some(Token::Element(element)) = self.peek()? {
            // Consume the element
            self.next()?;
            Ok(Some(element))
        } else {
            Ok(None)
        }
    }

    #[inline]
    fn peek(&mut self) -> Result<Option<ParserToken<T>>, ParserError<T>> {
        self.tokens.peek().copied().transpose().map_err(Into::into)
    }

    #[inline]
    fn next(&mut self) -> Result<ParserToken<T>, ParserError<T>> {
        match self.tokens.next() {
            Some(Ok(token)) => Ok(token),
            Some(Err(e)) => Err(e.into()),
            None => Err(TokenError::UnexpectedEndOfInputWhileParsingTokens.into()),
        }
    }

    #[allow(clippy::too_many_lines)]
    fn parse_unit(
        &mut self,
        terminator: Terminator,
        mut initial_token: Option<ParserToken<T>>,
    ) -> Result<T, ParserError<T>> {
        // We initialize an empty tree
        let mut sequence: Vec<T> = Vec::new();

        'unit: loop {
            let next_token = if let Some(pending_token) = initial_token.take() {
                pending_token
            } else {
                let peeked = self.peek()?;

                if peeked == Some(Token::Terminator(terminator)) {
                    self.next()?;
                    // We have reached the end of the unit
                    break 'unit;
                }

                if terminator == Terminator::Dot && peeked.is_none() {
                    // We have reached the end of the input while parsing a mixture
                    break 'unit;
                }

                self.next()?
            };

            match next_token {
                Token::Radical => {
                    // A radical at the beginning of a unit decorates the entire unit
                    // that follows it, while it wraps up the entire unit if it is at
                    // some point inside the unit.
                    if sequence.is_empty() {
                        // If the unit is empty, we parse the following unit
                        // and then decorate it with the radical.
                        sequence.push(self.parse_unit(terminator, None)?.left_radical());
                        // If we are parsing a mixture, we need to stop here
                        break 'unit;
                    }
                    // Otherwise, we decorate the entire unit parsed so far
                    // with the radical.
                    sequence = vec![T::from_iter(sequence)?.right_radical()];
                }
                Token::Element(element) => {
                    // We need to check whether there is an isotope specifier
                    // after the element.
                    if self.consume_open_square_bracket()? {
                        // This might be an isotope specifier, or an unrelated
                        // square bracket group.
                        let next_token = self.next()?;
                        // If the next token is a terminator, we are in an illegal state.
                        if let Token::Terminator(inner_terminator) = next_token {
                            return Err(TokenError::UnexpectedTerminatorWhileParsingTokens(
                                inner_terminator,
                            )
                            .into());
                        }

                        // If the next_token is a count, this could be an isotope
                        // specifier. We need to further check whether the next token
                        // is a closing square bracket.
                        if let Token::Repeat(mass_number) = next_token
                            && self.consume_close_square_bracket()?
                        {
                            // This is indeed an isotope specifier, so we need
                            // to create the isotope.
                            sequence.push(T::isotope(Isotope::try_from((element, mass_number))?));
                        } else {
                            sequence.push(T::element(element));

                            // Otherwise, we are parsing a new square bracket group, from which we
                            // have already consumed the opening square bracket and the first token.
                            let unit = self.parse_unit(
                                Terminator::CloseBracket(Bracket::Square),
                                Some(next_token),
                            )?;
                            if unit.is_leaf() {
                                sequence.push(unit);
                            } else {
                                sequence.push(unit.square());
                            }
                        }
                    } else {
                        // No isotope specifier.
                        sequence.push(T::element(element));
                    }
                }
                Token::Isotope(isotope) => {
                    sequence.push(T::isotope(isotope));
                }
                Token::Complex(complex) => {
                    let complex = T::complex(complex);

                    match complex.uncharge() {
                        Ok((uncharged_complex, charge)) => {
                            sequence.push(uncharged_complex);
                            sequence = vec![T::from_iter(sequence)?.charge(charge)?];
                        }
                        Err(complex) => {
                            sequence.push(complex);
                        }
                    }
                }
                Token::Charge(charge) => {
                    assert_ne!(charge, T::Signed::ZERO);
                    let charged = T::from_iter(sequence)?.charge(charge)?;

                    sequence = match charged.into_sequence() {
                        Ok(formulas) => formulas,
                        Err(tree) => vec![tree],
                    };
                }
                Token::Terminator(inner_terminator) => {
                    return Err(TokenError::UnexpectedTerminatorWhileParsingTokens(
                        inner_terminator,
                    )
                    .into());
                }
                Token::Repeat(repeat) => {
                    // A repeat always decorates the previous unit. Only at the start
                    // of a new mixture can there be a repeat without a preceding unit
                    // which is handled in `Self::parse_mixture`. The only exception is
                    // when this repeat is followed by an `Element`,
                    // in which case it is an isotope specifier.
                    if let Some(last) = sequence.pop() {
                        sequence.push(last.repeat(repeat)?);
                    } else if let Some(element) = self.consume_element()? {
                        sequence.push(T::isotope(Isotope::try_from((element, repeat))?));
                    } else {
                        return Err(TokenError::UnexpectedEndOfInputWhileParsingTokens.into());
                    }
                }
                Token::Residual => {
                    sequence.push(T::residual()?);
                }
                Token::OpenBracket(bracket) => {
                    let unit = self.parse_unit(Terminator::CloseBracket(bracket), None)?;

                    if unit.is_leaf() {
                        sequence.push(unit);
                    } else {
                        match bracket {
                            Bracket::Round => sequence.push(unit.round()),
                            Bracket::Square => sequence.push(unit.square()),
                        }
                    }
                }
                Token::Greek(greek_letter) => {
                    return Err(ParseError::UnexpectedGreekLetter(greek_letter));
                }
            }
        }

        T::from_iter(sequence)
    }
}
