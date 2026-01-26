//! General enumeration for chemical tree nodes.

use alloc::boxed::Box;
use core::fmt::Display;

use crate::{
    ChargeLike, ChargedMolecularTree, Complex, CountLike, MolecularTree, display_isotope,
    errors::{NumericError, ParserError},
    prelude::{BracketNode, ChargeNode, Element, Isotope, RadicalNode, RepeatNode, SequenceNode},
};

mod chemical_tree_element_iter;
use chemical_tree_element_iter::ChemicalTreeElementIter;

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enumeration of chemical tree nodes.
pub enum ChemicalTree<Count: CountLike, Charge: ChargeLike, Extension> {
    /// An atom (element)
    Element(Element),
    /// An isotope (element with mass number)
    Isotope(Isotope),
    /// A left-hand side radical.
    Radical(RadicalNode<Box<Self>>),
    /// An ion (element or molecule with charge)
    Charge(ChargeNode<Charge, Box<Self>>),
    /// Number of molecules
    Repeat(RepeatNode<Count, Box<Self>>),
    /// A sequence of molecular formulas
    Sequence(SequenceNode<Self>),
    /// A repeating unit wrapped in round brackets
    Unit(BracketNode<Box<Self>>),
    /// An extension node for arbitrary extensions
    Extension(Extension),
}

impl<Count: CountLike, Charge: ChargeLike, Extension> ChemicalTree<Count, Charge, Extension> {
    /// Consumes the chemical tree and returns a version decorated with a
    /// left-hand side radical.
    pub(crate) fn left_radical(self) -> Self {
        Self::Radical(RadicalNode::left(Box::new(self)))
    }

    /// Consumes the chemical tree and returns a version decorated with a
    /// right-hand side radical.
    pub(crate) fn right_radical(self) -> Self {
        Self::Radical(RadicalNode::right(Box::new(self)))
    }

    #[inline]
    /// Wraps the chemical tree into square brackets.
    pub(crate) fn square(self) -> Self {
        if self.is_leaf() { self } else { Self::Unit(BracketNode::square(Box::new(self))) }
    }

    #[inline]
    /// Wraps the chemical tree into round brackets.
    pub(crate) fn round(self) -> Self {
        if self.is_leaf() { self } else { Self::Unit(BracketNode::round(Box::new(self))) }
    }

    /// Consumes the chemical tree and returns a version decorated with an
    /// isotope specifier.
    pub(crate) fn isotope(self, isotope: Isotope) -> Self {
        self.push(Self::Isotope(isotope))
    }

    /// Consumes the chemical tree and returns a version decorated with a
    /// charge specifier.
    pub(crate) fn charge(self, mut charge: Charge) -> Result<Self, ParserError> {
        if let Self::Sequence(sequence) = &self
            && sequence.is_empty()
        {
            return Err(ParserError::EmptyMolecularTree);
        }

        Ok(if let Self::Charge(charge_node) = self {
            charge = charge_node.charge.checked_add(&charge).ok_or(
                if charge_node.charge > Charge::ZERO && charge > Charge::ZERO {
                    NumericError::PositiveOverflow
                } else {
                    NumericError::NegativeOverflow
                },
            )?;
            if charge.is_zero() {
                *charge_node.into_tree()
            } else {
                Self::Charge(ChargeNode::new(charge, charge_node.into_tree()))
            }
        } else {
            Self::Charge(ChargeNode::new(charge, Box::new(self)))
        })
    }

    /// Consumes the chemical tree and returns a version decorated with a
    /// repeat specifier.
    pub(crate) fn repeat(self, count: Count) -> Self {
        if let Self::Sequence(mut sequence) = self {
            assert!(!sequence.is_empty());
            let last = sequence.pop().unwrap().repeat(count);
            sequence.push(last);
            Self::Sequence(sequence)
        } else {
            Self::Repeat(RepeatNode::new(count, Box::new(self)))
        }
    }

    /// Consumes the chemical tree and returns a version decorated with an
    /// extension specifier.
    pub(crate) fn extension(self, extension: Extension) -> Self {
        self.push(Self::Extension(extension))
    }

    /// Returns whether the chemical tree contains an extension node.
    pub(crate) fn contains_extension(&self) -> bool {
        match self {
            Self::Element(_) => false,
            Self::Isotope(_) => false,
            Self::Radical(r) => r.as_ref().contains_extension(),
            Self::Charge(c) => c.as_ref().contains_extension(),
            Self::Repeat(r) => r.as_ref().contains_extension(),
            Self::Sequence(s) => s.iter().any(|node| node.contains_extension()),
            Self::Unit(b) => b.as_ref().contains_extension(),
            Self::Extension(_) => true,
        }
    }

