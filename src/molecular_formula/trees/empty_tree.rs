//! Submodule providing an implementation of an empty tree for molecular
//! formulas.

use std::fmt::Display;

use elements_rs::{Element, Isotope};

use crate::{
    ChargeLike, Complex, CountLike, NoResidualsTree, ParseError, Tree,
    molecular_formula::trees::InstantiableTree,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// This tree implementation represents a molecular formula with no components.
pub struct EmptyTree<S, U> {
    /// Marker for the signed number type.
    _marker_s: std::marker::PhantomData<S>,
    /// Marker for the unsigned number type.
    _marker_u: std::marker::PhantomData<U>,
}

impl<S: ChargeLike + TryFrom<U>, U: CountLike> Tree for EmptyTree<S, U> {
    type Unsigned = U;
    type Signed = S;

    #[inline]
    fn iter_elements(&self) -> Box<dyn Iterator<Item = Element> + '_> {
        Box::new(std::iter::empty())
    }

    #[inline]
    fn iter_counted_elements(&self) -> Box<dyn Iterator<Item = Element> + '_> {
        Box::new(std::iter::empty())
    }

    #[inline]
    fn iter_isotopes(&self) -> Box<dyn Iterator<Item = Isotope> + '_> {
        Box::new(std::iter::empty())
    }

    #[inline]
    fn element_count(&self, _target: Element) -> Option<u64> {
        Some(0)
    }

    #[inline]
    fn isotope_count(&self, _target: Isotope) -> Option<u64> {
        Some(0)
    }

    #[inline]
    fn get_counted_element(&self, _index: u64) -> Option<Element> {
        None
    }

    #[inline]
    fn get_counted_element_or_size(&self, _index: u64) -> Result<Element, u64> {
        Err(0)
    }
}

impl<S: ChargeLike + TryFrom<U>, U: CountLike> NoResidualsTree for EmptyTree<S, U> {
    #[inline]
    fn total_charge(&self) -> f64 {
        0.0
    }

    #[inline]
    fn isotopologue_mass_with_charge(&self) -> f64 {
        0.0
    }

    #[inline]
    fn isotopologue_mass_without_charge(&self) -> f64 {
        0.0
    }

    #[inline]
    fn molar_mass(&self) -> f64 {
        0.0
    }
}

impl<S: ChargeLike + TryFrom<U>, U: CountLike> InstantiableTree for EmptyTree<S, U> {
    #[inline]
    fn charge(self, _charge: Self::Signed) -> Result<Self, ParseError<S, U>> {
        unreachable!("Cannot charge an empty tree")
    }

    #[inline]
    fn uncharge(self) -> Result<(Self, Self::Signed), Self> {
        unreachable!("Cannot uncharge an empty tree")
    }

    #[inline]
    fn complex(_complex: Complex) -> Self {
        unreachable!("Cannot create a complex from an empty tree")
    }

    #[inline]
    fn repeat(self, _times: Self::Unsigned) -> Result<Self, ParseError<S, U>> {
        unreachable!("Cannot repeat an empty tree")
    }

    #[inline]
    fn round(self) -> Self {
        unreachable!("Cannot create a unit from an empty tree")
    }

    #[inline]
    fn square(self) -> Self {
        unreachable!("Cannot create a unit from an empty tree")
    }

    #[inline]
    fn left_radical(self) -> Self {
        unreachable!("Cannot create a radical from an empty tree")
    }

    #[inline]
    fn right_radical(self) -> Self {
        unreachable!("Cannot create a radical from an empty tree")
    }

    #[inline]
    fn is_leaf(&self) -> bool {
        true
    }

    #[inline]
    fn residual() -> Result<Self, ParseError<S, U>> {
        Err(ParseError::ResidualNotSupportedInCurrentTree)
    }

    #[inline]
    fn element(_element: Element) -> Self {
        unreachable!("Cannot create an element from an empty tree")
    }

    #[inline]
    fn isotope(_isotope: Isotope) -> Self {
        unreachable!("Cannot create an isotope from an empty tree")
    }

    #[inline]
    fn from_iter<I: IntoIterator<Item = Self>>(
        _iter: I,
    ) -> Result<Self, ParseError<Self::Signed, Self::Unsigned>> {
        unreachable!("Cannot create an empty tree from an iterator")
    }

    fn into_sequence(self) -> Result<Vec<Self>, Self> {
        unreachable!("Cannot create a sequence from an empty tree")
    }
}

impl<S, U> Display for EmptyTree<S, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
