//! Submodule providing a struct and implementation of the `ExtensionTree` trait
//! for molecular formulas that can contain charges.

use crate::{ChargeLike, ChargedMolecularTree, CountLike, MolecularTree, display_charge};

const ELECTRON_MASS: f64 = 0.000548579909065;

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Extension tree for molecular formulas that can contain charges.
pub struct ChargeNode<C, T> {
    /// The charge amount.
    pub(crate) charge: C,
    /// The underlying tree.
    tree: T,
}

impl<C, T> AsRef<T> for ChargeNode<C, T> {
    fn as_ref(&self) -> &T {
        &self.tree
    }
}

impl<C: ChargeLike, T> ChargeNode<C, T> {
    /// Creates a new `ChargeNode` with the given charge and underlying tree.
    pub fn new(charge: C, tree: T) -> Self {
        Self { charge, tree }
    }

    /// Converts the node into the underlying tree, consuming the node.
    pub fn into_tree(self) -> T {
        self.tree
    }
}

impl<C: ChargeLike, T: core::fmt::Display> core::fmt::Display for ChargeNode<C, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.tree)?;
        display_charge(self.charge, f)
    }
}

impl<Count: CountLike, Charge: ChargeLike, T: MolecularTree<Count>> MolecularTree<Count>
    for ChargeNode<Charge, T>
{
    type ElementIter<'a>
        = T::ElementIter<'a>
    where
        Self: 'a;

    type NonHydrogenElementIter<'a>
        = T::NonHydrogenElementIter<'a>
    where
        Self: 'a;

    #[inline]
    fn elements(&self) -> Self::ElementIter<'_> {
        self.tree.elements()
    }

    #[inline]
    fn non_hydrogens(&self) -> Self::NonHydrogenElementIter<'_> {
        self.tree.non_hydrogens()
    }

    #[inline]
    fn contains_elements(&self) -> bool {
        self.tree.contains_elements()
    }

    #[inline]
    fn contains_non_hydrogens(&self) -> bool {
        self.tree.contains_non_hydrogens()
    }

    #[inline]
    fn contains_isotopes(&self) -> bool {
        self.tree.contains_isotopes()
    }

    #[inline]
    fn contains_element(&self, element: elements_rs::Element) -> bool {
        self.tree.contains_element(element)
    }

    #[inline]
    fn contains_isotope(&self, isotope: elements_rs::Isotope) -> bool {
        self.tree.contains_isotope(isotope)
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
        self.tree.count_of_element::<C>(element)
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
        self.tree.count_of_isotope::<C>(isotope)
    }

    fn isotopologue_mass(&self) -> f64 {
        self.tree.isotopologue_mass()
    }

    fn is_noble_gas_compound(&self) -> bool {
        self.tree.is_noble_gas_compound()
    }

    fn check_hill_ordering(
        &self,
        predecessor: Option<elements_rs::Element>,
        has_carbon: bool,
    ) -> Result<Option<elements_rs::Element>, ()> {
        self.tree.check_hill_ordering(predecessor, has_carbon)
    }
}

impl<Count: CountLike, Charge: ChargeLike, T: ChargedMolecularTree<Count, Charge>>
    ChargedMolecularTree<Count, Charge> for ChargeNode<Charge, T>
{
    fn charge(&self) -> f64 {
        self.charge.into()
    }

    fn isotopologue_mass_with_charge(&self) -> f64 {
        let charge: f64 = self.charge.into();
        self.tree.isotopologue_mass_with_charge() - charge * ELECTRON_MASS
    }

    fn molar_mass(&self) -> f64 {
        let charge: f64 = self.charge.into();
        self.tree.molar_mass() - charge * ELECTRON_MASS
    }
}