    /// Consumes the chemical tree and returns a version decorated with a
    /// complex specifier.
    pub(crate) fn complex(self, complex: Complex) -> Self {
        match complex {
            Complex::Benzyl => {
                let mut sequence: SequenceNode<Self> = SequenceNode::empty();
                sequence.push(Self::Repeat(RepeatNode::new(
                    Count::SEVEN,
                    Box::new(Self::Element(Element::C)),
                )));
                sequence.push(Self::Repeat(RepeatNode::new(
                    Count::SEVEN,
                    Box::new(Self::Element(Element::H)),
                )));
                self.push(Self::Sequence(sequence).round())
            }
            Complex::Butyl => {
                let mut sequence: SequenceNode<Self> = SequenceNode::empty();
                sequence.push(Self::Repeat(RepeatNode::new(
                    Count::FOUR,
                    Box::new(Self::Element(Element::C)),
                )));
                sequence.push(Self::Repeat(RepeatNode::new(
                    Count::NINE,
                    Box::new(Self::Element(Element::H)),
                )));
                self.push(Self::Sequence(sequence).round())
            }
            Complex::Phenyl => {
                let mut sequence: SequenceNode<Self> = SequenceNode::empty();
                sequence.push(Self::Repeat(RepeatNode::new(
                    Count::SIX,
                    Box::new(Self::Element(Element::C)),
                )));
                sequence.push(Self::Repeat(RepeatNode::new(
                    Count::FIVE,
                    Box::new(Self::Element(Element::H)),
                )));
                self.push(Self::Sequence(sequence).round())
            }
            Complex::Cyclohexyl => {
                let mut sequence: SequenceNode<Self> = SequenceNode::empty();
                sequence.push(Self::Repeat(RepeatNode::new(
                    Count::SIX,
                    Box::new(Self::Element(Element::C)),
                )));
                sequence.push(Self::Repeat(RepeatNode::new(
                    Count::ELEVEN,
                    Box::new(Self::Element(Element::H)),
                )));
                self.push(Self::Sequence(sequence).round())
            }
            Complex::Ethyl => {
                let mut sequence: SequenceNode<Self> = SequenceNode::empty();
                sequence.push(Self::Repeat(RepeatNode::new(
                    Count::TWO,
                    Box::new(Self::Element(Element::C)),
                )));
                sequence.push(Self::Repeat(RepeatNode::new(
                    Count::FIVE,
                    Box::new(Self::Element(Element::H)),
                )));
                self.push(Self::Sequence(sequence).round())
            }
            Complex::Methyl => {
                let mut sequence: SequenceNode<Self> = SequenceNode::empty();
                sequence.push(Self::Repeat(RepeatNode::new(
                    Count::ONE,
                    Box::new(Self::Element(Element::C)),
                )));
                sequence.push(Self::Repeat(RepeatNode::new(
                    Count::THREE,
                    Box::new(Self::Element(Element::H)),
                )));
                self.push(Self::Sequence(sequence).round())
            }
            Complex::Cyclopentadienyl => {
                let mut sequence: SequenceNode<Self> = SequenceNode::empty();
                sequence.push(Self::Repeat(RepeatNode::new(
                    Count::FIVE,
                    Box::new(Self::Element(Element::C)),
                )));
                sequence.push(Self::Repeat(RepeatNode::new(
                    Count::FIVE,
                    Box::new(Self::Element(Element::H)),
                )));
                Self::Charge(ChargeNode::new(
                    -Charge::ONE,
                    Box::new(self.push(Self::Sequence(sequence).round())),
                ))
            }
        }
    }

    /// Returns whether the tree is a leaf node (i.e., an element or isotope).
    pub(crate) fn is_leaf(&self) -> bool {
        matches!(self, Self::Element(_) | Self::Isotope(_))
    }

    /// Pushes a new node onto a sequence, converting the tree into a sequence
    /// if necessary.
    pub(crate) fn push(mut self, node: Self) -> Self {
        if let ChemicalTree::Sequence(ref mut sequence) = self {
            if sequence.is_empty() {
                node
            } else {
                sequence.push(node);
                self
            }
        } else {
            let mut sequence = SequenceNode::empty();
            sequence.push(self);
            sequence.push(node);
            ChemicalTree::Sequence(sequence)
        }
    }
}

