//! Submodule creating the `Tokens` struct, which is an iterator over
//! the `Token`s found in a provided string.

use core::{fmt::Debug, iter::Peekable};

use elements_rs::{Isotope, isotopes::HydrogenIsotope};

mod subtokens;
pub use subtokens::*;
mod inchi_tokens;
pub use inchi_tokens::InchiToken;

use crate::{
    ChargedMolecularFormulaMetadata, ChemicalFormula, ChemicalTree, SequenceNode, TokenLike,
    display_charge, display_isotope, errors::ParserError, parsable::ParsableMolecularTree,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of the tokens used in parsing chemical formulas.
pub enum Token<Count: CountLike, Charge: ChargeLike, Extension> {
    /// An InChI-specific token.
    Inchi(InchiToken<Count>),
    /// An isotope token, such as '[13C]'.
    Isotope(Isotope),
    /// A charge token, such as '+', '2-', etc.
    Charge(Charge),
    /// A complex token, such as "Em" (Ethyl), "Bu" (Butyl), etc.
    Complex(Complex),
    /// A radical token, such as '·'.
    Radical,
    /// An opening bracket token, including '(' or '['.
    OpenBracket(Bracket),
    /// A closing bracket token, including ')' or ']'.
    CloseBracket(Bracket),
    /// An extension token, for any additional extensions.
    Extension(Extension),
}

impl<Count: CountLike, Charge: ChargeLike, Extension> From<elements_rs::Element>
    for Token<Count, Charge, Extension>
{
    fn from(element: elements_rs::Element) -> Self {
        Token::Inchi(InchiToken::Element(element))
    }
}

impl<Count: CountLike, Charge: ChargeLike, Extension> From<elements_rs::Isotope>
    for Token<Count, Charge, Extension>
{
    fn from(isotope: elements_rs::Isotope) -> Self {
        Token::Isotope(isotope)
    }
}

impl<Count: CountLike, Charge: ChargeLike, Extension: Debug + Eq + Copy> TokenLike
    for Token<Count, Charge, Extension>
{
    type Count = Count;

    fn is_mixture_separator(&self) -> bool {
        match self {
            Token::Inchi(token) => token.is_mixture_separator(),
            _ => false,
        }
    }

    fn mixture_separator() -> Self {
        Token::Inchi(InchiToken::mixture_separator())
    }

    fn as_count(&self) -> Option<Count> {
        match self {
            Token::Inchi(token) => token.as_count(),
            _ => None,
        }
    }

    fn as_element(&self) -> Option<elements_rs::Element> {
        match self {
            Token::Inchi(token) => token.as_element(),
            _ => None,
        }
    }
}

impl<Count: CountLike, Charge: ChargeLike, Extension: Debug + Eq + Copy> From<HydrogenIsotope>
    for Token<Count, Charge, Extension>
{
    fn from(hydrogen_isotope: HydrogenIsotope) -> Self {
        Token::Isotope(hydrogen_isotope.into())
    }
}

/// Iterator over the `Token`s found in a provided string.
pub(crate) struct Tokens<I: Iterator<Item = char>, M: ChargedMolecularFormulaMetadata, Extension>
where
    Extension: TryFrom<char>,
{
    /// A peekable iterator over the allowed characters.
    stream: core::iter::Peekable<SubTokens<I, M, Extension>>,
}

impl<I: Iterator<Item = char>, M: ChargedMolecularFormulaMetadata, Extension> From<Peekable<I>>
    for Tokens<I, M, Extension>
where
    Extension: TryFrom<char>,
{
    fn from(iter: Peekable<I>) -> Self {
        Self { stream: SubTokens::from(iter).peekable() }
    }
}

impl<I: Iterator<Item = char>, M: ChargedMolecularFormulaMetadata, Extension: Debug + Copy + Eq>
    Iterator for Tokens<I, M, Extension>
where
    Isotope: TryFrom<(elements_rs::Element, M::Count), Error = elements_rs::errors::Error>,
    Extension: TryFrom<char>,
{
    type Item = Result<Token<M::Count, M::Charge, Extension>, ParserError>;
    fn next(&mut self) -> Option<Self::Item> {
        let next_subtoken = match self.stream.next() {
            Some(Ok(subtoken)) => subtoken,
            Some(Err(e)) => return Some(Err(e)),
            None => return None,
        };

        Some(Ok(match next_subtoken {
            SubToken::Inchi(token) => Token::Inchi(token),
            SubToken::HydrogenIsotope(isotope) => isotope.into(),
            SubToken::Charge(charge) => Token::Charge(charge),
            SubToken::Complex(complex) => Token::Complex(complex),
            SubToken::Radical => Token::Radical,
            SubToken::OpenBracket(bracket) => Token::OpenBracket(bracket),
            SubToken::CloseBracket(bracket) => Token::CloseBracket(bracket),
            SubToken::Extension(extension) => Token::Extension(extension),
            SubToken::SuperscriptDigit(candidate_isotopic_number) => {
                // A superscript number must be followed by an element to be valid,
                // and be the isotopic number of that element.
                let next = match self.stream.next() {
                    Some(Ok(subtoken)) => subtoken,
                    Some(Err(e)) => return Some(Err(e.into())),
                    None => {
                        return Some(Err(ParserError::UnexpectedEndOfInput));
                    }
                };
                if let SubToken::Inchi(InchiToken::Element(element)) = next {
                    match Isotope::try_from((element, candidate_isotopic_number)) {
                        Ok(isotope) => Token::Isotope(isotope),
                        Err(err) => {
                            return Some(Err(err.into()));
                        }
                    }
                } else {
                    return Some(Err(ParserError::UnprocessableNumber));
                }
            }
        }))
    }
}

impl<Count: CountLike, Charge: ChargeLike, Extension: Copy + Debug + Eq>
    ParsableMolecularTree<Count> for ChemicalTree<Count, Charge, Extension>
where
    Isotope: TryFrom<(elements_rs::Element, Count), Error = elements_rs::errors::Error>,
    Charge: TryFrom<Count>,
    Extension: TryFrom<char>,
{
    type Token = Token<Count, Charge, Extension>;
    type Tokens<I>
        = Tokens<I, ChemicalFormula<Count, Charge>, Extension>
    where
        I: Iterator<Item = char>;

    #[inline]
    fn empty() -> Self {
        ChemicalTree::Sequence(SequenceNode::empty())
    }

    #[inline]
    fn is_empty(&self) -> bool {
        match self {
            ChemicalTree::Sequence(sequence) => sequence.is_empty(),
            _ => false,
        }
    }

    #[inline]
    fn element(self, element: elements_rs::Element) -> Self {
        self.push(Self::Element(element))
    }
}

#[cfg(feature = "fuzzing")]
impl<'a, Count, Charge, Extension> arbitrary::Arbitrary<'a> for Token<Count, Charge, Extension>
where
    Count: CountLike + arbitrary::Arbitrary<'a>,
    Charge: ChargeLike + arbitrary::Arbitrary<'a>,
    Extension: arbitrary::Arbitrary<'a>,
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let variant = u.int_in_range(0..=7)?;
        Ok(match variant {
            0 => elements_rs::Element::arbitrary(u)?.into(),
            1 => {
                let count = Count::arbitrary(u)?;
                Token::Inchi(InchiToken::Count(count))
            }
            2 => Token::Inchi(InchiToken::Dot),
            3 => elements_rs::Isotope::arbitrary(u)?.into(),
            4 => Token::Charge(Charge::arbitrary(u)?),
            5 => Token::Complex(Complex::arbitrary(u)?),
            6 => Token::Radical,
            7 => {
                if u.ratio(1, 2)? {
                    Token::OpenBracket(Bracket::arbitrary(u)?)
                } else {
                    Token::CloseBracket(Bracket::arbitrary(u)?)
                }
            }
            _ => Token::Extension(Extension::arbitrary(u)?),
        })
    }
}

impl<Count, Charge, Extension> core::fmt::Display for Token<Count, Charge, Extension>
where
    Count: CountLike + core::fmt::Display,
    Charge: ChargeLike + core::fmt::Display,
    Extension: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Token::Inchi(t) => write!(f, "{t}"),
            Token::Isotope(iso) => display_isotope(iso, f),
            Token::Charge(c) => display_charge(*c, f),
            Token::Complex(c) => write!(f, "{c}"),
            Token::Radical => write!(f, "."), // Radical is dot? Or how is it parsed?
            Token::OpenBracket(b) => write!(f, "{}", b.opening()),
            Token::CloseBracket(b) => write!(f, "{}", b.closing()),
            Token::Extension(e) => write!(f, "{e}"),
        }
    }
}
