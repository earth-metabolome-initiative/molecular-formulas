//! Submodule providing the `MolecularTreeParser` implementation for InChI
//! trees.

use crate::{
    CountLike, InChIFormula, InChITree, InchiToken, RepeatNode, SequenceNode,
    errors::ParserError,
    parsable::{
        MoleculeParser, molecule_parser::MolecularTreeParser,
        parsable_molecular_tree::ParsableMolecularTree,
    },
};

impl<I: Iterator<Item = char>, Count: CountLike> MolecularTreeParser<Count, SequenceNode<InChITree<Count>>>
    for MoleculeParser<I, InChIFormula<Count>>
{
    fn extend_tree(
        &mut self,
        mut tree: SequenceNode<InChITree<Count>>,
        _terminator: InchiToken<Count>,
        token: InchiToken<Count>,
    ) -> Result<SequenceNode<InChITree<Count>>, ParserError> {
        Ok(match token {
            InchiToken::Element(element) => {
                match self.tokens.peek().copied() {
                    Some(Ok(InchiToken::Count(count))) => {
                        // We consume the count token as we have used it.
                        self.tokens.next();
                        tree.push(RepeatNode::new(count.clone(), element).into());
                        tree
                    }
                    _ => tree.element(element),
                }
            }
            InchiToken::Count(_) => {
                unreachable!("Counts should be handled at a higher level than sequence extension")
            }
            InchiToken::Dot => {
                unreachable!(
                    "Mixture separators should be handled at a higher level than sequence extension"
                )
            }
        })
    }
}
