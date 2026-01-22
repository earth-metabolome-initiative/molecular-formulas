//! Submodule implementing the `TryFrom` trait for the `MolecularFormula` struct

use core::str::FromStr;

use elements_rs::{Element, Isotope};

use crate::{InstantiableTree, MolecularFormula, ParseError};

impl<T: InstantiableTree> TryFrom<&str> for MolecularFormula<T>
where
    Isotope: TryFrom<(Element, T::Unsigned), Error = elements_rs::errors::Error>,
{
    type Error = ParseError<T::Signed, T::Unsigned>;

    #[inline]
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::from_str(s)
    }
}
