//! Generic extendible tree implementation for molecular formulas.

use std::fmt::Display;

use elements_rs::{Element, ElementVariant, Isotope, MassNumber, isotopes::HydrogenIsotope};

use crate::{
    Bracket, CharacterMarker, ChargeLike, Complex, CountLike, ParseError, Radical, Residual,
    SubTokenError, SuperscriptMinus, SuperscriptPlus, Tree,
    molecular_formula::{
        Side,
        parser::{subscript_digits_ltr, superscript_digits_ltr},
        trees::{InstantiableTree, empty_tree::EmptyTree},
    },
};

mod no_residuals_tree_impl;
mod residual_tree_impl;
mod tree_impl;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Generic extendible tree for molecular formulas.
pub enum GenericTree<S: ChargeLike, U: CountLike, Extension = EmptyTree<S, U>> {
    /// An atom (element)
    Element(Element),
    /// An isotope (element with mass number)
    Isotope(Isotope),
    /// A left-hand side radical.
    Radical(Box<Self>, Side),
    /// An ion (element or molecule with charge)
    Charge(Box<Self>, S),
    /// Number of molecules
    Repeat(Box<Self>, U),
    /// A sequence of molecular formulas
    Sequence(Vec<Self>),
    /// A repeating unit wrapped in round brackets
    Unit(Box<Self>, Bracket),
    /// Extension point for future variants
    Extension(Extension),
}

impl<S: ChargeLike + TryFrom<U>, U: CountLike, T: TryFrom<Residual, Error = ParseError<S, U>>>
    TryFrom<Residual> for GenericTree<S, U, T>
{
    type Error = ParseError<S, U>;

    fn try_from(residual: Residual) -> Result<Self, Self::Error> {
        Ok(GenericTree::Extension(T::try_from(residual)?))
    }
}

impl<S: ChargeLike + TryFrom<U>, U: CountLike, T: Tree<Unsigned = U, Signed = S>> From<Complex>
    for GenericTree<S, U, T>
where
    Self: InstantiableTree<Unsigned = U, Signed = S>,
{
    fn from(complex: Complex) -> Self {
        match complex {
            Complex::Benzyl => {
                Self::from_iter([
                    Self::element(Element::C).repeat(U::SEVEN).unwrap(),
                    Self::element(Element::H).repeat(U::SEVEN).unwrap(),
                ])
                .unwrap()
                .round()
            }
            Complex::Butyl => {
                Self::from_iter([
                    Self::element(Element::C).repeat(U::FOUR).unwrap(),
                    Self::element(Element::H).repeat(U::NINE).unwrap(),
                ])
                .unwrap()
                .round()
            }
            Complex::Phenyl => {
                Self::from_iter([
                    Self::element(Element::C).repeat(U::SIX).unwrap(),
                    Self::element(Element::H).repeat(U::FIVE).unwrap(),
                ])
                .unwrap()
                .round()
            }
            Complex::Cyclohexyl => {
                Self::from_iter([
                    Self::element(Element::C).repeat(U::SIX).unwrap(),
                    Self::element(Element::H).repeat(U::ELEVEN).unwrap(),
                ])
                .unwrap()
                .round()
            }
            Complex::Ethyl => {
                Self::from_iter([
                    Self::element(Element::C).repeat(U::TWO).unwrap(),
                    Self::element(Element::H).repeat(U::FIVE).unwrap(),
                ])
                .unwrap()
                .round()
            }
            Complex::Methyl => {
                Self::from_iter([
                    Self::element(Element::C).repeat(U::ONE).unwrap(),
                    Self::element(Element::H).repeat(U::THREE).unwrap(),
                ])
                .unwrap()
                .round()
            }
            Complex::Cyclopentadienyl => {
                Self::charge(
                    Self::from_iter([
                        Self::element(Element::C).repeat(U::FIVE).unwrap(),
                        Self::element(Element::H).repeat(U::FIVE).unwrap(),
                    ])
                    .unwrap()
                    .round(),
                    -S::ONE,
                )
                .unwrap()
            }
        }
    }
}

impl<S: ChargeLike + TryFrom<U>, U: CountLike, T: InstantiableTree<Unsigned = U, Signed = S>>
    From<Element> for GenericTree<S, U, T>
{
    fn from(element: Element) -> Self {
        Self::element(element)
    }
}

impl<S: ChargeLike + TryFrom<U>, U: CountLike, T: InstantiableTree<Unsigned = U, Signed = S>>
    From<Element> for Box<GenericTree<S, U, T>>
{
    fn from(element: Element) -> Self {
        Box::new(element.into())
    }
}

impl<S: ChargeLike + TryFrom<U>, U: CountLike, T: InstantiableTree<Unsigned = U, Signed = S>>
    From<Isotope> for GenericTree<S, U, T>
{
    fn from(isotope: Isotope) -> Self {
        Self::isotope(isotope)
    }
}

impl<S: ChargeLike + TryFrom<U>, U: CountLike, T: InstantiableTree<Unsigned = U, Signed = S>>
    From<Isotope> for Box<GenericTree<S, U, T>>
{
    fn from(isotope: Isotope) -> Self {
        Box::new(isotope.into())
    }
}

