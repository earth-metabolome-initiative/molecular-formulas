//! Submodule providing an iterator over the elements present in a chemical
//! tree.

use alloc::boxed::Box;

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
    Extension: MolecularTree<Count> + 'a,
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
    Extension(<Extension as MolecularTree<Count>>::ElementIter<'a>),
}

impl<'a, Count: CountLike + 'a, Charge: ChargeLike + 'a, Extension: MolecularTree<Count> + 'a>
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
            ChemicalTree::Extension(extension) => {
                ChemicalTreeElementIter::Extension(extension.elements())
            }
        }
    }
}

impl<'a, Count: CountLike + 'a, Charge: ChargeLike + 'a, Extension: MolecularTree<Count> + 'a>
    Iterator for ChemicalTreeElementIter<'a, Count, Charge, Extension>
{
    type Item = Element;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ChemicalTreeElementIter::Element(iter) | ChemicalTreeElementIter::Isotope(iter) => iter.next(),
            ChemicalTreeElementIter::Radical(iter) | ChemicalTreeElementIter::Charge(iter) | ChemicalTreeElementIter::Repeat(iter) | ChemicalTreeElementIter::Unit(iter) => iter.next(),
            ChemicalTreeElementIter::Sequence(iter) => iter.next(),
            ChemicalTreeElementIter::Extension(iter) => iter.next(),
        }
    }
}
