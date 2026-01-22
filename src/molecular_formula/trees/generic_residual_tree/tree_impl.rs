//! Tree trait implementation for `GenericResidualTree`.

use crate::{ChargeLike, CountLike, Tree, molecular_formula::GenericResidualTree};

impl<S: ChargeLike + TryFrom<U>, U: CountLike> Tree for GenericResidualTree<S, U> {
    type Signed = S;
    type Unsigned = U;

    fn iter_elements(&self) -> Box<dyn Iterator<Item = elements_rs::Element> + '_> {
        match self {
            GenericResidualTree::Tree(tree) => tree.iter_elements(),
            GenericResidualTree::Residual => Box::new(std::iter::empty()),
        }
    }

    fn iter_counted_elements(&self) -> Box<dyn Iterator<Item = elements_rs::Element> + '_> {
        match self {
            GenericResidualTree::Tree(tree) => tree.iter_counted_elements(),
            GenericResidualTree::Residual => Box::new(std::iter::empty()),
        }
    }

    fn iter_isotopes(&self) -> Box<dyn Iterator<Item = elements_rs::Isotope> + '_> {
        match self {
            GenericResidualTree::Tree(tree) => tree.iter_isotopes(),
            GenericResidualTree::Residual => Box::new(std::iter::empty()),
        }
    }

    fn contains_elements(&self) -> bool {
        match self {
            GenericResidualTree::Tree(tree) => tree.contains_elements(),
            GenericResidualTree::Residual => false,
        }
    }

    fn contains_isotopes(&self) -> bool {
        match self {
            GenericResidualTree::Tree(tree) => tree.contains_isotopes(),
            GenericResidualTree::Residual => false,
        }
    }

    fn contains_element(&self, element: elements_rs::Element) -> bool {
        match self {
            GenericResidualTree::Tree(tree) => tree.contains_element(element),
            GenericResidualTree::Residual => false,
        }
    }

    fn element_count(&self, target: elements_rs::Element) -> Option<u64> {
        match self {
            GenericResidualTree::Tree(tree) => tree.element_count(target),
            GenericResidualTree::Residual => Some(0),
        }
    }

    fn isotope_count(&self, target: elements_rs::Isotope) -> Option<u64> {
        match self {
            GenericResidualTree::Tree(tree) => tree.isotope_count(target),
            GenericResidualTree::Residual => Some(0),
        }
    }

    fn contains_isotope(&self, isotope: elements_rs::Isotope) -> bool {
        match self {
            GenericResidualTree::Tree(tree) => tree.contains_isotope(isotope),
            GenericResidualTree::Residual => false,
        }
    }

    fn get_counted_element(&self, index: u64) -> Option<elements_rs::Element> {
        match self {
            GenericResidualTree::Tree(tree) => tree.get_counted_element(index),
            GenericResidualTree::Residual => None,
        }
    }

    fn get_counted_element_or_size(&self, index: u64) -> Result<elements_rs::Element, u64> {
        match self {
            GenericResidualTree::Tree(tree) => tree.get_counted_element_or_size(index),
            GenericResidualTree::Residual => Err(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use elements_rs::Element;

    use crate::{
        Tree,
        molecular_formula::{GenericResidualTree, GenericTree},
    };

    // Helper type alias for brevity using i32/u32
    type TestTree = GenericResidualTree<i32, u32>;
    type InnerTree = GenericTree<i32, u32, TestTree>;

    #[test]
    fn test_residual_variant() {
        let tree = TestTree::Residual;

        // iter_elements
        assert_eq!(tree.iter_elements().count(), 0);

        // iter_counted_elements
        assert_eq!(tree.iter_counted_elements().count(), 0);

        // iter_isotopes
        assert_eq!(tree.iter_isotopes().count(), 0);

        // element_count
        assert_eq!(tree.element_count(Element::C), Some(0));

        // isotope_count
        if let Some(&iso) = Element::H.isotopes().first() {
            assert_eq!(tree.isotope_count(iso), Some(0));
        }

        // get_counted_element
        assert_eq!(tree.get_counted_element(0), None);

        // get_counted_element_or_size
        assert_eq!(tree.get_counted_element_or_size(0), Err(0));
    }

    #[test]
    fn test_tree_variant() {
        // Construct a tree representing "C"
        // Note: GenericTree::Element usually takes a Box<Element> or Element.

        let inner = InnerTree::Element(Element::C);
        let tree = TestTree::Tree(inner);

        // iter_elements
        let elements: Vec<_> = tree.iter_elements().collect();
        assert_eq!(elements, vec![Element::C]);

        // iter_counted_elements
        let counted: Vec<_> = tree.iter_counted_elements().collect();
        assert_eq!(counted, vec![Element::C]);

        // iter_isotopes
        assert_eq!(tree.iter_isotopes().count(), 0);

        // element_count
        assert_eq!(tree.element_count(Element::C), Some(1));
        assert_eq!(tree.element_count(Element::H), Some(0));

        // isotope_count
        if let Some(&iso) = Element::H.isotopes().first() {
            assert_eq!(tree.isotope_count(iso), Some(0));
        }

        // get_counted_element
        assert_eq!(tree.get_counted_element(0), Some(Element::C));
        assert_eq!(tree.get_counted_element(1), None);

        // get_counted_element_or_size
        assert_eq!(tree.get_counted_element_or_size(0), Ok(Element::C));
        assert_eq!(tree.get_counted_element_or_size(1), Err(1));
    }
}
