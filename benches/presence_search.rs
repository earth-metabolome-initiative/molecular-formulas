//! Measures complete ordered element and isotope searches on matched synthetic
//! catalogues.

use std::{env, fs::File, time::Instant};

use diesel::{
    backend::Backend,
    connection::{DefaultLoadingMode, SimpleConnection},
    dsl::sql,
    prelude::*,
    query_builder::{AsQuery, AstPass, Query, QueryFragment, QueryId},
    query_dsl::RunQueryDslSupport,
    sql_types::{BigInt, Text, Untyped},
};
use molecular_formulas::{
    formula_index::{POSTGRES_SCHEMA, SQLITE_SCHEMA},
    index_queries::{ElementQuery, IsotopeQuery, mf_documents},
    prelude::{ChemicalFormula, Element, Isotope, MolecularFormula, MolecularTree},
};

const SEED: u32 = 0x4d46_5052;
const DOCUMENTS: i64 = 6_000;
const RUNS: u32 = 8;
const CARBON: u8 = 1;
const FLUORINE: u8 = 2;
const CARBON_THIRTEEN: u8 = 4;
const GOLD: u8 = 8;
const FORMULAS: [(&str, u8); 12] = [
    ("C6H12O6", CARBON),
    ("CH4", CARBON),
    ("Fe2O3", 0),
    ("CaF2", FLUORINE),
    ("[13C]H4", CARBON | CARBON_THIRTEEN),
    ("[13C]C2H6", CARBON | CARBON_THIRTEEN),
    ("H2O", 0),
    ("CuSO4.5H2O", 0),
    ("C2H5F", CARBON | FLUORINE),
    ("FeF3", FLUORINE),
    ("(CH2)6", CARBON),
    ("NaCl", 0),
];

fn documents() -> impl Iterator<Item = (i64, &'static str, u8)> {
    let mut state = SEED;
    (1..=DOCUMENTS).map(move |identifier| {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let position = usize::try_from(state >> 16).unwrap() % FORMULAS.len();
        let (formula, matches) = FORMULAS[position];
        (identifier, formula, matches)
    })
}

#[derive(Clone, Copy)]
enum Needle {
    Element(Element),
    Isotope(Isotope),
}

impl Needle {
    fn matches(self, formula: &ChemicalFormula) -> bool {
        match self {
            Self::Element(element) => formula.contains_element(element),
            Self::Isotope(isotope) => formula.mixtures().any(|tree| tree.contains_isotope(isotope)),
        }
    }
}

fn scan(
    rows: impl Iterator<Item = QueryResult<(i64, ChemicalFormula)>>,
    needle: Needle,
) -> Vec<i64> {
    rows.filter_map(|row| {
        let (identifier, formula) = row.unwrap();
        needle.matches(&formula).then_some(identifier)
    })
    .collect()
}

#[derive(QueryableByName)]
struct SqlitePlan {
    #[diesel(sql_type = Text)]
    detail: String,
}

#[derive(QueryableByName)]
struct PostgresPlan {
    #[diesel(sql_type = Text, column_name = "QUERY PLAN")]
    detail: String,
}

struct Explain<Statement>(&'static str, Statement);

impl<Statement> QueryId for Explain<Statement> {
    type QueryId = ();
    const HAS_STATIC_QUERY_ID: bool = false;
}

impl<Statement> Query for Explain<Statement> {
    type SqlType = Untyped;
}

impl<Statement> RunQueryDslSupport for Explain<Statement> {}

impl<Database: Backend, Statement: QueryFragment<Database>> QueryFragment<Database>
    for Explain<Statement>
{
    fn walk_ast<'query>(&'query self, mut pass: AstPass<'_, 'query, Database>) -> QueryResult<()> {
        pass.push_sql(self.0);
        self.1.walk_ast(pass.reborrow())
    }
}

fn measure(engine: &str, method: &str, mut execute: impl FnMut(Needle) -> Vec<i64>) {
    let queries = [
        ("C", CARBON, Needle::Element(Element::C)),
        ("F", FLUORINE, Needle::Element(Element::F)),
        ("13C", CARBON_THIRTEEN, Needle::Isotope(Isotope::try_from((Element::C, 13_u8)).unwrap())),
        ("Au", GOLD, Needle::Element(Element::Au)),
    ];
    let expected = queries.map(|(_, mask, _)| {
        documents()
            .filter_map(|(identifier, _, matches)| (matches & mask != 0).then_some(identifier))
            .collect::<Vec<_>>()
    });
    let mut samples = [[0.0; 8]; 4];
    for round in 0..=RUNS {
        let order = queries.iter().copied().enumerate();
        for (position, (label, _, needle)) in
            order.cycle().skip(usize::try_from(round).unwrap()).take(queries.len())
        {
            let start = Instant::now();
            let result = execute(needle);
            let elapsed = start.elapsed().as_secs_f64() * 1e6;
            assert_eq!(result, expected[position], "{engine} {method} {label} round {round}");
            if round != 0 {
                samples[position][usize::try_from(round - 1).unwrap()] = elapsed;
                println!("sample_us\t{engine}\t{method}\t{label}\t{round}\t{elapsed:.3}");
            }
        }
    }
    for (position, (label, _, _)) in queries.iter().enumerate() {
        let times = samples[position];
        let mean = times.iter().sum::<f64>() / f64::from(RUNS);
        let deviation = (times.iter().map(|time| (time - mean).powi(2)).sum::<f64>()
            / f64::from(RUNS - 1))
        .sqrt();
        println!(
            "summary_hits_mean_us_sample_sd_us\t{engine}\t{method}\t{label}\t{}\t{mean:.3}\t{deviation:.3}",
            expected[position].len()
        );
    }
}

