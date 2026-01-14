//! Test submodule checking that mixture corner cases are parsed correctly.

#[test]
/// Test parsing a simple mixture "H2O.D2O".
fn parse_mixture1() -> Result<(), Box<dyn std::error::Error>> {
    let formula: molecular_formulas::MolecularFormula = "H2O.D2O".parse()?;
    assert!(formula.contains_mixture());
    assert_eq!(formula.number_of_mixtures(), 2);
    assert_eq!(
        formula.mixtures().collect::<Vec<_>>(),
        &[
            &molecular_formulas::MolecularFormula::try_from("H2O")?,
            &molecular_formulas::MolecularFormula::try_from("D2O")?,
        ]
    );
    Ok(())
}

#[test]
/// Test parsing a tri-mixture "H2O.D2O.T2O".
fn parse_mixture2() -> Result<(), Box<dyn std::error::Error>> {
    let formula: molecular_formulas::MolecularFormula = "H2O.D2O.T2O".parse()?;
    assert!(formula.contains_mixture());
    assert_eq!(formula.number_of_mixtures(), 3, "{:#?}", formula);
    assert_eq!(
        formula.mixtures().collect::<Vec<_>>(),
        &[
            &molecular_formulas::MolecularFormula::try_from("H2O")?,
            &molecular_formulas::MolecularFormula::try_from("D2O")?,
            &molecular_formulas::MolecularFormula::try_from("T2O")?,
        ]
    );
    Ok(())
}
