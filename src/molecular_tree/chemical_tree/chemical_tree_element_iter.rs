//! Submodule providing an iterator over the elements present in a chemical
//! tree.

use alloc::boxed::Box;
use core::iter::Empty;

use elements_rs::{Element, Isotope};

use crate::{
    BracketNode, ChargeLike, ChargeNode, ChemicalTree, CountLike, RadicalNode, RepeatNode,
    SequenceNode, molecular_tree::MolecularTree,
};

#[allow(clippy::type_complexity)]
pub enum ChemicalTreeElementIter<
    'a,
    Count: CountLike + 'a,
    Charge: ChargeLike + 'a,
    Extension: Clone + 'a,
> {
    /// An atom (element)
    Element(<Element as MolecularTree<Count>>::ElementIter<'a>),
    /// An isotope (element with mass number)
    Isotope(<Isotope as MolecularTree<Count>>::ElementIter<'a>),
    /// A left-hand side radical.
    Radical(Box<<RadicalNode<Box<ChemicalTree<Count, Charge, Extension>>> as MolecularTree<Count>>::ElementIter<'a>>),
    /// An ion (element or molecule with charge)
    Charge(Box<<ChargeNode<Charge, Box<ChemicalTree<Count, Charge, Extension>>> as MolecularTree<Count>>::ElementIter<'a>>),
    /// Number of molecules
    Repeat(Box<<RepeatNode<Count, Box<ChemicalTree<Count, Charge, Extension>>> as MolecularTree<Count>>::ElementIter<'a>>),
    /// A sequence of molecular formulas
    Sequence(Box<<SequenceNode<ChemicalTree<Count, Charge, Extension>> as MolecularTree<Count>>::ElementIter<'a>>),
    /// A repeating unit wrapped in round brackets
    Unit(Box<<BracketNode<Box<ChemicalTree<Count, Charge, Extension>>> as MolecularTree<Count>>::ElementIter<'a>>),
    /// An extension node for arbitrary extensions
    Extension(Empty<Element>),
}

impl<'a, Count: CountLike + 'a, Charge: ChargeLike + 'a, Extension: Clone>
    From<&'a ChemicalTree<Count, Charge, Extension>>
    for ChemicalTreeElementIter<'a, Count, Charge, Extension>
{
    fn from(
        tree: &'a ChemicalTree<Count, Charge, Extension>,
    ) -> ChemicalTreeElementIter<'a, Count, Charge, Extension> {
        match tree {
            ChemicalTree::Element(e) => {
                ChemicalTreeElementIter::Element(<Element as MolecularTree<Count>>::elements(e))
            }
            ChemicalTree::Isotope(i) => {
                ChemicalTreeElementIter::Isotope(<Isotope as MolecularTree<Count>>::elements(i))
            }
            ChemicalTree::Radical(r) => ChemicalTreeElementIter::Radical(Box::new(r.elements())),
            ChemicalTree::Charge(c) => ChemicalTreeElementIter::Charge(Box::new(c.elements())),
            ChemicalTree::Repeat(r) => ChemicalTreeElementIter::Repeat(Box::new(r.elements())),
            ChemicalTree::Sequence(s) => ChemicalTreeElementIter::Sequence(Box::new(s.elements())),
            ChemicalTree::Unit(b) => ChemicalTreeElementIter::Unit(Box::new(b.elements())),
            ChemicalTree::Extension(_) => ChemicalTreeElementIter::Extension(core::iter::empty()),
        }
    }
}

impl<'a, Count: CountLike + 'a, Charge: ChargeLike + 'a, Extension: Clone + 'a> Iterator
    for ChemicalTreeElementIter<'a, Count, Charge, Extension>
{
    type Item = Element;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ChemicalTreeElementIter::Element(iter) | ChemicalTreeElementIter::Isotope(iter) => {
                iter.next()
            }
            ChemicalTreeElementIter::Radical(iter)
            | ChemicalTreeElementIter::Charge(iter)
            | ChemicalTreeElementIter::Unit(iter) => iter.next(),
            ChemicalTreeElementIter::Repeat(iter) => iter.next(),
            ChemicalTreeElementIter::Sequence(iter) => iter.next(),
            ChemicalTreeElementIter::Extension(iter) => iter.next(),
        }
    }
}

#[allow(clippy::type_complexity)]
pub enum ChemicalTreeNonHydrogenElementIter<
    'a,
    Count: CountLike + 'a,
    Charge: ChargeLike + 'a,
    Extension: Clone + 'a,
> {
    /// An atom (element)
    Element(<Element as MolecularTree<Count>>::NonHydrogenElementIter<'a>),
    /// An isotope (element with mass number)
    Isotope(<Isotope as MolecularTree<Count>>::NonHydrogenElementIter<'a>),
    /// A left-hand side radical.
    Radical(Box<<RadicalNode<Box<ChemicalTree<Count, Charge, Extension>>> as MolecularTree<Count>>::NonHydrogenElementIter<'a>>),
    /// An ion (element or molecule with charge)
    Charge(Box<<ChargeNode<Charge, Box<ChemicalTree<Count, Charge, Extension>>> as MolecularTree<Count>>::NonHydrogenElementIter<'a>>),
    /// Number of molecules
    Repeat(Box<<RepeatNode<Count, Box<ChemicalTree<Count, Charge, Extension>>> as MolecularTree<Count>>::NonHydrogenElementIter<'a>>),
    /// A sequence of molecular formulas
    Sequence(Box<<SequenceNode<ChemicalTree<Count, Charge, Extension>> as MolecularTree<Count>>::NonHydrogenElementIter<'a>>),
    /// A repeating unit wrapped in round brackets
    Unit(Box<<BracketNode<Box<ChemicalTree<Count, Charge, Extension>>> as MolecularTree<Count>>::NonHydrogenElementIter<'a>>),
    /// An extension node for arbitrary extensions
    Extension(Empty<Element>),
}

impl<'a, Count: CountLike + 'a, Charge: ChargeLike + 'a, Extension: Clone>
    From<&'a ChemicalTree<Count, Charge, Extension>>
    for ChemicalTreeNonHydrogenElementIter<'a, Count, Charge, Extension>
{
    fn from(
        tree: &'a ChemicalTree<Count, Charge, Extension>,
    ) -> ChemicalTreeNonHydrogenElementIter<'a, Count, Charge, Extension> {
        match tree {
            ChemicalTree::Element(e) => {
                ChemicalTreeNonHydrogenElementIter::Element(
                    <Element as MolecularTree<Count>>::non_hydrogens(e),
                )
            }
            ChemicalTree::Isotope(i) => {
                ChemicalTreeNonHydrogenElementIter::Isotope(
                    <Isotope as MolecularTree<Count>>::non_hydrogens(i),
                )
            }
            ChemicalTree::Radical(r) => {
                ChemicalTreeNonHydrogenElementIter::Radical(Box::new(r.non_hydrogens()))
            }
            ChemicalTree::Charge(c) => {
                ChemicalTreeNonHydrogenElementIter::Charge(Box::new(c.non_hydrogens()))
            }
            ChemicalTree::Repeat(r) => {
                ChemicalTreeNonHydrogenElementIter::Repeat(Box::new(r.non_hydrogens()))
            }
            ChemicalTree::Sequence(s) => {
                ChemicalTreeNonHydrogenElementIter::Sequence(Box::new(s.non_hydrogens()))
            }
            ChemicalTree::Unit(b) => {
                ChemicalTreeNonHydrogenElementIter::Unit(Box::new(b.non_hydrogens()))
            }
            ChemicalTree::Extension(_) => {
                ChemicalTreeNonHydrogenElementIter::Extension(core::iter::empty())
            }
        }
    }
}

impl<'a, Count: CountLike + 'a, Charge: ChargeLike + 'a, Extension: Clone + 'a> Iterator
    for ChemicalTreeNonHydrogenElementIter<'a, Count, Charge, Extension>
{
    type Item = Element;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ChemicalTreeNonHydrogenElementIter::Element(iter)
            | ChemicalTreeNonHydrogenElementIter::Isotope(iter) => iter.next(),
            ChemicalTreeNonHydrogenElementIter::Radical(iter)
            | ChemicalTreeNonHydrogenElementIter::Charge(iter)
            | ChemicalTreeNonHydrogenElementIter::Unit(iter) => iter.next(),
            ChemicalTreeNonHydrogenElementIter::Repeat(iter) => iter.next(),
            ChemicalTreeNonHydrogenElementIter::Sequence(iter) => iter.next(),
            ChemicalTreeNonHydrogenElementIter::Extension(iter) => iter.next(),
        }
    }
}