macro_rules! campaign {
    ($connection:ident, $engine:literal, $schema:expr, $maintenance:literal, $prefix:literal, $plan:ty, $storage:literal) => {{
        $connection.batch_execute($schema).unwrap();
        $connection.transaction::<_, diesel::result::Error, _>(|connection| {
            for (identifier, formula, _) in documents() {
                diesel::insert_into(mf_documents::table)
                    .values((mf_documents::id.eq(identifier), mf_documents::formula.eq(formula)))
                    .execute(connection)?;
            }
            Ok(())
        }).unwrap();
        $connection.batch_execute($maintenance).unwrap();
        let (source, postings, lookup) = sql::<(BigInt, BigInt, BigInt)>($storage)
            .get_result::<(i64, i64, i64)>(&mut $connection).unwrap();
        println!("bytes_source_postings_lookup_incremental_total\t{}\t{source}\t{postings}\t{lookup}\t{}", $engine, postings + lookup);
        let raw = mf_documents::table.select((mf_documents::id, mf_documents::formula)).order(mf_documents::id);
        for line in Explain($prefix, raw).load::<$plan>(&mut $connection).unwrap() {
            println!("plan\t{}\traw\tall\t{}", $engine, line.detail);
        }
        measure($engine, "raw", |needle| {
            scan(raw.load_iter::<(i64, ChemicalFormula), DefaultLoadingMode>(&mut $connection).unwrap(), needle)
        });
        for method in ["indexed", "without_lookup"] {
            if method == "without_lookup" {
                $connection.batch_execute("DROP INDEX mf_postings_lookup").unwrap();
            }
            for (label, element) in [("C", Element::C), ("F", Element::F), ("Au", Element::Au)] {
                let statement = ElementQuery::new(element).as_query();
                for line in Explain($prefix, statement).load::<$plan>(&mut $connection).unwrap() {
                    println!("plan\t{}\t{method}\t{label}\t{}", $engine, line.detail);
                }
            }
            let statement = IsotopeQuery::new(Isotope::try_from((Element::C, 13_u8)).unwrap()).as_query();
            for line in Explain($prefix, statement).load::<$plan>(&mut $connection).unwrap() {
                println!("plan\t{}\t{method}\t13C\t{}", $engine, line.detail);
            }
            measure($engine, method, |needle| match needle {
                Needle::Element(element) => ElementQuery::new(element).load::<i64>(&mut $connection).unwrap(),
                Needle::Isotope(isotope) => IsotopeQuery::new(isotope).load::<i64>(&mut $connection).unwrap(),
            });
        }
    }};
}

fn main() {
    let mut arguments = env::args().skip(1);
    let path = arguments.next().expect("supply a new SQLite file path");
    let url = arguments.next().expect("supply a fresh PostgreSQL 18 database URL");
    drop(File::create_new(&path).expect("SQLite path must be new"));
    let mut sqlite = SqliteConnection::establish(&path).unwrap();
    molecular_formulas::sqlite::register(&mut sqlite).unwrap();
    let mut postgres = PgConnection::establish(&url).unwrap();
    let version = sql::<BigInt>("SELECT current_setting('server_version_num')::bigint")
        .get_result::<i64>(&mut postgres)
        .unwrap();
    assert_eq!(version / 10_000, 18, "benchmark requires PostgreSQL 18");
    let sqlite_version =
        sql::<Text>("SELECT sqlite_version()").get_result::<String>(&mut sqlite).unwrap();
    println!(
        "environment\tsqlite={sqlite_version}\tpostgres={version}\tseed={SEED}\tdocuments={DOCUMENTS}\twarmups=1\truns={RUNS}"
    );
    campaign!(
        sqlite,
        "sqlite",
        SQLITE_SCHEMA,
        "ANALYZE",
        "EXPLAIN QUERY PLAN ",
        SqlitePlan,
        "SELECT SUM(CASE WHEN name='mf_documents' THEN pgsize ELSE 0 END),
                SUM(CASE WHEN name='mf_postings' THEN pgsize ELSE 0 END),
                SUM(CASE WHEN name='mf_postings_lookup' THEN pgsize ELSE 0 END) FROM dbstat"
    );
    campaign!(postgres, "postgres", POSTGRES_SCHEMA, "VACUUM (ANALYZE) mf_documents, mf_postings", "EXPLAIN ", PostgresPlan,
        "SELECT pg_total_relation_size('mf_documents'),
                pg_total_relation_size('mf_postings') - pg_total_relation_size('mf_postings_lookup'),
                pg_total_relation_size('mf_postings_lookup')");
}
