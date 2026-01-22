//! Submodule iplementing `Tree` for `GenericTree`

use elements_rs::{Element, ElementVariant, Isotope};

use crate::{ChargeLike, CountLike, Tree, molecular_formula::GenericTree};

impl<S: ChargeLike + TryFrom<U>, U: CountLike, E: Tree<Unsigned = U, Signed = S>> Tree
    for GenericTree<S, U, E>
{
    type Unsigned = U;
    type Signed = S;

    fn iter_elements(&self) -> Box<dyn Iterator<Item = Element> + '_> {
        match self {
            Self::Element(element) => Box::new(std::iter::once(*element)),
            Self::Isotope(isotope) => Box::new(std::iter::once(isotope.element())),
            Self::Sequence(formulas) => Box::new(formulas.iter().flat_map(|f| f.iter_elements())),
            Self::Repeat(inner, _)
            | Self::Charge(inner, _)
            | Self::Unit(inner, _)
            | Self::Radical(inner, _) => inner.iter_elements(),
            Self::Extension(ext) => ext.iter_elements(),
        }
    }

    fn iter_counted_elements(&self) -> Box<dyn Iterator<Item = Element> + '_> {
        match self {
            Self::Element(element) => Box::new(std::iter::once(*element)),
            Self::Isotope(isotope) => Box::new(std::iter::once(isotope.element())),
            Self::Sequence(formulas) => {
                Box::new(formulas.iter().flat_map(|f| f.iter_counted_elements()))
            }
            Self::Repeat(inner, count) => {
                let n: u64 = (*count).into();
                Box::new((0..n).flat_map(move |_| inner.iter_counted_elements()))
            }
            Self::Charge(inner, _) | Self::Unit(inner, _) | Self::Radical(inner, _) => {
                inner.iter_counted_elements()
            }
            Self::Extension(ext) => ext.iter_counted_elements(),
        }
    }

    fn iter_isotopes(&self) -> Box<dyn Iterator<Item = Isotope> + '_> {
        match self {
            Self::Element(_) => Box::new(std::iter::empty()),
            Self::Isotope(isotope) => Box::new(std::iter::once(*isotope)),
            Self::Sequence(formulas) => Box::new(formulas.iter().flat_map(|f| f.iter_isotopes())),
            Self::Repeat(inner, _)
            | Self::Charge(inner, _)
            | Self::Unit(inner, _)
            | Self::Radical(inner, _) => inner.iter_isotopes(),
            Self::Extension(ext) => ext.iter_isotopes(),
        }
    }
}
