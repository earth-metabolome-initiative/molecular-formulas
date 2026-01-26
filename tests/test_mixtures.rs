//! Test submodule checking that mixture corner cases are parsed correctly.

use molecular_formulas::prelude::*;

#[test]
/// Test parsing a simple mixture "H2O.D2O".
fn parse_mixture1() -> Result<(), Box<dyn std::error::Error>> {
    let formula: ChemicalFormula = "H2O.D2O".parse()?;
    assert_eq!(formula.number_of_mixtures(), 2);
    assert_eq!(formula, ChemicalFormula::try_from("H2O")? + ChemicalFormula::try_from("D2O")?);
    Ok(())
}

#[test]
/// Test parsing a tri-mixture "H2O.D2O.T2O".
fn parse_mixture2() -> Result<(), Box<dyn std::error::Error>> {
    let formula: ChemicalFormula = "H2O.D2O.T2O".parse()?;
    assert_eq!(formula.number_of_mixtures(), 3, "{formula:#?}");
    assert_eq!(
        formula,
        ChemicalFormula::try_from("H2O")?
            + ChemicalFormula::try_from("D2O")?
            + ChemicalFormula::try_from("T2O")?
    );
    Ok(())
}

#[test]
/// Test parsing a tri-mixture "H2O.2H20".
fn parse_mixture3() -> Result<(), Box<dyn std::error::Error>> {
    let formula: ChemicalFormula = "H2O.2H20".parse()?;
    assert_eq!(formula.number_of_mixtures(), 3, "{formula:#?}");
    assert_eq!(
        formula,
        ChemicalFormula::try_from("H2O")?
            + ChemicalFormula::try_from("H20")?
            + ChemicalFormula::try_from("H20")?,
    );
    Ok(())
}

#[test]
/// Test parsing a mixture with hydrate "CuSO4.5H2O".
fn parse_mixture4() -> Result<(), Box<dyn std::error::Error>> {
    let formula: ChemicalFormula = "CuSO4.5H2O".parse()?;
    assert_eq!(formula.number_of_mixtures(), 6, "{formula:#?}");
    assert_eq!(
        formula,
        ChemicalFormula::try_from("CuSO4")?
            + ChemicalFormula::try_from("H2O")?
            + ChemicalFormula::try_from("H2O")?
            + ChemicalFormula::try_from("H2O")?
            + ChemicalFormula::try_from("H2O")?
            + ChemicalFormula::try_from("H2O")?,
    );
    Ok(())
}

#[test]
fn parse_mixture_with_complex_zncl2_2etoh() -> Result<(), Box<dyn std::error::Error>> {
    use elements_rs::Element;
    let formula: ChemicalFormula = "ZnCl2.2EtOH".parse()?;
    assert_eq!(formula.number_of_mixtures(), 3);

    // Check total counts
    // Zn: 1
    // Cl: 2
    // C: 2 * 2 = 4 (EtOH is C2H5OH - C2H6O)
    // H: 2 * 6 = 12
    // O: 2 * 1 = 2
    assert_eq!(formula.count_of_element(Element::Zn), Some(1), "Zn count incorrect");
    assert_eq!(formula.count_of_element(Element::Cl), Some(2), "Cl count incorrect");
    assert_eq!(formula.count_of_element(Element::C), Some(4), "C count incorrect");
    assert_eq!(formula.count_of_element(Element::H), Some(12), "H count incorrect");
    assert_eq!(formula.count_of_element(Element::O), Some(2), "O count incorrect");

    Ok(())
}

#[test]
fn parse_mixture_with_complex_sncl4_2et2o() -> Result<(), Box<dyn std::error::Error>> {
    use elements_rs::Element;
    let formula: ChemicalFormula = "SnCl4.2Et2O".parse()?;
    assert_eq!(formula.number_of_mixtures(), 3);

    // SnCl4 + 2 * (C2H5)2O
    // Et2O -> (C2H5)2O -> C4H10O
    // Total C: 2 * 4 = 8
    // Total H: 2 * 10 = 20
    // Total O: 2 * 1 = 2
    // Sn: 1
    // Cl: 4

    assert_eq!(formula.count_of_element(Element::Sn), Some(1), "Sn count incorrect");
    assert_eq!(formula.count_of_element(Element::Cl), Some(4), "Cl count incorrect");
    assert_eq!(formula.count_of_element(Element::C), Some(8), "C count incorrect");
    assert_eq!(formula.count_of_element(Element::H), Some(20), "H count incorrect");
    assert_eq!(formula.count_of_element(Element::O), Some(2), "O count incorrect");

    Ok(())
}

#[test]
fn parse_mixture_complex_organic() -> Result<(), Box<dyn std::error::Error>> {
    use elements_rs::Element;
    let formula: ChemicalFormula = "C21H23NO5.3EtOH".parse()?;
    assert_eq!(formula.number_of_mixtures(), 4);

    // C21H23NO5 + 3 * C2H6O
    // C: 21 + 3*2 = 27
    // H: 23 + 3*6 = 41
    // N: 1
    // O: 5 + 3*1 = 8

    assert_eq!(formula.count_of_element(Element::C), Some(27), "C count incorrect");
    assert_eq!(formula.count_of_element(Element::H), Some(41), "H count incorrect");
    assert_eq!(formula.count_of_element(Element::N), Some(1), "N count incorrect");
    assert_eq!(formula.count_of_element(Element::O), Some(8), "O count incorrect");

    Ok(())
}
