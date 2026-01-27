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
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
///
/// use molecular_formulas::prelude::*;
///
/// let formula = ChemicalFormula::<u32, i32>::from_str("H2O").unwrap();
/// assert_eq!(formula.to_string(), "H₂O");
///
/// let glucose = ChemicalFormula::<u32, i32>::from_str("C6H12O6").unwrap();
/// assert_eq!(glucose.count_of_element(Element::C), Some(6u32));
/// ```
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
                if *self_tree == other_tree
                    && let Some(new_count) = self_count.checked_add(&other_count)
                {
                    *self_count = new_count;
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

#[cfg(test)]
mod tests {
    use alloc::string::ToString;
    use core::str::FromStr;

    use super::*;

    #[test]
    fn test_add_same_molecules() {
        let f1 = ChemicalFormula::<u32, i32>::from_str("H2O").unwrap();
        let f2 = ChemicalFormula::<u32, i32>::from_str("H2O").unwrap();
        let sum = f1 + f2;
        assert_eq!(sum.to_string(), "2H₂O");
    }

    #[test]
    fn test_add_different_molecules() {
        let f1 = ChemicalFormula::<u32, i32>::from_str("H2O").unwrap();
        let f2 = ChemicalFormula::<u32, i32>::from_str("NaCl").unwrap();
        let sum = f1 + f2;
        // The implementation appends new mixtures to the end
        assert_eq!(sum.to_string(), "H₂O.NaCl");
    }

    #[test]
    fn test_add_assign_mixture() {
        let mut f1 = ChemicalFormula::<u32, i32>::from_str("2H2O").unwrap();
        let f2 = ChemicalFormula::<u32, i32>::from_str("3H2O.NaCl").unwrap();

        f1 += f2;
        // 2H2O + (3H2O + NaCl) -> (2+3)H2O + NaCl -> 5H2O.NaCl
        assert_eq!(f1.to_string(), "5H₂O.NaCl");
    }

    #[test]
    fn test_add_complex() {
        let f1 = ChemicalFormula::<u32, i32>::from_str("C6H12O6").unwrap();
        let f2 = ChemicalFormula::<u32, i32>::from_str("6O2").unwrap();
        let sum = f1 + f2;
        assert_eq!(sum.to_string(), "C₆H₁₂O₆.6O₂");
    }

    #[test]
    fn test_charge_summation() {
        use crate::ChargedMolecularFormula;
        let f1 = ChemicalFormula::<u32, i32>::from_str("Na+").unwrap();
        assert!((f1.charge() - 1.0).abs() < f64::EPSILON);

        let f2 = ChemicalFormula::<u32, i32>::from_str("2Na+").unwrap();
        assert!(
            (f2.charge() - 2.0).abs() < f64::EPSILON,
            "Charge of 2Na+ should be 2.0, got {}",
            f2.charge()
        );

        let f3 = f1 + f2;
        assert_eq!(f3.to_string(), "3Na⁺");
        assert!(
            (f3.charge() - 3.0).abs() < f64::EPSILON,
            "Charge of 3Na+ should be 3.0, got {}",
            f3.charge()
        );
    }

    #[test]
    fn test_add_overflow_chains() {
        // Use u8 for count to easily trigger overflow
        let f1 = ChemicalFormula::<u8, i16>::from_str("250H2O").unwrap();
        let f2 = ChemicalFormula::<u8, i16>::from_str("10H2O").unwrap();

        // 250 + 10 = 260 (overflow u8 which is max 255)
        // Should result in 250H2O.10H2O
        let sum = f1 + f2;
        assert_eq!(sum.to_string(), "250H₂O.10H₂O");
    }
}
