//! Element and explicit isotope presence in a searchable formula catalogue.

use alloc::{string::String, vec::Vec};
use core::fmt::Write;

use elements_rs::{AtomicNumber, MassNumber};

use crate::{ChemicalFormula, ChemicalTree, Empty, MolecularFormula, errors::ParserError};

/// Creates the indexed catalogue on a registered SQLite connection.
#[cfg(feature = "diesel-sqlite")]
pub const SQLITE_SCHEMA: &str = include_str!("index_sql/sqlite.sql");

/// Creates the indexed catalogue in the current PostgreSQL schema.
#[cfg(feature = "diesel-postgres")]
pub const POSTGRES_SCHEMA: &str = include_str!("index_sql/postgres.sql");

/// Records each distinct label once without expanding repeated atoms or groups.
fn collect(tree: &ChemicalTree<u16, i16, Empty>, labels: &mut Vec<(u8, u16)>) {
    let label = match tree {
        ChemicalTree::Element(element) => (element.atomic_number(), 0),
        ChemicalTree::Isotope(isotope) => (isotope.atomic_number(), isotope.mass_number()),
        ChemicalTree::Radical(radical) => return collect(radical.as_ref(), labels),
        ChemicalTree::Charge(charge) => return collect(charge.as_ref(), labels),
        ChemicalTree::Unit(unit) => return collect(unit.as_ref(), labels),
        ChemicalTree::Sequence(sequence) => {
            for child in sequence.iter() {
                collect(child, labels);
            }
            return;
        }
        ChemicalTree::Repeat(repeat) => {
            if *repeat.count() != 0 {
                collect(repeat.node(), labels);
            }
            return;
        }
        ChemicalTree::Extension(Empty) => return,
    };
    if let Err(position) = labels.binary_search(&label) {
        labels.insert(position, label);
    }
}

/// Projects a formula to sorted, distinct `[atomic_number, mass_number]` pairs.
/// A zero mass number denotes an atom without an explicit isotope label.
///
/// # Errors
///
/// Returns the default chemical formula parser's diagnostic on malformed text.
pub fn presence_json(formula: &str) -> Result<String, ParserError> {
    let formula: ChemicalFormula = formula.parse()?;
    let mut labels = Vec::new();
    for (coefficient, tree) in formula.counted_mixtures() {
        if coefficient != 0 {
            collect(tree, &mut labels);
        }
    }
    let mut output = String::with_capacity(2 + labels.len() * 8);
    output.push('[');
    for (position, (element, mass)) in labels.into_iter().enumerate() {
        if position != 0 {
            output.push(',');
        }
        write!(output, "[{element},{mass}]").expect("writing to a String cannot fail");
    }
    output.push(']');
    Ok(output)
}
