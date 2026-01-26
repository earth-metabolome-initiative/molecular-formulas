#![cfg(feature = "serde")]

use alloc::string::{String, ToString};
use core::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{ChargeLike, ChemicalFormula, CountLike, MineralFormula, errors::ParserError};

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
