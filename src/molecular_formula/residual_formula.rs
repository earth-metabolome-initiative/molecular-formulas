//! Submodule defining the expression syntax trees for molecular formulas
//! as found in certain specialized contexts. This format includes residual
//! notations like `R` used in specific scientific fields.

use alloc::vec::Vec;

use elements_rs::Isotope;

use crate::{
    ChargeLike, ChargedMolecularFormulaMetadata, ChemicalTree, CountLike, MolecularFormulaMetadata,
    ParsableFormula, Residual,
};

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A chemical formula which can contain residual notations.
pub struct ResidualFormula<Count: CountLike = u16, Charge: ChargeLike = i16> {
    mixtures: Vec<(Count, ChemicalTree<Count, Charge, Residual>)>,
}

impl<Count: CountLike, Charge: ChargeLike> ResidualFormula<Count, Charge> {
    /// Checks if the formula contains any residual notations.
    pub fn contains_residuals(&self) -> bool {
        for (_fraction, tree) in &self.mixtures {
            if tree.contains_extension() {
                return true;
            }
        }
        false
    }
}

impl<Count: CountLike, Charge: ChargeLike> MolecularFormulaMetadata
    for ResidualFormula<Count, Charge>
{
    type Count = Count;
}

impl<Count: CountLike, Charge: ChargeLike> ChargedMolecularFormulaMetadata
    for ResidualFormula<Count, Charge>
where
    Charge: TryFrom<Count>,
{
    type Charge = Charge;
}

impl<Count: CountLike, Charge: ChargeLike> ParsableFormula for ResidualFormula<Count, Charge>
where
    Isotope: TryFrom<(elements_rs::Element, Count), Error = elements_rs::errors::Error>,
    Charge: TryFrom<Count>,
{
    type StartOutput = ();
    type Tree = ChemicalTree<Count, Charge, Residual>;

    fn on_start<J>(
        _chars: &mut core::iter::Peekable<J>,
    ) -> Result<Self::StartOutput, crate::errors::ParserError>
    where
        J: Iterator<Item = char>,
    {
        Ok(())
    }

    fn from_parsed(
        _start_output: Self::StartOutput,
        mixtures: Vec<(Count, Self::Tree)>,
    ) -> Result<Self, crate::errors::ParserError> {
        assert!(mixtures.len() > 0, "At least one mixture is required");
        Ok(Self { mixtures })
    }
}

impl<Count: CountLike, Charge: ChargeLike> core::fmt::Display for ResidualFormula<Count, Charge>
where
    Charge: TryFrom<Count>,
    ChemicalTree<Count, Charge, Residual>: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for (i, (fraction, tree)) in self.mixtures.iter().enumerate() {
            if i > 0 {
                write!(f, ".")?;
            }
            if *fraction != Count::one() {
                write!(f, "{}", fraction)?;
            }
            write!(f, "{}", tree)?;
        }
        Ok(())
    }
}
