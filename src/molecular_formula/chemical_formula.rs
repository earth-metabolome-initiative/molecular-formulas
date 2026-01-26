//! Submodule defining the expression syntax trees for molecular formulas
//! as found in resources such as PubChem. This is a more permissive format
//! than InChI, allowing for a wider variety of notations.

use alloc::vec::Vec;
use core::{
    fmt::Display,
    ops::{Add, AddAssign},
};

use elements_rs::Isotope;

use crate::{
    ChargeLike, ChargedMolecularFormulaMetadata, CountLike, Empty, MolecularFormula,
    MolecularFormulaMetadata, ParsableFormula, prelude::ChemicalTree,
};

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord, Hash)]
/// A chemical formula representing molecular formulas
pub struct ChemicalFormula<Count: CountLike = u16, Charge: ChargeLike = i16> {
    mixtures: Vec<(Count, ChemicalTree<Count, Charge, Empty>)>,
}

impl<Count: CountLike, Charge: ChargeLike> Add for ChemicalFormula<Count, Charge> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut result = self.clone();
        result += other;
        result
    }
}

impl<Count: CountLike, Charge: ChargeLike> AddAssign for ChemicalFormula<Count, Charge> {
    fn add_assign(&mut self, other: Self) {
        for (other_count, other_tree) in other.mixtures {
            let mut found = false;
            for (self_count, self_tree) in &mut self.mixtures {
                if *self_tree == other_tree {
                    *self_count = (*self_count) + other_count;
                    found = true;
                    break;
                }
            }
            if !found {
                self.mixtures.push((other_count, other_tree));
            }
        }
    }
}

impl<Count: CountLike, Charge: ChargeLike> MolecularFormulaMetadata
    for ChemicalFormula<Count, Charge>
{
    type Count = Count;
}

impl<Count: CountLike, Charge: ChargeLike> MolecularFormula for ChemicalFormula<Count, Charge> {
    type Tree = ChemicalTree<Count, Charge, Empty>;

    fn mixtures(&self) -> impl Iterator<Item = (Self::Count, &ChemicalTree<Count, Charge, Empty>)> {
        self.mixtures.iter().map(|(count, tree)| (*count, tree))
    }
}

impl<Count: CountLike, Charge: ChargeLike> ChargedMolecularFormulaMetadata
    for ChemicalFormula<Count, Charge>
where
    Charge: TryFrom<Count>,
{
    type Charge = Charge;
}

impl<Count: CountLike, Charge: ChargeLike> ParsableFormula for ChemicalFormula<Count, Charge>
where
    Isotope: TryFrom<(elements_rs::Element, Count), Error = elements_rs::errors::Error>,
    Charge: TryFrom<Count>,
{
    type StartOutput = ();
    type Tree = ChemicalTree<Count, Charge, Empty>;

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
        assert!(!mixtures.is_empty(), "At least one mixture is required");
        Ok(Self { mixtures })
    }
}

impl<Count: CountLike, Charge: ChargeLike> Display for ChemicalFormula<Count, Charge> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for (i, (count, tree)) in self.mixtures.iter().enumerate() {
            if i > 0 {
                write!(f, ".")?;
            }
            if !count.is_one() {
                write!(f, "{count}")?;
            }
            write!(f, "{tree}")?;
        }
        Ok(())
    }
}
