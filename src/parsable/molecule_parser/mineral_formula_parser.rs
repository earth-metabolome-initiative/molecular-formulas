//! Submodule providing the `MolecularTreeParser` implementation for molecular
//! formulas which can be prefixed by polymorph information, such as mineral
//! formulas.

use elements_rs::Isotope;

use crate::{
    ChargeLike, ChemicalTree, CountLike, Empty, MineralFormula, Token,
    errors::ParserError,
    parsable::{MoleculeParser, molecule_parser::MolecularTreeParser},
};

impl<I: Iterator<Item = char>, Count: CountLike, Charge: ChargeLike>
    MolecularTreeParser<Count, ChemicalTree<Count, Charge, Empty>>
    for MoleculeParser<I, MineralFormula<Count, Charge>>
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
