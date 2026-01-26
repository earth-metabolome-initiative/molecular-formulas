//! Submodule defining a parsable entity.

mod from_str_impls;
mod molecule_parser;
mod parsable_formula;
mod parsable_molecular_tree;
mod tokens;

use core::fmt::Debug;

pub(crate) use parsable_formula::ParsableFormula;
pub use tokens::*;

use crate::parsable::{
    molecule_parser::MoleculeParser, parsable_molecular_tree::ParsableMolecularTree,
};

/// Trait for tokens used in parsing molecular formulas.
pub(crate) trait TokenLike: Copy + Eq + Sized + Debug {
    /// The count type used by this token.
    type Count: CountLike;

    /// Returns whether this token is a mixture separator.
    fn is_mixture_separator(&self) -> bool;

    /// Returns the mixture separator token.
    fn mixture_separator() -> Self;

    /// Returns the count associated with this token if it represents a count.
    fn as_count(&self) -> Option<Self::Count>;

    /// Returns the element associated with this token if it represents an
    /// element.
    fn as_element(&self) -> Option<elements_rs::Element>;
}

#[cfg(test)]
mod tests {
    use elements_rs::Element;

    use super::*;
    use crate::nodes::Empty;

    #[test]
    fn test_inchi_token_implementation() {
        // Test Mixture Separator
        let dot = InchiToken::<u32>::mixture_separator();
        assert!(dot.is_mixture_separator());
        assert!(matches!(dot, InchiToken::Dot));
        assert_eq!(dot.as_count(), None);
        assert_eq!(dot.as_element(), None);

        // Test Element
        let elem = InchiToken::<u32>::Element(Element::C);
        assert!(!elem.is_mixture_separator());
        assert_eq!(elem.as_count(), None);
        assert_eq!(elem.as_element(), Some(Element::C));

        // Test Count
        let count = InchiToken::<u32>::Count(42);
        assert!(!count.is_mixture_separator());
        assert_eq!(count.as_count(), Some(42));
        assert_eq!(count.as_element(), None);
    }

    #[test]
    fn test_token_implementation() {
        // Test Mixture Separator
        let dot = Token::<u32, i32, Empty>::mixture_separator();
        assert!(dot.is_mixture_separator());
        assert_eq!(dot.as_count(), None);
        assert_eq!(dot.as_element(), None);

        // Test Element
        let elem = Token::<u32, i32, Empty>::from(Element::O);
        assert!(!elem.is_mixture_separator());
        assert_eq!(elem.as_count(), None);
        assert_eq!(elem.as_element(), Some(Element::O));

        // Test Count (wrapped InchiToken)
        let count_val = 15;
        let count_token = Token::<u32, i32, Empty>::Inchi(InchiToken::Count(count_val));
        assert!(!count_token.is_mixture_separator());
        assert_eq!(count_token.as_count(), Some(count_val));
        assert_eq!(count_token.as_element(), None);

        // Test other variants (should return None/false)
        let charge = Token::<u32, i32, Empty>::Charge(1);
        assert!(!charge.is_mixture_separator());
        assert_eq!(charge.as_count(), None);
        assert_eq!(charge.as_element(), None);

        // Radical
        let radical = Token::<u32, i32, Empty>::Radical;
        assert!(!radical.is_mixture_separator());
        assert_eq!(radical.as_count(), None);
        assert_eq!(radical.as_element(), None);

        // Isotope
        if let Ok(iso) = elements_rs::Isotope::try_from((Element::C, 13u16)) {
            let isotope = Token::<u32, i32, Empty>::Isotope(iso);
            assert!(!isotope.is_mixture_separator());
            assert_eq!(isotope.as_count(), None);
            assert_eq!(isotope.as_element(), None);
        }
    }
}
