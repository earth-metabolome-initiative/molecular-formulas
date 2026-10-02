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
    assert_eq!(formula.count_of_element(Element::Zn), Ok(1), "Zn count incorrect");
    assert_eq!(formula.count_of_element(Element::Cl), Ok(2), "Cl count incorrect");
    assert_eq!(formula.count_of_element(Element::C), Ok(4), "C count incorrect");
    assert_eq!(formula.count_of_element(Element::H), Ok(12), "H count incorrect");
    assert_eq!(formula.count_of_element(Element::O), Ok(2), "O count incorrect");

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

    assert_eq!(formula.count_of_element(Element::Sn), Ok(1), "Sn count incorrect");
    assert_eq!(formula.count_of_element(Element::Cl), Ok(4), "Cl count incorrect");
    assert_eq!(formula.count_of_element(Element::C), Ok(8), "C count incorrect");
    assert_eq!(formula.count_of_element(Element::H), Ok(20), "H count incorrect");
    assert_eq!(formula.count_of_element(Element::O), Ok(2), "O count incorrect");

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

    assert_eq!(formula.count_of_element(Element::C), Ok(27), "C count incorrect");
    assert_eq!(formula.count_of_element(Element::H), Ok(41), "H count incorrect");
    assert_eq!(formula.count_of_element(Element::N), Ok(1), "N count incorrect");
    assert_eq!(formula.count_of_element(Element::O), Ok(8), "O count incorrect");

    Ok(())
}

#[test]
fn merge_mixtures_flattens_hydrate_counts() -> Result<(), Box<dyn std::error::Error>> {
    use elements_rs::Element;

    let formula: ChemicalFormula<u32, i32> = "CuSO4.5H2O".parse()?;
    let merged = formula.merge_mixtures()?;

    assert_eq!(merged.number_of_mixtures(), 1);
    assert_eq!(merged.to_string(), "CuH₁₀O₉S");
    assert_eq!(merged.count_of_element::<u32>(Element::Cu), Ok(1));
    assert_eq!(merged.count_of_element::<u32>(Element::H), Ok(10));
    assert_eq!(merged.count_of_element::<u32>(Element::O), Ok(9));
    assert_eq!(merged.count_of_element::<u32>(Element::S), Ok(1));

    Ok(())
}

#[test]
fn merge_mixtures_preserves_isotope_counts() -> Result<(), Box<dyn std::error::Error>> {
    use elements_rs::isotopes::HydrogenIsotope;

    let c13 = Isotope::try_from((Element::C, 13_u8))?;
    let formula: ChemicalFormula<u32, i32> = "[13C]H4.CH4.D2O".parse()?;
    let merged = formula.merge_mixtures()?;

    assert_eq!(merged.number_of_mixtures(), 1);
    assert_eq!(merged.count_of_element::<u32>(Element::C), Ok(2));
    assert_eq!(merged.count_of_isotope::<u32>(c13), Ok(1));
    assert_eq!(merged.count_of_element::<u32>(Element::H), Ok(10));
    assert_eq!(merged.count_of_isotope::<u32>(HydrogenIsotope::D.into()), Ok(2));
    assert_eq!(merged.count_of_element::<u32>(Element::O), Ok(1));

    Ok(())
}

#[test]
fn merge_mixtures_returns_single_formula_unchanged() -> Result<(), Box<dyn std::error::Error>> {
    let formula: ChemicalFormula<u32, i32> = "Na+".parse()?;
    let merged = formula.merge_mixtures()?;

    assert_eq!(merged.number_of_mixtures(), 1);
    assert_eq!(merged.to_string(), "Na⁺");
    assert_eq!(merged.count_of_element::<u32>(Element::Na), Ok(1));
    assert!((merged.charge() - 1.0).abs() < f64::EPSILON);

    Ok(())
}

#[test]
fn merge_mixtures_detects_count_overflow() -> Result<(), Box<dyn std::error::Error>> {
    use molecular_formulas::errors::{CountError, NumericError};

    let formula: ChemicalFormula<u8, i16> = "250H2O.10H2O".parse()?;

    assert_eq!(formula.merge_mixtures(), Err(CountError::Numeric(NumericError::PositiveOverflow)));

    Ok(())
}

#[test]
fn merge_inchi_mixtures() -> Result<(), Box<dyn std::error::Error>> {
    let formula: InChIFormula<u32> = "2C2H6O.ClNa".parse()?;
    let merged = formula.merge_mixtures()?;

    assert_eq!(merged.number_of_mixtures(), 1);
    assert_eq!(merged.to_string(), "C4H12ClNaO2");
    assert_eq!(merged.count_of_element::<u32>(Element::C), Ok(4));
    assert_eq!(merged.count_of_element::<u32>(Element::H), Ok(12));
    assert_eq!(merged.count_of_element::<u32>(Element::Cl), Ok(1));
    assert_eq!(merged.count_of_element::<u32>(Element::Na), Ok(1));
    assert_eq!(merged.count_of_element::<u32>(Element::O), Ok(2));

    Ok(())
}

#[test]
fn merge_mineral_mixtures_preserves_prefix() -> Result<(), Box<dyn std::error::Error>> {
    let formula: MineralFormula<u32, i32> = "α-SiO2.H2O".parse()?;
    let merged = formula.merge_mixtures()?;

    assert_eq!(merged.number_of_mixtures(), 1);
    assert_eq!(merged.to_string(), "α-H₂O₃Si");
    assert_eq!(merged.count_of_element::<u32>(Element::H), Ok(2));
    assert_eq!(merged.count_of_element::<u32>(Element::O), Ok(3));
    assert_eq!(merged.count_of_element::<u32>(Element::Si), Ok(1));

    Ok(())
}
