//! Atom totals at the limits of the platform's count type.

use molecular_formulas::{
    ChemicalFormula, MolecularFormula, MolecularTree,
    errors::{CountError, NumericError},
};

#[test]
fn atom_totals_at_the_32_bit_boundary() {
    for (input, expected) in [
        ("65535H65537", 4_294_967_295_u64),
        ("65536H65536", 4_294_967_296),
        ("(H65536)65536", 4_294_967_296),
        ("H4294967295O", 4_294_967_296),
        ("H4294967295.O", 4_294_967_296),
    ] {
        let formula: ChemicalFormula<u32, i32> = input.parse().unwrap();
        let expected = usize::try_from(expected)
            .map_err(|_| CountError::Numeric(NumericError::PositiveOverflow));
        assert_eq!(formula.number_of_elements(), expected, "{input}");
    }
}

#[test]
fn excluded_hydrogen_does_not_overflow_the_non_hydrogen_total() {
    let formula: ChemicalFormula<u32, i32> = "65536((H65536)65536)65536.O".parse().unwrap();
    assert_eq!(
        formula.number_of_elements(),
        Err(CountError::Numeric(NumericError::PositiveOverflow))
    );
    assert_eq!(formula.number_of_non_hydrogens(), Ok(1));
}

#[test]
fn component_count_excludes_its_mixture_coefficient() {
    let formula: ChemicalFormula<u32, i32> = "2([13C]H65535)65536".parse().unwrap();
    let (_, group) = formula.counted_mixtures().next().unwrap();
    assert_eq!(
        group.number_of_elements(),
        usize::try_from(4_294_967_296_u64)
            .map_err(|_| CountError::Numeric(NumericError::PositiveOverflow))
    );
    assert_eq!(formula.number_of_non_hydrogens(), Ok(131_072));
}
