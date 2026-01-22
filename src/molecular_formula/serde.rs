#![cfg(feature = "serde")]
//! Submodule implementing serialization and deserialization for the
//! `MolecularFormula` struct using serialization and deserialization traits
//! with `String` as the underlying type.
use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{MolecularFormula, ParseError, Tree};

impl<T: Tree + Display> Serialize for MolecularFormula<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de, T: Tree> Deserialize<'de> for MolecularFormula<T>
where
    Self: FromStr<Err = ParseError<T::Signed, T::Unsigned>>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        MolecularFormula::from_str(&s).map_err(serde::de::Error::custom)
    }
}
