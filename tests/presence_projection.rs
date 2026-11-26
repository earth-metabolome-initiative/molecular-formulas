//! Distinct labels projected from parsed formula trees.

use molecular_formulas::formula_index::presence_json;

#[test]
fn mixed_labels_groups_and_components_contribute_once() {
    assert_eq!(presence_json("C([13C]H2)2.C").unwrap(), "[[1,0],[6,0],[6,13]]");
}

#[test]
fn presence_does_not_multiply_large_nested_counts() {
    assert_eq!(presence_json("(((H65535)65535)65535)65535").unwrap(), "[[1,0]]");
}

#[test]
fn malformed_formula_keeps_its_parse_error() {
    assert!(presence_json("(").is_err());
}
