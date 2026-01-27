//! Submodule defining the expression syntax trees for molecular formulas
//! as found in InChI strings. This is the strictest format supported by this
//! crate.

use alloc::vec::Vec;
use core::fmt::Display;

use crate::{
    CountLike, InChITree, MolecularFormula, MolecularFormulaMetadata, ParsableFormula,
    prelude::SequenceNode,
};

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord, Hash)]
/// A chemical formula representing molecular formulas in InChI format.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
///
/// use molecular_formulas::prelude::*;
///
/// // InChI formulas must usually be Hill sorted (C, H, then alphabetical)
/// let formula = InChIFormula::<u32>::from_str("C2H6O").unwrap();
/// assert_eq!(formula.to_string(), "C2H6O");
/// ```
pub struct InChIFormula<Count: CountLike = u16> {
    mixtures: Vec<(Count, SequenceNode<InChITree<Count>>)>,
}

impl<Count: CountLike> InChIFormula<Count> {
    /// Iterates on the sub-formulas in the InChI formula, repeating them
    /// according to their counts.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use molecular_formulas::prelude::*;
    ///
    /// let formula = InChIFormula::<u32>::from_str("2C2H6O.ClNa").unwrap();
    /// let subformulas: Vec<_> = formula.subformulas().collect();
    /// assert_eq!(subformulas.len(), 3);
    /// assert_eq!(subformulas[0].to_string(), "C2H6O");
    /// assert_eq!(subformulas[1].to_string(), "C2H6O");
    /// assert_eq!(subformulas[2].to_string(), "ClNa");
    /// ```
    pub fn subformulas(&self) -> impl Iterator<Item = Self> {
        self.mixtures().cloned().map(Into::into)
    }
}

impl<Count: CountLike> From<SequenceNode<InChITree<Count>>> for InChIFormula<Count> {
    fn from(tree: SequenceNode<InChITree<Count>>) -> Self {
        Self { mixtures: alloc::vec![(Count::one(), tree)] }
    }
}

impl<Count: CountLike> MolecularFormulaMetadata for InChIFormula<Count> {
    type Count = Count;
}

impl<Count: CountLike> MolecularFormula for InChIFormula<Count> {
    type Tree = SequenceNode<InChITree<Count>>;

    fn counted_mixtures(&self) -> impl Iterator<Item = (Self::Count, &Self::Tree)> {
        self.mixtures.iter().map(|(count, tree)| (*count, tree))
    }
}

impl<Count: CountLike> ParsableFormula for InChIFormula<Count> {
    type StartOutput = ();
    type Tree = SequenceNode<InChITree<Count>>;

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
        let inchi = InChIFormula { mixtures };

        if !inchi.is_hill_sorted() {
            return Err(crate::errors::ParserError::NotHillOrdered);
        }

        Ok(inchi)
    }
}

impl<Count: CountLike> Display for InChIFormula<Count> {
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
