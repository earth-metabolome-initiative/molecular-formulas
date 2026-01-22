//! Submodule defining the expression syntax trees for molecular formulas.

use std::fmt::Debug;

use elements_rs::{Element, ElementMask, Isotope};

use super::parser::{ChargeLike, Complex, CountLike, ParseError};

mod empty_tree;
mod generic_residual_tree;
mod generic_tree;
pub use empty_tree::EmptyTree;
pub use generic_residual_tree::GenericResidualTree;
pub use generic_tree::GenericTree;

/// A trait for molecular formula trees.
pub trait Tree: PartialEq + Debug + Clone {
    /// The unsigned number type used in the tree.
    type Unsigned: CountLike;
    /// The signed number type used in the tree.
    type Signed: ChargeLike + TryFrom<Self::Unsigned>;

    /// Returns an iterator over all elements in the molecular formula tree.
    fn iter_elements(&self) -> Box<dyn Iterator<Item = Element> + '_>;

    /// Returns the count of the specified element in the molecular formula
    /// tree.
    fn element_count(&self, target: Element) -> Option<u64>;

    /// Returns whether the tree contains any elements.
    fn contains_elements(&self) -> bool;

    /// Returns whether the tree contains a specific element.
    fn contains_element(&self, element: Element) -> bool;

    /// Returns an iterator over all isotopes in the molecular formula tree.
    fn iter_isotopes(&self) -> Box<dyn Iterator<Item = Isotope> + '_>;

    /// Returns the count of the specified isotope in the molecular formula
    /// tree.
    fn isotope_count(&self, target: Isotope) -> Option<u64>;

    /// Returns whether the tree contains any isotopes.
    fn contains_isotopes(&self) -> bool;

    /// Returns whether the tree contains a specific isotope.
    fn contains_isotope(&self, isotope: Isotope) -> bool;

    /// Returns an iterator over all elements in the molecular formula tree,
    /// repeating the repeating units according to their counts.
    fn iter_counted_elements(&self) -> Box<dyn Iterator<Item = Element> + '_>;

    /// Returns the element at the specified index in the tree, counting
    /// repeats.
    fn get_counted_element(&self, index: u64) -> Option<Element> {
        self.get_counted_element_or_size(index).ok()
    }

    /// Returns the element at the specified index, or the total size of the
    /// tree if the index is out of bounds.
    ///
    /// This method allows optimizing traversal by avoiding a separate
    /// `number_of_atoms` call when searching.
    ///
    /// # Errors
    ///
    /// If the index is out of bounds, returns the total number of atoms in the
    /// tree.
    fn get_counted_element_or_size(&self, index: u64) -> Result<Element, u64>;
}

/// A trait for molecular formula trees.
pub trait InstantiableTree: Tree + PartialEq + Sized + Debug {
    /// Returns whether the tree is composited or a leaf.
    fn is_leaf(&self) -> bool;

    /// Wraps the tree with a repeat.
    ///
    /// # Errors
    ///
    /// * If combining the repeats causes numeric overflow.
    fn repeat(
        self,
        times: Self::Unsigned,
    ) -> Result<Self, ParseError<Self::Signed, Self::Unsigned>>;

    /// Wraps the tree with a charge.
    ///
    /// # Errors
    ///
    /// * If combining the charges causes numeric overflow.
    fn charge(self, charge: Self::Signed)
    -> Result<Self, ParseError<Self::Signed, Self::Unsigned>>;

    /// Pops the charge from the tree, if the current
    /// node is charged.
    ///
    /// # Errors
    ///
    /// * If the current node is not a charged node.
    fn uncharge(self) -> Result<(Self, Self::Signed), Self>;

    /// Wraps a tree into a round bracket unit.
    #[must_use]
    fn round(self) -> Self;

    /// Wraps a tree into a square bracket unit.
    #[must_use]
    fn square(self) -> Self;

    /// Wraps a tree into a left-hand side radical.
    #[must_use]
    fn left_radical(self) -> Self;

    /// Wraps a tree into a right-hand side radical.
    #[must_use]
    fn right_radical(self) -> Self;

    /// Creates a residual tree node.
    ///
    /// # Errors
    ///
    /// * If the tree type does not support residuals.
    fn residual() -> Result<Self, ParseError<Self::Signed, Self::Unsigned>>;

    /// Creates an element tree node.
    fn element(element: Element) -> Self;

    /// Creates an isotope tree node.
    fn isotope(isotope: Isotope) -> Self;

    /// Creates a tree node from the provided complex.
    fn complex(complex: Complex) -> Self;

    /// Turns the provided iterable into a tree node.
    ///
    /// # Errors
    ///
    /// * If the iterable is empty.
    fn from_iter<I: IntoIterator<Item = Self>>(
        iter: I,
    ) -> Result<Self, ParseError<Self::Signed, Self::Unsigned>>;

    /// Returns a sequence of nodes in the tree, if
    /// the current node is a sequence, otherwise self.
    ///
    /// # Errors
    ///
    /// * If the current node is not a sequence.
    fn into_sequence(self) -> Result<Vec<Self>, Self>;
}

/// A trait for molecular formula trees that do not support residuals.
pub trait NoResidualsTree: Tree {
    /// Returns whether the molecular formula has repeated elements.
    fn has_repeated_elements(&self) -> bool {
        let mut element_mask = ElementMask::default();
        for element in self.iter_elements() {
            if !element_mask.insert(element) {
                return true;
            }
        }
        false
    }

    /// Returns whether the molecular formula is sorted according to Hill
    /// system.
    fn is_hill_sorted(&self) -> bool {
        if self.has_repeated_elements() {
            return false;
        }

        let mut previous = None;
        let mut found_carbon = false;
        for element in self.iter_elements() {
            if let Element::C = element {
                found_carbon = true;
                if previous.is_some() {
                    // Carbon must be first
                    return false;
                }
            }

            if matches!(element, Element::H)
                && found_carbon
                && !matches!(previous, Some(Element::C))
            {
                return false;
            }

            if found_carbon && matches!(previous, Some(Element::H)) {
                previous = Some(element);
                continue;
            }

            if let Some(prev) = previous {
                let prev_symbol: &str = prev.as_ref();
                let element_symbol: &str = element.as_ref();
                if element_symbol < prev_symbol {
                    return false;
                }
            }

            previous = Some(element);
        }
        true
    }

    /// Returns the total charge of the molecular formula tree.
    fn total_charge(&self) -> f64;

    /// Returns the isotopologue_mass of the molecular formula tree, including
    /// the charge.
    fn isotopologue_mass_with_charge(&self) -> f64;

    /// Returns the isotopologue_mass of the molecular formula tree, excluding
    /// the charge.
    fn isotopologue_mass_without_charge(&self) -> f64;

    /// Returns the molar mass of the molecular formula tree.
    fn molar_mass(&self) -> f64;
}

pub trait ResidualTree: Tree {
    /// Returns whether the molecular formula contains residuals.
    fn contains_residuals(&self) -> bool;
}
