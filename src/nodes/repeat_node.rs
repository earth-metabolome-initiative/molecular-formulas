//! A repeat node which can be defined using either baseline digits ('0'-'9')
//! or subscript digits ('₀'-'₉').

use core::fmt::Display;

use crate::{ChargedMolecularTree, CountLike, MolecularTree, subscript_digits_ltr};

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RepeatNode<Count, T> {
    /// The number of repetitions.
    pub(crate) count: Count,
    /// The node being repeated.
    pub(crate) node: T,
}

impl<Count, T> AsRef<T> for RepeatNode<Count, T> {
    fn as_ref(&self) -> &T {
        &self.node
    }
}

impl<Count: CountLike, T: MolecularTree<Count>> MolecularTree<Count> for RepeatNode<Count, T> {
    type ElementIter<'a>
        = core::iter::FlatMap<
        core::iter::RepeatN<&'a T>,
        T::ElementIter<'a>,
        fn(&'a T) -> T::ElementIter<'a>,
    >
    where
        Self: 'a;

    type NonHydrogenElementIter<'a>
        = core::iter::FlatMap<
        core::iter::RepeatN<&'a T>,
        T::NonHydrogenElementIter<'a>,
        fn(&'a T) -> T::NonHydrogenElementIter<'a>,
    >
    where
        Self: 'a;

    #[inline]
    fn elements(&self) -> Self::ElementIter<'_> {
        let count: usize = self
            .count
            .try_into()
            .ok()
            .expect("Count too large for usize - do you have an extremely large repeat count?");
        core::iter::repeat_n(&self.node, count).flat_map(T::elements)
    }

    #[inline]
    fn non_hydrogens(&self) -> Self::NonHydrogenElementIter<'_> {
        let count: usize = self
            .count
            .try_into()
            .ok()
            .expect("Count too large for usize - do you have an extremely large repeat count?");
        core::iter::repeat_n(&self.node, count).flat_map(T::non_hydrogens)
    }

    #[inline]
    fn contains_elements(&self) -> bool {
        self.node.contains_elements()
    }

    #[inline]
    fn contains_non_hydrogens(&self) -> bool {
        self.node.contains_non_hydrogens()
    }

    #[inline]
    fn contains_isotopes(&self) -> bool {
        self.node.contains_isotopes()
    }

    #[inline]
    fn contains_element(&self, element: elements_rs::Element) -> bool {
        self.node.contains_element(element)
    }

    #[inline]
    fn contains_isotope(&self, isotope: elements_rs::Isotope) -> bool {
        self.node.contains_isotope(isotope)
    }

    #[inline]
    fn number_of_elements(&self) -> usize {
        let count: usize = self.count.try_into().ok().expect("Count too large for usize");
        count * self.node.number_of_elements()
    }

    #[inline]
    fn count_of_element<C>(&self, element: elements_rs::Element) -> Option<C>
    where
        C: From<Count>
            + num_traits::CheckedAdd
            + num_traits::CheckedMul
            + num_traits::ConstZero
            + num_traits::ConstOne,
    {
        let node_count = self.node.count_of_element::<C>(element)?;
        let count_as_c: C = C::from(self.count);
        node_count.checked_mul(&count_as_c)
    }

    #[inline]
    fn count_of_isotope<C>(&self, isotope: elements_rs::Isotope) -> Option<C>
    where
        C: From<Count>
            + num_traits::CheckedAdd
            + num_traits::CheckedMul
            + num_traits::ConstZero
            + num_traits::ConstOne,
    {
        let node_count = self.node.count_of_isotope::<C>(isotope)?;
        let count_as_c: C = C::from(self.count);
        node_count.checked_mul(&count_as_c)
    }

    #[inline]
    fn isotopologue_mass(&self) -> f64 {
        let count: f64 = self.count.into();
        self.node.isotopologue_mass() * count
    }

    #[inline]
    fn is_noble_gas_compound(&self) -> bool {
        self.node.is_noble_gas_compound()
    }

    fn check_hill_ordering(
        &self,
        predecessor: Option<elements_rs::Element>,
        has_carbon: bool,
    ) -> Result<Option<elements_rs::Element>, ()> {
        self.node.check_hill_ordering(predecessor, has_carbon)
    }
}

impl<Count: CountLike, T: Display> Display for RepeatNode<Count, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.node)?;
        for digit in subscript_digits_ltr(self.count) {
            write!(f, "{digit}")?;
        }
        Ok(())
    }
}

impl<Count: CountLike, T> RepeatNode<Count, T> {
    /// Create a new `RepeatNode` with the provided count and node.
    pub(crate) fn new(count: Count, node: T) -> Self {
        assert_ne!(count, Count::ZERO, "RepeatNode count cannot be zero");
        Self { count, node }
    }

    /// Get the count of the repeat node.
    pub fn count(&self) -> &Count {
        &self.count
    }

    /// Get the node being repeated.
    pub fn node(&self) -> &T {
        &self.node
    }
}

impl<Count: CountLike, Charge, T: ChargedMolecularTree<Count, Charge>>
    ChargedMolecularTree<Count, Charge> for RepeatNode<Count, T>
{
    fn charge(&self) -> f64 {
        let count: f64 = self.count.into();
        self.node.charge() * count
    }

    fn isotopologue_mass_with_charge(&self) -> f64 {
        let count: f64 = self.count.into();
        self.node.isotopologue_mass_with_charge() * count
    }

    fn molar_mass(&self) -> f64 {
        let count: f64 = self.count.into();
        self.node.molar_mass() * count
    }
}
