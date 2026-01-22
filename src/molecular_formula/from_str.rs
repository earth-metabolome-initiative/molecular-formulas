//! Submodule implementing the `FromStr` trait for the `MolecularFormula` struct

use core::str::FromStr;

use elements_rs::{Element, Isotope};

use super::{MolecularFormula, parser::Parser};
use crate::molecular_formula::trees::InstantiableTree;

impl<T: InstantiableTree> FromStr for MolecularFormula<T>
where
    Isotope: TryFrom<(Element, T::Unsigned), Error = elements_rs::errors::Error>,
{
    type Err = super::ParseError<T::Signed, T::Unsigned>;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Parser::from(s).parse()
    }
}
