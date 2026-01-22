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

#[cfg(test)]
mod tests {
    use crate::MolecularFormula;

    #[test]
    fn test_serde_roundtrip() {
        let formula: MolecularFormula = "H2O".parse().unwrap();
        let serialized = serde_json::to_string(&formula).unwrap();
        // Uses subscript characters in JSON string
        assert_eq!(serialized, "\"H₂O\"");
        let deserialized: MolecularFormula = serde_json::from_str(&serialized).unwrap();
        assert_eq!(formula, deserialized);
    }

    #[test]
    fn test_serde_complex() {
        let formula: MolecularFormula = "C6H12O6".parse().unwrap();
        let serialized = serde_json::to_string(&formula).unwrap();
        assert_eq!(serialized, "\"C₆H₁₂O₆\"");
        let deserialized: MolecularFormula = serde_json::from_str(&serialized).unwrap();
        assert_eq!(formula, deserialized);
    }

    #[test]
    fn test_serde_ion() {
        let formula: MolecularFormula = "SO4-2".parse().unwrap();
        let serialized = serde_json::to_string(&formula).unwrap();
        // Expects something like "SO₄⁻²" or however charge is formatted.
        // Let's rely on roundtrip equality mainly if we are unsure of exact formatting,
        // but exact string check is good.
        // Assuming standard display uses superscripts for charge?
        // Let's check display logic or just try roundtrip first.
        let deserialized: MolecularFormula = serde_json::from_str(&serialized).unwrap();
        assert_eq!(formula, deserialized);
    }
}
