//! Diesel `TEXT` storage for molecular formulas.
#![cfg(any(feature = "diesel-sqlite", feature = "diesel-postgres"))]

use core::str::FromStr;

use diesel::{connection::SimpleConnection, prelude::*, result::Error};
use molecular_formulas::{
    ChemicalFormula, InChIFormula, MineralFormula, ResidualFormula, errors::ParserError,
};

mod schema {
    diesel::table! {
        /// Formula samples in ordinary text columns.
        formula_samples (id) {
            /// Sample identifier.
            id -> Integer,
            /// Chemical formula text.
            chemical -> Text,
            /// InChI formula text.
            inchi -> Text,
            /// Mineral formula text.
            mineral -> Text,
            /// Residual formula text.
            residual -> Text,
        }
    }
}
use schema::formula_samples::dsl::*;

macro_rules! check_formula_storage {
    ($connection:expr) => {{
        let connection = $connection;
        connection
            .batch_execute(
                "CREATE TEMP TABLE formula_samples (
                    id INTEGER PRIMARY KEY, chemical TEXT NOT NULL, inchi TEXT NOT NULL,
                    mineral TEXT NOT NULL, residual TEXT NOT NULL
                )",
            )
            .expect("create formula samples table");

        let chemical_formula = ChemicalFormula::<u32, i32>::from_str("CuSO4.5H2O").unwrap();
        let inchi_formula: InChIFormula = "C2H6O".parse().unwrap();
        let mineral_formula: MineralFormula = "α-SiO2".parse().unwrap();
        let residual_formula: ResidualFormula = "C6H5R".parse().unwrap();
        diesel::insert_into(formula_samples)
            .values((
                id.eq(1),
                chemical.eq(&chemical_formula),
                inchi.eq(&inchi_formula),
                mineral.eq(&mineral_formula),
                residual.eq(&residual_formula),
            ))
            .execute(connection)
            .expect("insert typed formulas");

        let columns = (chemical, inchi, mineral, residual);
        let sample = formula_samples.find(1).select(columns);
        let stored: (String, String, String, String) = sample.get_result(connection).unwrap();
        assert_eq!(
            (&*stored.0, &*stored.1, &*stored.2, &*stored.3),
            ("CuSO₄.5H₂O", "C2H6O", "α-SiO₂", "C₆H₅R"),
        );
        let loaded: (ChemicalFormula<u32, i32>, InChIFormula, MineralFormula, ResidualFormula) =
            sample.get_result(connection).expect("load typed formulas");
        assert_eq!(loaded, (chemical_formula, inchi_formula, mineral_formula, residual_formula));

        diesel::update(formula_samples.find(1)).set(inchi.eq("C2OH5")).execute(connection).unwrap();
        let Err(Error::DeserializationError(error)) =
            formula_samples.find(1).select(inchi).get_result::<InChIFormula>(connection)
        else {
            panic!("expected out-of-order InChI text to fail deserialization");
        };
        assert_eq!(
            error.source().and_then(|source| source.downcast_ref()),
            Some(&ParserError::NotHillOrdered),
        );
    }};
}

#[cfg(feature = "diesel-sqlite")]
#[test]
fn sqlite_stores_and_loads_formula_text() {
    let mut connection = diesel::SqliteConnection::establish(":memory:").unwrap();
    check_formula_storage!(&mut connection);
}

#[cfg(feature = "diesel-postgres")]
#[test]
#[ignore = "requires a live PostgreSQL server and DATABASE_URL"]
fn postgres_stores_and_loads_formula_text() {
    let database_url = std::env::var("DATABASE_URL").expect("set DATABASE_URL for PostgreSQL");
    let mut connection = diesel::PgConnection::establish(&database_url).unwrap();
    check_formula_storage!(&mut connection);
}
