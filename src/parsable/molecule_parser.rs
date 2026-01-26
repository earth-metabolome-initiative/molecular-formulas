//! Submodule handling the parsing of streams of characters into molecular
//! formulas.

use alloc::vec::Vec;
use core::iter::Peekable;

use num_traits::ConstOne;

use crate::{
    TokenLike,
    errors::ParserError,
    parsable::{parsable_formula::ParsableFormula, parsable_molecular_tree::ParsableMolecularTree},
};

mod chemical_tree_parser;
mod inchi_tree_parser;
mod mineral_formula_parser;
mod residual_tree_parser;

/// Trait for parsable molecular trees.
pub(crate) trait MolecularTreeParser<Count, T: ParsableMolecularTree<Count>> {
    /// Extends the provided sequence with a new token.
    fn extend_tree(
        &mut self,
        sequence: T,
        terminator: <T as ParsableMolecularTree<Count>>::Token,
        token: <T as ParsableMolecularTree<Count>>::Token,
    ) -> Result<T, ParserError>;
}

pub(super) struct MoleculeParser<I: Iterator<Item = char>, M: ParsableFormula> {
    tokens: Peekable<<M::Tree as ParsableMolecularTree<M::Count>>::Tokens<I>>,
    start_output: M::StartOutput,
}

impl<I: Iterator<Item = char>, M: ParsableFormula> MoleculeParser<I, M>
where
    Self: MolecularTreeParser<M::Count, M::Tree>,
{
    pub(crate) fn new(chars: I) -> Result<Self, ParserError> {
        let mut peekable_chars = chars.peekable();
        let start_output = M::on_start(&mut peekable_chars)?;
        Ok(Self {
            tokens: <M::Tree as ParsableMolecularTree<M::Count>>::Tokens::from(peekable_chars)
                .peekable(),
            start_output,
        })
    }
}

impl<I: Iterator<Item = char>, M: ParsableFormula> MoleculeParser<I, M>
where
    Self: MolecularTreeParser<M::Count, M::Tree>,
{
    /// Peeks at the next token without consuming it.
    #[allow(clippy::type_complexity)]
    fn peek_token(
        &mut self,
    ) -> Result<Option<<M::Tree as ParsableMolecularTree<M::Count>>::Token>, ParserError> {
        match self.tokens.peek().copied() {
            Some(Ok(token)) => Ok(Some(token)),
            Some(Err(e)) => Err(e),
            None => Ok(None),
        }
    }

    /// Consumes and returns the next token.
    fn consume_token(
        &mut self,
    ) -> Result<<M::Tree as ParsableMolecularTree<M::Count>>::Token, ParserError> {
        match self.tokens.next() {
            Some(Ok(token)) => Ok(token),
            Some(Err(e)) => Err(e),
            None => Err(ParserError::UnexpectedEndOfInput),
        }
    }

    /// Consumes a mixture separator and returns whether one was found.
    fn consume_mixture_separator(&mut self) -> Result<bool, ParserError> {
        match self.tokens.next() {
            Some(Ok(token)) => {
                if token.is_mixture_separator() {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Some(Err(e)) => Err(e),
            None => Ok(true), // End of input is also a valid mixture separator
        }
    }

    /// Consumes a count token if present and returns its value.
    fn consume_count(&mut self) -> Result<Option<M::Count>, ParserError> {
        match self.tokens.peek().copied() {
            Some(Ok(token)) => {
                if let Some(count) = token.as_count() {
                    // Consume the count token
                    self.tokens.next();
                    Ok(Some(count))
                } else {
                    Ok(None)
                }
            }
            Some(Err(e)) => Err(e),
            None => Err(ParserError::UnexpectedEndOfInput),
        }
    }

    /// Consumes an element token if present and returns its value.
    pub(crate) fn consume_element(&mut self) -> Result<Option<elements_rs::Element>, ParserError> {
        match self.tokens.peek().copied() {
            Some(Ok(token)) => {
                if let Some(element) = token.as_element() {
                    // Consume the element token
                    self.tokens.next();
                    Ok(Some(element))
                } else {
                    Ok(None)
                }
            }
            Some(Err(e)) => Err(e),
            None => Err(ParserError::UnexpectedEndOfInput),
        }
    }

    pub(super) fn parse_formula(mut self) -> Result<M, ParserError> {
        // Next, we start to parse the mixtures, which are separated by dots.
        let mut mixtures: Vec<(M::Count, M::Tree)> = Vec::new();

        while self.tokens.peek().is_some() {
            // If there is a mixture multiplier, it appears at the beginning
            // of the mixture
            let mixture_multiplier = self.consume_count()?.unwrap_or(<M::Count as ConstOne>::ONE);

            let sequence: M::Tree = self.parse_sequence(<<M::Tree as ParsableMolecularTree<M::Count>>::Token as TokenLike>::mixture_separator(), None)?;

            if !self.consume_mixture_separator()? {
                // if there is no mixture separator, we are in an illegal state.
                todo!("raise an error here");
            }

            mixtures.push((mixture_multiplier, sequence));
        }

        if mixtures.is_empty() {
            return Err(ParserError::EmptyMolecularTree);
        }

        <M as ParsableFormula>::from_parsed(self.start_output, mixtures)
    }

    fn parse_sequence(
        &mut self,
        terminator: <M::Tree as ParsableMolecularTree<M::Count>>::Token,
        mut initial_token: Option<<M::Tree as ParsableMolecularTree<M::Count>>::Token>,
    ) -> Result<M::Tree, ParserError> {
        // We initialize an empty tree
        let mut sequence: M::Tree = M::Tree::empty();

        'unit: loop {
            let next_token = if let Some(pending_token) = initial_token.take() {
                pending_token
            } else {
                let peeked = self.tokens.peek().copied().transpose()?;

                if peeked == Some(terminator)
                    || terminator.is_mixture_separator() && peeked.is_none()
                {
                    // We have reached the end of the unit
                    break 'unit;
                }

                self.tokens.next().transpose()?.ok_or(ParserError::UnexpectedEndOfInput)?
            };

            sequence = self.extend_tree(sequence, terminator, next_token)?;
        }

        if sequence.is_empty() {
            return Err(ParserError::EmptyMolecularTree);
        }

        Ok(sequence)
    }
}
