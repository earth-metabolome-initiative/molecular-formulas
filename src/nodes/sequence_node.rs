//! Submodule providing a struct and implementation of the `ExtensionTree` trait
//! for sequences of nodes, allowing for concatenation of different node types
//! in molecular formulas.

use alloc::vec::Vec;
use core::fmt::Display;

use super::{Node, Supports};
use crate::{ChargeLike, ChargedMolecularTree, CountLike, MolecularTree};

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A sequence node containing a vector of nodes of type `N`.
pub struct SequenceNode<N> {
    nodes: Vec<N>,
}

impl<N> SequenceNode<N> {
    /// Creates a new empty `SequenceNode`.
    pub(crate) fn empty() -> Self {
        Self { nodes: Vec::new() }
    }

    /// Pushes a new node into the sequence.
    pub(crate) fn push(&mut self, node: N) {
        self.nodes.push(node);
    }

    /// Returns whether the sequence node is empty.
    pub(crate) fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns the last node as mutable reference.
    pub(crate) fn pop(&mut self) -> Option<N> {
        self.nodes.pop()
    }

    /// Returns an iterator over the nodes in the sequence.
    pub(crate) fn iter(&self) -> core::slice::Iter<'_, N> {
        self.nodes.iter()
    }

    /// Returns an into iterator over the nodes in the sequence.
    pub(crate) fn into_iter(self) -> alloc::vec::IntoIter<N> {
        self.nodes.into_iter()
    }
}

impl<M, N> Supports<M> for SequenceNode<N>
where
    N: Supports<M> + Node,
    M: Node,
{
}

impl<N: Display> Display for SequenceNode<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for node in &self.nodes {
            write!(f, "{node}")?;
        }
        Ok(())
    }
}

impl<Count, T: MolecularTree<Count>> MolecularTree<Count> for SequenceNode<T> {
    type ElementIter<'a>
        = core::iter::FlatMap<
        core::slice::Iter<'a, T>,
        T::ElementIter<'a>,
        fn(&'a T) -> T::ElementIter<'a>,
    >
    where
        Self: 'a;

    type NonHydrogenElementIter<'a>
        = core::iter::FlatMap<
        core::slice::Iter<'a, T>,
        T::NonHydrogenElementIter<'a>,
        fn(&'a T) -> T::NonHydrogenElementIter<'a>,
    >
    where
        Self: 'a;

    fn elements(&self) -> Self::ElementIter<'_> {
        self.nodes.iter().flat_map(|node: &T| node.elements())
    }

    fn non_hydrogens(&self) -> Self::NonHydrogenElementIter<'_> {
        self.nodes.iter().flat_map(|node: &T| node.non_hydrogens())
    }

    fn contains_elements(&self) -> bool {
        self.nodes.iter().any(|node: &T| node.contains_elements())
    }

    fn contains_non_hydrogens(&self) -> bool {
        self.nodes.iter().any(|node: &T| node.contains_non_hydrogens())
    }

    fn contains_isotopes(&self) -> bool {
        self.nodes.iter().any(|node: &T| node.contains_isotopes())
    }

    fn contains_element(&self, element: elements_rs::Element) -> bool {
        self.nodes.iter().any(|node: &T| node.contains_element(element))
    }

    fn contains_isotope(&self, isotope: elements_rs::Isotope) -> bool {
        self.nodes.iter().any(|node: &T| node.contains_isotope(isotope))
    }

    fn number_of_elements(&self) -> usize {
        self.nodes.iter().map(|node: &T| node.number_of_elements()).sum()
    }

    fn count_of_element<C>(&self, element: elements_rs::Element) -> Option<C>
    where
        C: From<Count>
            + num_traits::CheckedAdd
            + num_traits::CheckedMul
            + num_traits::ConstZero
            + num_traits::ConstOne,
    {
        let mut total = C::ZERO;
        for node in &self.nodes {
            total = total.checked_add(&node.count_of_element::<C>(element)?)?;
        }
        Some(total)
    }

    fn count_of_isotope<C>(&self, isotope: elements_rs::Isotope) -> Option<C>
    where
        C: From<Count>
            + num_traits::CheckedAdd
            + num_traits::CheckedMul
            + num_traits::ConstZero
            + num_traits::ConstOne,
    {
        let mut total = C::ZERO;
        for node in &self.nodes {
            total = total.checked_add(&node.count_of_isotope::<C>(isotope)?)?;
        }
        Some(total)
    }

    fn isotopologue_mass(&self) -> f64 {
        self.nodes.iter().map(MolecularTree::isotopologue_mass).sum()
    }

    fn is_noble_gas_compound(&self) -> bool {
        self.nodes.iter().all(MolecularTree::is_noble_gas_compound)
    }

    fn isotopic_normalization(&self) -> Self {
        Self { nodes: self.nodes.iter().map(MolecularTree::isotopic_normalization).collect() }
    }

    fn check_hill_ordering(
        &self,
        mut predecessor: Option<elements_rs::Element>,
        has_carbon: bool,
    ) -> Result<Option<elements_rs::Element>, ()> {
        for node in &self.nodes {
            predecessor = node.check_hill_ordering(predecessor, has_carbon)?;
        }
        Ok(predecessor)
    }
}

impl<Count: CountLike, Charge: ChargeLike, T: ChargedMolecularTree<Count, Charge>>
    ChargedMolecularTree<Count, Charge> for SequenceNode<T>
{
    fn charge(&self) -> f64 {
        self.nodes.iter().map(ChargedMolecularTree::charge).sum()
    }

    fn isotopologue_mass_with_charge(&self) -> f64 {
        self.nodes.iter().map(ChargedMolecularTree::isotopologue_mass_with_charge).sum()
    }

    fn molar_mass(&self) -> f64 {
        self.nodes.iter().map(ChargedMolecularTree::molar_mass).sum()
    }
}
