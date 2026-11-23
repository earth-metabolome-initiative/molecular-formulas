//! Diesel `Text` codecs for molecular formulas.

#[cfg(feature = "diesel-sqlite")]
use alloc::string::ToString;
use core::str::FromStr;
#[cfg(feature = "diesel-postgres")]
use std::io::Write;

#[cfg(feature = "diesel-postgres")]
use diesel::pg::Pg;
#[cfg(feature = "diesel-sqlite")]
use diesel::sqlite::Sqlite;
use diesel::{
    backend::Backend,
    deserialize::{self, FromSql},
    serialize::{self, ToSql},
    sql_types::Text,
};

use crate::{
    ChargeLike, ChemicalFormula, CountLike, InChIFormula, MineralFormula, ResidualFormula,
    errors::ParserError,
};

macro_rules! impl_formula_sql {
    ($formula:ident<$($parameter:ident: $bound:ident),+>) => {
        #[cfg(feature = "diesel-sqlite")]
        impl<$($parameter: $bound),+> FromSql<Text, Sqlite> for $formula<$($parameter),+>
        where
            Self: FromStr<Err = ParserError>,
        {
            fn from_sql(mut value: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
                value.read_text().parse().map_err(Into::into)
            }
        }

        #[cfg(feature = "diesel-sqlite")]
        impl<$($parameter: $bound),+> ToSql<Text, Sqlite> for $formula<$($parameter),+> {
            fn to_sql<'bind>(
                &'bind self,
                output: &mut serialize::Output<'bind, '_, Sqlite>,
            ) -> serialize::Result {
                output.set_value(self.to_string());
                Ok(serialize::IsNull::No)
            }
        }

        #[cfg(feature = "diesel-postgres")]
        impl<$($parameter: $bound),+> FromSql<Text, Pg> for $formula<$($parameter),+>
        where
            Self: FromStr<Err = ParserError>,
        {
            fn from_sql(value: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
                core::str::from_utf8(value.as_bytes())?.parse().map_err(Into::into)
            }
        }

        #[cfg(feature = "diesel-postgres")]
        impl<$($parameter: $bound),+> ToSql<Text, Pg> for $formula<$($parameter),+> {
            fn to_sql<'bind>(
                &'bind self,
                output: &mut serialize::Output<'bind, '_, Pg>,
            ) -> serialize::Result {
                write!(output, "{self}")?;
                Ok(serialize::IsNull::No)
            }
        }
    };
}

impl_formula_sql!(ChemicalFormula<Count: CountLike, Charge: ChargeLike>);
impl_formula_sql!(InChIFormula<Count: CountLike>);
impl_formula_sql!(MineralFormula<Count: CountLike, Charge: ChargeLike>);
impl_formula_sql!(ResidualFormula<Count: CountLike, Charge: ChargeLike>);