impl<Count: CountLike, Charge: ChargeLike, Extension: MolecularTree<Count>> MolecularTree<Count>
    for ChemicalTree<Count, Charge, Extension>
{
    type ElementIter<'a>
        = ChemicalTreeElementIter<'a, Count, Charge, Extension>
    where
        Self: 'a;

    fn elements(&self) -> Self::ElementIter<'_> {
        self.into()
    }

    fn contains_elements(&self) -> bool {
        match self {
            Self::Element(e) => <Element as MolecularTree<Count>>::contains_elements(e),
            Self::Isotope(i) => <Isotope as MolecularTree<Count>>::contains_elements(i),
            Self::Radical(r) => r.contains_elements(),
            Self::Charge(c) => c.contains_elements(),
            Self::Repeat(r) => r.contains_elements(),
            Self::Sequence(s) => s.contains_elements(),
            Self::Unit(b) => b.contains_elements(),
            Self::Extension(e) => e.contains_elements(),
        }
    }

    fn contains_isotopes(&self) -> bool {
        match self {
            Self::Element(e) => <Element as MolecularTree<Count>>::contains_isotopes(e),
            Self::Isotope(i) => <Isotope as MolecularTree<Count>>::contains_isotopes(i),
            Self::Radical(r) => r.contains_isotopes(),
            Self::Charge(c) => c.contains_isotopes(),
            Self::Repeat(r) => r.contains_isotopes(),
            Self::Sequence(s) => s.contains_isotopes(),
            Self::Unit(b) => b.contains_isotopes(),
            Self::Extension(e) => e.contains_isotopes(),
        }
    }

    fn contains_element(&self, element: Element) -> bool {
        match self {
            Self::Element(e) => <Element as MolecularTree<Count>>::contains_element(e, element),
            Self::Isotope(i) => <Isotope as MolecularTree<Count>>::contains_element(i, element),
            Self::Radical(r) => r.contains_element(element),
            Self::Charge(c) => c.contains_element(element),
            Self::Repeat(r) => r.contains_element(element),
            Self::Sequence(s) => s.contains_element(element),
            Self::Unit(b) => b.contains_element(element),
            Self::Extension(e) => e.contains_element(element),
        }
    }

    fn contains_isotope(&self, isotope: elements_rs::Isotope) -> bool {
        match self {
            Self::Element(e) => <Element as MolecularTree<Count>>::contains_isotope(e, isotope),
            Self::Isotope(i) => <Isotope as MolecularTree<Count>>::contains_isotope(i, isotope),
            Self::Radical(r) => r.contains_isotope(isotope),
            Self::Charge(c) => c.contains_isotope(isotope),
            Self::Repeat(r) => r.contains_isotope(isotope),
            Self::Sequence(s) => s.contains_isotope(isotope),
            Self::Unit(b) => b.contains_isotope(isotope),
            Self::Extension(e) => e.contains_isotope(isotope),
        }
    }

    #[inline]
    fn count_of_element<C>(&self, element: Element) -> Option<C>
    where
        C: From<Count>
            + num_traits::CheckedAdd
            + num_traits::CheckedMul
            + num_traits::ConstZero
            + num_traits::ConstOne,
    {
        match self {
            Self::Element(e) => {
                <Element as MolecularTree<Count>>::count_of_element::<C>(e, element)
            }
            Self::Isotope(i) => {
                <Isotope as MolecularTree<Count>>::count_of_element::<C>(i, element)
            }
            Self::Radical(r) => r.count_of_element::<C>(element),
            Self::Charge(c) => c.count_of_element::<C>(element),
            Self::Repeat(r) => r.count_of_element::<C>(element),
            Self::Sequence(s) => s.count_of_element::<C>(element),
            Self::Unit(b) => b.count_of_element::<C>(element),
            Self::Extension(e) => e.count_of_element::<C>(element),
        }
    }

    #[inline]
    fn count_of_isotope<C>(&self, isotope: Isotope) -> Option<C>
    where
        C: From<Count>
            + num_traits::CheckedAdd
            + num_traits::CheckedMul
            + num_traits::ConstZero
            + num_traits::ConstOne,
    {
        match self {
            Self::Element(e) => {
                <Element as MolecularTree<Count>>::count_of_isotope::<C>(e, isotope)
            }
            Self::Isotope(i) => {
                <Isotope as MolecularTree<Count>>::count_of_isotope::<C>(i, isotope)
            }
            Self::Radical(r) => r.count_of_isotope::<C>(isotope),
            Self::Charge(c) => c.count_of_isotope::<C>(isotope),
            Self::Repeat(r) => r.count_of_isotope::<C>(isotope),
            Self::Sequence(s) => s.count_of_isotope::<C>(isotope),
            Self::Unit(b) => b.count_of_isotope::<C>(isotope),
            Self::Extension(e) => e.count_of_isotope::<C>(isotope),
        }
    }

    fn isotopologue_mass(&self) -> f64 {
        match self {
            Self::Element(e) => <Element as MolecularTree<Count>>::isotopologue_mass(e),
            Self::Isotope(i) => <Isotope as MolecularTree<Count>>::isotopologue_mass(i),
            Self::Radical(r) => r.isotopologue_mass(),
            Self::Charge(c) => c.isotopologue_mass(),
            Self::Repeat(r) => r.isotopologue_mass(),
            Self::Sequence(s) => s.isotopologue_mass(),
            Self::Unit(b) => b.isotopologue_mass(),
            Self::Extension(e) => e.isotopologue_mass(),
        }
    }

    fn is_noble_gas_compound(&self) -> bool {
        match self {
            Self::Element(e) => <Element as MolecularTree<Count>>::is_noble_gas_compound(e),
            Self::Isotope(i) => <Isotope as MolecularTree<Count>>::is_noble_gas_compound(i),
            Self::Radical(r) => r.is_noble_gas_compound(),
            Self::Charge(c) => c.is_noble_gas_compound(),
            Self::Repeat(r) => r.is_noble_gas_compound(),
            Self::Sequence(s) => s.is_noble_gas_compound(),
            Self::Unit(b) => b.is_noble_gas_compound(),
            Self::Extension(e) => e.is_noble_gas_compound(),
        }
    }
}

