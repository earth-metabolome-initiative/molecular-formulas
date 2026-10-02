//! Module defining traits and structures for parsing chemical formulas.

use alloc::vec::Vec;
use core::iter::Peekable;

use crate::{MolecularFormulaMetadata, errors::ParserError, parsable::ParsableMolecularTree};

/// Trait defining a parsable chemical formula.
pub(crate) trait ParsableFormula: MolecularFormulaMetadata {
    /// Expected output type of the start hook call.
    type StartOutput;
    /// The molecular tree to construct.
    type Tree: ParsableMolecularTree<Self::Count>;

    /// Creates a new Molecular Formula from the start output and the mixtures.
    fn from_parsed(
        start_output: Self::StartOutput,
        mixtures: Vec<(Self::Count, Self::Tree)>,
    ) -> Result<Self, ParserError>;

    /// Hook called at the start of the parsing process.
    fn on_start<J>(chars: &mut Peekable<J>) -> Result<Self::StartOutput, ParserError>
    where
        J: Iterator<Item = char>;
}
