//! Blanket implementation for `Tree` types wrapped in a Box.

use crate::{
    Complex, NoResidualsTree, ParseError, Tree,
    molecular_formula::trees::{InstantiableTree, ResidualTree},
};

impl<T> Tree for Box<T>
where
    T: Tree,
{
    type Unsigned = T::Unsigned;
    type Signed = T::Signed;

    fn iter_counted_elements(&self) -> Box<dyn Iterator<Item = elements_rs::Element> + '_> {
        (**self).iter_counted_elements()
    }

    fn iter_elements(&self) -> Box<dyn Iterator<Item = elements_rs::Element> + '_> {
        (**self).iter_elements()
    }

    fn iter_isotopes(&self) -> Box<dyn Iterator<Item = elements_rs::Isotope> + '_> {
        (**self).iter_isotopes()
    }

    fn element_count(&self, target: elements_rs::Element) -> Option<u64> {
        (**self).element_count(target)
    }

    fn isotope_count(&self, target: elements_rs::Isotope) -> Option<u64> {
        (**self).isotope_count(target)
    }

    fn number_of_atoms(&self) -> Option<u64> {
        (**self).number_of_atoms()
    }

    fn get_counted_element(&self, index: u64) -> Option<elements_rs::Element> {
        (**self).get_counted_element(index)
    }

    fn get_counted_element_or_size(&self, index: u64) -> Result<elements_rs::Element, u64> {
        (**self).get_counted_element_or_size(index)
    }
}

impl<T: NoResidualsTree> NoResidualsTree for Box<T> {
    fn total_charge(&self) -> f64 {
        (**self).total_charge()
    }

    fn isotopologue_mass_with_charge(&self) -> f64 {
        (**self).isotopologue_mass_with_charge()
    }

    fn isotopologue_mass_without_charge(&self) -> f64 {
        (**self).isotopologue_mass_without_charge()
    }

    fn molar_mass(&self) -> f64 {
        (**self).molar_mass()
    }
}

impl<T: ResidualTree> ResidualTree for Box<T> {
    fn contains_residuals(&self) -> bool {
        (**self).contains_residuals()
    }
}

impl<T: InstantiableTree> InstantiableTree for Box<T> {
    #[inline]
    fn is_leaf(&self) -> bool {
        (**self).is_leaf()
    }

    #[inline]
    fn charge(
        self,
        charge: Self::Signed,
    ) -> Result<Self, ParseError<Self::Signed, Self::Unsigned>> {
        Ok(Box::new((*self).charge(charge)?))
    }

    #[inline]
    fn uncharge(self) -> Result<(Self, Self::Signed), Self> {
        match (*self).uncharge() {
            Ok((tree, charge)) => Ok((Box::new(tree), charge)),
            Err(tree) => Err(Box::new(tree)),
        }
    }

    #[inline]
    fn repeat(
        self,
        times: Self::Unsigned,
    ) -> Result<Self, ParseError<Self::Signed, Self::Unsigned>> {
        Ok(Box::new((*self).repeat(times)?))
    }

    #[inline]
    fn round(self) -> Self {
        Box::new((*self).round())
    }

    #[inline]
    fn square(self) -> Self {
        Box::new((*self).square())
    }

    #[inline]
    fn left_radical(self) -> Self {
        Box::new((*self).left_radical())
    }

    #[inline]
    fn right_radical(self) -> Self {
        Box::new((*self).right_radical())
    }

    #[inline]
    fn residual() -> Result<Self, ParseError<Self::Signed, Self::Unsigned>> {
        Ok(Box::new(T::residual()?))
    }

    #[inline]
    fn element(element: elements_rs::Element) -> Self {
        Box::new(T::element(element))
    }

    #[inline]
    fn isotope(isotope: elements_rs::Isotope) -> Self {
        Box::new(T::isotope(isotope))
    }

    #[inline]
    fn complex(complex: Complex) -> Self {
        Box::new(T::complex(complex))
    }

    fn from_iter<I: IntoIterator<Item = Self>>(
        iter: I,
    ) -> Result<Self, ParseError<Self::Signed, Self::Unsigned>> {
        Ok(Box::new(T::from_iter(iter.into_iter().map(|tree| *tree).collect::<Vec<T>>())?))
    }

    fn into_sequence(self) -> Result<Vec<Self>, Self> {
        Ok((*self).into_sequence()?.into_iter().map(Box::new).collect())
    }
}
