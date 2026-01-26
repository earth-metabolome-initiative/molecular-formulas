//! Submodule providing an enumeration of the inchi-specific tokens and
//! an iterator to convert a stream of characters into these tokens.

use core::{fmt::Display, iter::Peekable};

use elements_rs::Element;

use crate::{
    BaselineDigit, CountLike, Dot, InChITree, SequenceNode, TokenLike,
    errors::ParserError,
    parsable::{CharacterMarker, ParsableMolecularTree},
    try_fold_number,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of the tokens specific to InChI molecular formulas.
pub enum InchiToken<Count> {
    /// An element symbol.
    Element(Element),
    /// A count (number).
    Count(Count),
    /// A dot (mixture separator).
    Dot,
}

impl<Count> From<Element> for InchiToken<Count> {
    fn from(element: Element) -> Self {
        InchiToken::Element(element)
    }
}

impl<Count> From<Dot> for InchiToken<Count> {
    fn from(_: Dot) -> Self {
        InchiToken::Dot
    }
}

impl<Count: CountLike> TokenLike for InchiToken<Count> {
    type Count = Count;

    fn is_mixture_separator(&self) -> bool {
        matches!(self, InchiToken::Dot)
    }

    fn mixture_separator() -> Self {
        InchiToken::Dot
    }

    fn as_count(&self) -> Option<Count> {
        match self {
            InchiToken::Count(count) => Some(count.clone()),
            _ => None,
        }
    }

    fn as_element(&self) -> Option<elements_rs::Element> {
        match self {
            InchiToken::Element(element) => Some(*element),
            _ => None,
        }
    }
}

pub(crate) struct InchiTokens<I: Iterator<Item = char>, Count> {
    stream: Peekable<I>,
    _marker: core::marker::PhantomData<Count>,
}

impl<I: Iterator<Item = char>, Count: CountLike> Iterator for InchiTokens<I, Count> {
    type Item = Result<InchiToken<Count>, ParserError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(count) = try_fold_number::<Count, BaselineDigit, _>(&mut self.stream) {
            return Some(count.map(|c| InchiToken::Count(c)).map_err(Into::into));
        }
        let next_char = match Element::try_from_stream(&mut self.stream) {
            Ok(Some(element)) => {
                return Some(Ok(element.into()));
            }
            Ok(None) => {
                // The stream has ended
                return None;
            }
            Err(e) => e,
        };

        if Dot::matches(next_char) {
            return Some(Ok(Dot.into()));
        }

        Some(Err(ParserError::UnexpectedCharacter(next_char)))
    }
}

impl<I: Iterator<Item = char>, Count> From<Peekable<I>> for InchiTokens<I, Count> {
    fn from(stream: Peekable<I>) -> Self {
        Self { stream, _marker: core::marker::PhantomData }
    }
}

impl<Count: CountLike> ParsableMolecularTree<Count> for SequenceNode<InChITree<Count>> {
    type Token = InchiToken<Count>;
    type Tokens<I>
        = InchiTokens<I, Count>
    where
        I: Iterator<Item = char>;

    fn empty() -> Self {
        SequenceNode::empty()
    }

    fn is_empty(&self) -> bool {
        SequenceNode::is_empty(self)
    }

    fn element(mut self, element: Element) -> Self {
        self.push(element.into());
        self
    }
}

impl<Count: CountLike> Display for InchiToken<Count> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            InchiToken::Element(e) => write!(f, "{e}"),
            InchiToken::Count(c) => write!(f, "{c}"),
            InchiToken::Dot => write!(f, "."),
        }
    }
}