impl<Count: CountLike, Charge: ChargeLike, Extension: Display> Display
    for ChemicalTree<Count, Charge, Extension>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Element(e) => write!(f, "{e}"),
            Self::Isotope(i) => display_isotope(i, f),
            Self::Radical(r) => write!(f, "{r}"),
            Self::Charge(c) => write!(f, "{c}"),
            Self::Repeat(r) => write!(f, "{r}"),
            Self::Sequence(s) => write!(f, "{s}"),
            Self::Unit(b) => write!(f, "{b}"),
            Self::Extension(e) => write!(f, "{e}"),
        }
    }
}

impl<Count: CountLike, Charge: ChargeLike, Extension: ChargedMolecularTree<Count, Charge>>
    ChargedMolecularTree<Count, Charge> for ChemicalTree<Count, Charge, Extension>
{
    fn charge(&self) -> f64 {
        match self {
            Self::Element(e) => <Element as ChargedMolecularTree<Count, Charge>>::charge(e),
            Self::Isotope(i) => <Isotope as ChargedMolecularTree<Count, Charge>>::charge(i),
            Self::Radical(r) => r.charge(),
            Self::Charge(c) => c.charge(),
            Self::Repeat(r) => r.charge(),
            Self::Sequence(s) => s.charge(),
            Self::Unit(b) => b.charge(),
            Self::Extension(e) => e.charge(),
        }
    }

    fn isotopologue_mass_with_charge(&self) -> f64 {
        match self {
            Self::Element(e) => {
                <Element as ChargedMolecularTree<Count, Charge>>::isotopologue_mass_with_charge(e)
            }
            Self::Isotope(i) => {
                <Isotope as ChargedMolecularTree<Count, Charge>>::isotopologue_mass_with_charge(i)
            }
            Self::Radical(r) => r.isotopologue_mass_with_charge(),
            Self::Charge(c) => c.isotopologue_mass_with_charge(),
            Self::Repeat(r) => r.isotopologue_mass_with_charge(),
            Self::Sequence(s) => s.isotopologue_mass_with_charge(),
            Self::Unit(b) => b.isotopologue_mass_with_charge(),
            Self::Extension(e) => e.isotopologue_mass_with_charge(),
        }
    }

    fn molar_mass(&self) -> f64 {
        match self {
            Self::Element(e) => <Element as ChargedMolecularTree<Count, Charge>>::molar_mass(e),
            Self::Isotope(i) => <Isotope as ChargedMolecularTree<Count, Charge>>::molar_mass(i),
            Self::Radical(r) => r.molar_mass(),
            Self::Charge(c) => c.molar_mass(),
            Self::Repeat(r) => r.molar_mass(),
            Self::Sequence(s) => s.molar_mass(),
            Self::Unit(b) => b.molar_mass(),
            Self::Extension(e) => e.molar_mass(),
        }
    }
}
