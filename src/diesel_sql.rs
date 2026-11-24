//! Diesel `Text` codecs for molecular formulas.

#[cfg(feature = "diesel-sqlite")]
use alloc::string::ToString;
use core::str::FromStr;
#[cfg(any(feature = "diesel-postgres", feature = "diesel-mariadb", feature = "diesel-mysql"))]
use std::io::Write;

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
        impl_server_text!(diesel::pg::Pg, $formula<$($parameter: $bound),+>);
        #[cfg(feature = "diesel-mariadb")]
        impl_server_text!(diesel::mariadb::Mariadb, $formula<$($parameter: $bound),+>);
        #[cfg(feature = "diesel-mysql")]
        impl_server_text!(diesel::mysql::Mysql, $formula<$($parameter: $bound),+>);
    };
}

#[cfg(any(feature = "diesel-postgres", feature = "diesel-mariadb", feature = "diesel-mysql"))]
macro_rules! impl_server_text {
    ($backend:ty, $formula:ident<$($parameter:ident: $bound:ident),+>) => {
        impl<$($parameter: $bound),+> FromSql<Text, $backend> for $formula<$($parameter),+>
        where
            Self: FromStr<Err = ParserError>,
        {
            fn from_sql(value: <$backend as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
                core::str::from_utf8(value.as_bytes())?.parse().map_err(Into::into)
            }
        }

        impl<$($parameter: $bound),+> ToSql<Text, $backend> for $formula<$($parameter),+> {
            fn to_sql<'bind>(
                &'bind self,
                output: &mut serialize::Output<'bind, '_, $backend>,
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