impl<S: ChargeLike + TryFrom<U>, U: CountLike, E: Tree<Unsigned = U, Signed = S> + InstantiableTree>
    InstantiableTree for GenericTree<S, U, E>
{
    fn is_leaf(&self) -> bool {
        match self {
            Self::Element(_) | Self::Isotope(_) => true,
            Self::Extension(extension) => extension.is_leaf(),
            _ => false,
        }
    }

    #[inline]
    fn repeat(
        self,
        times: Self::Unsigned,
    ) -> Result<Self, ParseError<Self::Signed, Self::Unsigned>> {
        if times == U::ZERO {
            return Ok(self);
        }

        if let Self::Repeat(inner, existing_times) = self {
            return (*inner)
                .repeat(existing_times.checked_add(&times).ok_or(SubTokenError::NumericOverflow)?);
        }

        Ok(Self::Repeat(Box::new(self), times))
    }

    #[inline]
    fn charge(
        self,
        charge: Self::Signed,
    ) -> Result<Self, ParseError<Self::Signed, Self::Unsigned>> {
        if charge == S::ZERO {
            return Ok(self);
        }

        if let Self::Charge(inner, existing_charge) = self {
            let new_charge =
                existing_charge.checked_add(&charge).ok_or(SubTokenError::NumericOverflow)?;
            return (*inner).charge(new_charge);
        }

        Ok(Self::Charge(Box::new(self), charge))
    }

    #[inline]
    fn uncharge(self) -> Result<(Self, Self::Signed), Self> {
        if let Self::Charge(inner, charge) = self { Ok((*inner, charge)) } else { Err(self) }
    }

    #[inline]
    fn round(self) -> Self {
        Self::Unit(Box::new(self), Bracket::Round)
    }

    #[inline]
    fn square(self) -> Self {
        Self::Unit(Box::new(self), Bracket::Square)
    }

    #[inline]
    fn residual() -> Result<Self, ParseError<Self::Signed, Self::Unsigned>> {
        Ok(Self::Extension(E::residual()?))
    }

    #[inline]
    fn left_radical(self) -> Self {
        Self::Radical(Box::new(self), Side::Left)
    }

    #[inline]
    fn right_radical(self) -> Self {
        Self::Radical(Box::new(self), Side::Right)
    }

    #[inline]
    fn element(element: Element) -> Self {
        Self::Element(element)
    }

    #[inline]
    fn isotope(isotope: Isotope) -> Self {
        Self::Isotope(isotope)
    }

    #[inline]
    fn complex(complex: Complex) -> Self {
        Self::from(complex)
    }

    fn from_iter<I: IntoIterator<Item = Self>>(
        iter: I,
    ) -> Result<Self, ParseError<Self::Signed, Self::Unsigned>> where {
        let mut iter = iter.into_iter().peekable();
        let Some(next) = iter.next() else {
            return Err(ParseError::EmptySequenceNotSupportedInCurrentTree);
        };
        if iter.peek().is_none() {
            Ok(next)
        } else {
            let mut formulas = vec![next];
            formulas.extend(iter);
            Ok(Self::Sequence(formulas))
        }
    }

    fn into_sequence(self) -> Result<Vec<Self>, Self> {
        match self {
            Self::Sequence(formulas) => Ok(formulas),
            tree => Ok(vec![tree]),
        }
    }
}

impl<S: ChargeLike, U: CountLike, E: Display> Display for GenericTree<S, U, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Element(element) => write!(f, "{element}"),
            Self::Isotope(isotope) => {
                // Names isotope special case
                if matches!(isotope, Isotope::H(HydrogenIsotope::D | HydrogenIsotope::T)) {
                    write!(f, "{isotope}")
                } else {
                    write!(f, "[")?;
                    for digit in superscript_digits_ltr(isotope.mass_number()) {
                        write!(f, "{digit}")?;
                    }
                    write!(f, "{}", isotope.element())?;
                    write!(f, "]")
                }
            }
            Self::Sequence(formulas) => {
                for formula in formulas {
                    write!(f, "{formula}")?;
                }
                Ok(())
            }
            Self::Charge(formula, charge) => {
                write!(f, "{formula}")?;
                if charge.abs() != S::ONE {
                    for digit in superscript_digits_ltr(charge.abs()) {
                        write!(f, "{digit}")?;
                    }
                }
                if charge > &S::ZERO {
                    write!(f, "{}", SuperscriptPlus::CANONICAL)?;
                }
                if charge < &S::ZERO {
                    write!(f, "{}", SuperscriptMinus::CANONICAL)?;
                }
                Ok(())
            }
            Self::Repeat(formula, count) => {
                write!(f, "{formula}")?;
                for digit in subscript_digits_ltr(*count) {
                    write!(f, "{digit}")?;
                }
                Ok(())
            }
            Self::Unit(formula, bracket) => {
                match bracket {
                    Bracket::Round => write!(f, "({formula})"),
                    Bracket::Square => write!(f, "[{formula}]"),
                }
            }
            Self::Radical(formula, side) => {
                match side {
                    Side::Left => write!(f, "{Radical}{formula}"),
                    Side::Right => write!(f, "{formula}{Radical}"),
                }
            }
            Self::Extension(e) => write!(f, "{e}"),
        }
    }
}
