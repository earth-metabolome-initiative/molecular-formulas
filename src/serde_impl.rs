#![cfg(feature = "serde")]

use alloc::string::{String, ToString};
use core::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    ChargeLike, ChemicalFormula, CountLike, InChIFormula, MineralFormula, ResidualFormula,
    errors::ParserError,
};

impl<Count: CountLike, Charge: ChargeLike> Serialize for ChemicalFormula<Count, Charge> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de, Count: CountLike, Charge: ChargeLike> Deserialize<'de> for ChemicalFormula<Count, Charge>
where
    Self: FromStr<Err = ParserError>,
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ChemicalFormula::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl<Count: CountLike, Charge: ChargeLike> Serialize for MineralFormula<Count, Charge> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de, Count: CountLike, Charge: ChargeLike> Deserialize<'de> for MineralFormula<Count, Charge>
where
    Self: FromStr<Err = ParserError>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        MineralFormula::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl<Count: CountLike, Charge: ChargeLike> Serialize for ResidualFormula<Count, Charge> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de, Count: CountLike, Charge: ChargeLike> Deserialize<'de> for ResidualFormula<Count, Charge>
where
    Self: FromStr<Err = ParserError>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ResidualFormula::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl<Count: CountLike> Serialize for InChIFormula<Count> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de, Count: CountLike> Deserialize<'de> for InChIFormula<Count>
where
    Self: FromStr<Err = ParserError>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        InChIFormula::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;

    use crate::{ChemicalFormula, InChIFormula, MineralFormula, ResidualFormula};

    #[test]
    fn test_chemical_formula_serde() {
        let formula = ChemicalFormula::from_str("C6H12O6").unwrap();
        let serialized = serde_json::to_string(&formula).unwrap();
        assert_eq!(serialized, "\"C₆H₁₂O₆\"");
        let deserialized: ChemicalFormula = serde_json::from_str(&serialized).unwrap();
        assert_eq!(formula, deserialized);
    }

    #[test]
    fn test_mineral_formula_serde() {
        let formula = MineralFormula::from_str("SiO2").unwrap();
        let serialized = serde_json::to_string(&formula).unwrap();
        assert_eq!(serialized, "\"SiO₂\"");
        let deserialized: MineralFormula = serde_json::from_str(&serialized).unwrap();
        assert_eq!(formula, deserialized);
    }

    #[test]
    fn test_residual_formula_serde() {
        let formula = ResidualFormula::from_str("C6H12O6").unwrap();
        let serialized = serde_json::to_string(&formula).unwrap();
        assert_eq!(serialized, "\"C₆H₁₂O₆\"");
        let deserialized: ResidualFormula = serde_json::from_str(&serialized).unwrap();
        assert_eq!(formula, deserialized);
    }

    #[test]
    fn test_inchi_formula_serde() {
        let s = "C2H6O";
        let formula = InChIFormula::from_str(s).unwrap();
        let serialized = serde_json::to_string(&formula).unwrap();
        assert_eq!(serialized, "\"C2H6O\"");
        let deserialized: InChIFormula = serde_json::from_str(&serialized).unwrap();
        assert_eq!(formula, deserialized);
    }
}
