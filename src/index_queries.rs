//! Diesel queries over indexed molecular formulas.

use diesel::{
    ExpressionMethods, QueryDsl, dsl, query_builder::AsQuery, query_dsl::RunQueryDslSupport,
    sql_types::BigInt,
};
use elements_rs::{AtomicNumber, Element, Isotope, MassNumber};

diesel::table! {
    /// Source formulas for the presence index.
    mf_documents (id) {
        /// Corpus document identifier.
        id -> BigInt,
        /// Original formula text.
        formula -> Text,
    }
}

diesel::table! {
    /// Distinct element and isotope labels in each document.
    mf_postings (document_id, element, mass) {
        /// Identifier of the indexed formula.
        document_id -> BigInt,
        /// Atomic number.
        element -> Integer,
        /// Isotope mass number, or zero for unlabeled atoms.
        mass -> Integer,
    }
}

diesel::joinable!(mf_postings -> mf_documents (document_id));
diesel::allow_tables_to_appear_in_same_query!(mf_documents, mf_postings);

type ElementPostings = dsl::Order<
    dsl::Select<
        dsl::Filter<mf_postings::table, dsl::Eq<mf_postings::element, i32>>,
        mf_postings::document_id,
    >,
    mf_postings::document_id,
>;

fn element_postings(atomic_number: u8) -> ElementPostings {
    mf_postings::table
        .filter(mf_postings::element.eq(i32::from(atomic_number)))
        .select(mf_postings::document_id)
        .order(mf_postings::document_id)
}

/// Distinct document IDs containing the element with any isotope label, in
/// ascending order.
#[derive(Debug, Clone, Copy)]
pub struct ElementQuery(Element);

impl ElementQuery {
    /// Searches for labeled and unlabeled atoms of `element`.
    #[must_use]
    pub const fn new(element: Element) -> Self {
        Self(element)
    }
}

impl AsQuery for ElementQuery {
    type SqlType = BigInt;
    type Query = dsl::Distinct<ElementPostings>;

    fn as_query(self) -> Self::Query {
        element_postings(self.0.atomic_number()).distinct()
    }
}

impl RunQueryDslSupport for ElementQuery {}

/// Document IDs containing the explicitly labeled isotope, in ascending order.
#[derive(Debug, Clone, Copy)]
pub struct IsotopeQuery(Isotope);

impl IsotopeQuery {
    /// Searches for `isotope` at its exact mass number.
    #[must_use]
    pub const fn new(isotope: Isotope) -> Self {
        Self(isotope)
    }
}

impl AsQuery for IsotopeQuery {
    type SqlType = BigInt;
    type Query = dsl::Filter<ElementPostings, dsl::Eq<mf_postings::mass, i32>>;

    fn as_query(self) -> Self::Query {
        element_postings(self.0.atomic_number())
            .filter(mf_postings::mass.eq(i32::from(self.0.mass_number())))
    }
}

impl RunQueryDslSupport for IsotopeQuery {}
