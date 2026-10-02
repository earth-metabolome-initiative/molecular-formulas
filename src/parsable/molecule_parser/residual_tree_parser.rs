//! Submodule providing the `MolecularTreeParser` implementation for
//! molecular formulas which can contain residuals `R`.

use elements_rs::Isotope;

use crate::{
    ChargeLike, ChemicalTree, CountLike, Residual, ResidualFormula, Token,
    errors::ParserError,
    parsable::{MoleculeParser, molecule_parser::MolecularTreeParser},
};

impl<I: Iterator<Item = char>, Count: CountLike, Charge: ChargeLike>
    MolecularTreeParser<Count, ChemicalTree<Count, Charge, Residual>>
    for MoleculeParser<I, ResidualFormula<Count, Charge>>
where
    Charge: TryFrom<Count>,
    Isotope: TryFrom<(elements_rs::Element, Count), Error = elements_rs::errors::Error>,
{
    fn extend_tree(
        &mut self,
        tree: ChemicalTree<Count, Charge, Residual>,
        terminator: Token<Count, Charge, Residual>,
        token: Token<Count, Charge, Residual>,
    ) -> Result<ChemicalTree<Count, Charge, Residual>, ParserError> {
        self.extend_generic_tree(tree, terminator, token)
    }
}
