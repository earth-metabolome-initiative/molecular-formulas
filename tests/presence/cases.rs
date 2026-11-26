use diesel::{connection::SimpleConnection, dsl::sql, prelude::*, sql_types::Text};
use molecular_formulas::{
    index_queries::{ElementQuery, IsotopeQuery, mf_documents},
    prelude::{Element, Isotope},
};

use super::TestConnection;

pub(super) fn element(connection: &mut TestConnection, element: Element) -> Vec<i64> {
    ElementQuery::new(element).load::<i64>(connection).unwrap()
}

fn carbon_isotope(connection: &mut TestConnection, mass: u8) -> Vec<i64> {
    let isotope = Isotope::try_from((Element::C, mass)).unwrap();
    IsotopeQuery::new(isotope).load::<i64>(connection).unwrap()
}

pub(super) fn check_presence(connection: &mut TestConnection) {
    let projection = diesel::select(sql::<Text>("mf_presence('[13C]C2.C')"))
        .get_result::<String>(connection)
        .unwrap();
    let mut terms: Vec<[u16; 2]> = serde_json::from_str(&projection).unwrap();
    terms.sort_unstable();
    assert_eq!(terms, [[6, 0], [6, 13]]);

    for (identifier, formula) in [(30_i64, "[13C]C"), (10, "[13C]C"), (20, "C"), (40, "Fe")] {
        diesel::insert_into(mf_documents::table)
            .values((mf_documents::id.eq(identifier), mf_documents::formula.eq(formula)))
            .execute(connection)
            .unwrap();
    }
    assert_eq!(element(connection, Element::C), [10, 20, 30]);
    assert_eq!(carbon_isotope(connection, 13), [10, 30]);
    assert!(carbon_isotope(connection, 12).is_empty());
    assert_eq!(element(connection, Element::Fe), [40]);
    assert!(element(connection, Element::F).is_empty());

    connection.batch_execute("UPDATE mf_documents SET formula = '[12C]' WHERE id = 10").unwrap();
    assert_eq!(carbon_isotope(connection, 12), [10]);
    assert_eq!(carbon_isotope(connection, 13), [30]);
    connection.batch_execute("UPDATE mf_documents SET id = 50 WHERE id = 10").unwrap();
    assert_eq!(element(connection, Element::C), [20, 30, 50]);
    assert_eq!(carbon_isotope(connection, 12), [50]);
    connection.batch_execute("DELETE FROM mf_documents WHERE id = 30").unwrap();
    assert!(carbon_isotope(connection, 13).is_empty());
    assert_eq!(element(connection, Element::C), [20, 50]);

    for statement in [
        "INSERT INTO mf_documents VALUES (60, 'F'), (70, 'not a formula')",
        "UPDATE mf_documents SET id = 60, formula = '(OH' WHERE id = 50",
    ] {
        assert!(
            connection
                .transaction::<_, diesel::result::Error, _>(|connection| {
                    connection.batch_execute(statement)
                })
                .is_err(),
            "{statement}"
        );
        assert_eq!(carbon_isotope(connection, 12), [50]);
        assert!(element(connection, Element::F).is_empty());
        assert_eq!(
            mf_documents::table.order(mf_documents::id).load::<(i64, String)>(connection).unwrap(),
            [(20, "C".into()), (40, "Fe".into()), (50, "[12C]".into())]
        );
    }
}
