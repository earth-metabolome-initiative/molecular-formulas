//! Submodule providing the `MolecularTreeParser` implementation for common
//! molecular tree formats, such as the `PubChem` chemical formula format.

use core::fmt::Debug;

use elements_rs::Isotope;

use crate::{
    Bracket, ChargeLike, ChargedMolecularFormulaMetadata, ChemicalFormula, ChemicalTree, CountLike,
    Empty, InchiToken, MolecularFormulaMetadata, ParsableFormula, Token,
    errors::ParserError,
    parsable::{
        MoleculeParser, molecule_parser::MolecularTreeParser,
        parsable_molecular_tree::ParsableMolecularTree,
    },
};

impl<I: Iterator<Item = char>, M> MoleculeParser<I, M>
where
    M: ParsableFormula + ChargedMolecularFormulaMetadata,
    M::Charge: TryFrom<M::Count>,
    Isotope: TryFrom<(elements_rs::Element, M::Count), Error = elements_rs::errors::Error>,
{
    pub(crate) fn extend_generic_tree<Count: CountLike, Charge: ChargeLike, Extension>(
        &mut self,
        mut tree: ChemicalTree<Count, Charge, Extension>,
        terminator: Token<Count, Charge, Extension>,
        token: Token<Count, Charge, Extension>,
    ) -> Result<ChemicalTree<Count, Charge, Extension>, ParserError>
    where
        Extension: Debug + Copy + Eq,
        M: MolecularFormulaMetadata<Count = Count>
            + ChargedMolecularFormulaMetadata<Charge = Charge>,
        M: ParsableFormula<Tree = ChemicalTree<Count, Charge, Extension>>,
        ChemicalTree<Count, Charge, Extension>:
            ParsableMolecularTree<Count, Token = Token<Count, Charge, Extension>>,
        Self: MolecularTreeParser<Count, ChemicalTree<Count, Charge, Extension>>,
    {
        Ok(match token {
            Token::Inchi(InchiToken::Element(element)) => {
                if self.peek_token()? == Some(Token::OpenBracket(Bracket::Square)) {
                    // This might be an isotope specifier, or an unrelated
                    // square bracket group.
                    self.consume_token()?; // Consume the opening square bracket

                    // If the next_token is a count, this could be an isotope
                    // specifier. We need to further check whether the next token
                    // is a closing square bracket.
                    if let Some(mass_number) = self.consume_count()? {
                        if self.peek_token()? == Some(Token::CloseBracket(Bracket::Square)) {
                            tree = tree.isotope(Isotope::try_from((element, mass_number))?);
                            self.consume_token()?; // Consume the closing square bracket
                            tree
                        } else {
                            tree = tree.element(element);

                            // Otherwise, we are parsing a new square bracket group, from which we
                            // have already consumed the opening square bracket and the first token.
                            let new_tree = self.parse_sequence(
                                Token::CloseBracket(Bracket::Square),
                                Some(Token::Inchi(InchiToken::Count(mass_number))),
                            )?;

                            // And we consume the closing square bracket
                            if self.consume_token()? != Token::CloseBracket(Bracket::Square) {
                                return Err(ParserError::MissingClosingBracket(Bracket::Square));
                            }

                            tree.push(if new_tree.is_leaf() { new_tree } else { new_tree.square() })
                        }
                    } else {
                        tree = tree.element(element);

                        // Otherwise, we are parsing a new square bracket group, from which we
                        // have already consumed the opening square bracket and the first token.
                        let new_tree =
                            self.parse_sequence(Token::CloseBracket(Bracket::Square), None)?;

                        // And we consume the closing square bracket
                        if self.consume_token()? != Token::CloseBracket(Bracket::Square) {
                            return Err(ParserError::MissingClosingBracket(Bracket::Square));
                        }

                        tree.push(if new_tree.is_leaf() { new_tree } else { new_tree.square() })
                    }
                } else {
                    // No isotope specifier.
                    tree.element(element)
                }
            }
            Token::Inchi(InchiToken::Count(count)) => {
                // A repeat always decorates the previous unit. Only at the start
                // of a new mixture can there be a repeat without a preceding unit
                // which is handled in `Self::parse_mixture`. The only exception is
                // when this repeat is followed by an `Element`,
                // in which case it is an isotope specifier.
                if !tree.is_empty() {
                    tree.repeat(count)
                } else if let Some(element) = self.consume_element()? {
                    tree.isotope(Isotope::try_from((element, count))?)
                } else {
                    return Err(ParserError::UnprocessableNumber);
                }
            }
            Token::Radical => {
                // A radical at the beginning of a unit decorates the entire unit
                // that follows it, while it wraps up the entire unit if it is at
                // some point inside the unit.
                if tree.is_empty() {
                    // If the unit is empty, we parse the following unit
                    // and then decorate it with the radical.
                    self.parse_sequence(terminator, None)?.left_radical()
                } else {
                    // If the unit is not empty, we decorate the entire
                    // unit with the radical.
                    tree.right_radical()
                }
            }
            Token::Isotope(isotope) => tree.isotope(isotope),
            Token::Complex(complex) => tree.complex(complex),
            Token::Charge(charge) => tree.charge(charge)?,
            Token::OpenBracket(bracket) => {
                let new_tree = self.parse_sequence(Token::CloseBracket(bracket), None)?;

                // There must be a closing bracket here
                if self.consume_token()? != Token::CloseBracket(bracket) {
                    return Err(ParserError::MissingClosingBracket(bracket));
                }

                tree.push(match bracket {
                    Bracket::Round => new_tree.round(),
                    Bracket::Square => new_tree.square(),
                })
            }
            Token::CloseBracket(bracket) => {
                return Err(ParserError::UnexpectedCharacter(bracket.closing()));
            }
            Token::Inchi(InchiToken::Dot) => {
                return Err(ParserError::UnexpectedCharacter('.'));
            }
            Token::Extension(extension) => tree.extension(extension),
        })
    }
}

impl<I: Iterator<Item = char>, Count: CountLike, Charge: ChargeLike>
    MolecularTreeParser<Count, ChemicalTree<Count, Charge, Empty>>
    for MoleculeParser<I, ChemicalFormula<Count, Charge>>
where
    Charge: TryFrom<Count>,
    Isotope: TryFrom<(elements_rs::Element, Count), Error = elements_rs::errors::Error>,
{
    fn extend_tree(
        &mut self,
        tree: ChemicalTree<Count, Charge, Empty>,
        terminator: Token<Count, Charge, Empty>,
        token: Token<Count, Charge, Empty>,
    ) -> Result<ChemicalTree<Count, Charge, Empty>, ParserError> {
        self.extend_generic_tree(tree, terminator, token)
    }
}
