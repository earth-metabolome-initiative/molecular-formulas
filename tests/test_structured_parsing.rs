//! Submodule for testing the parsing of molecular formulas.

use std::str::FromStr;

use elements_rs::{Element, Isotope};
use molecular_formulas::{Bracket, DefaultTree, InstantiableTree, MolecularFormula};

fn test_parse<M: Into<MolecularFormula>>(formula: &str, expected: M, simmetric: Option<&str>) {
    let expected = expected.into();
    let simmetric = simmetric.unwrap_or(formula);
    let parsed_formula = MolecularFormula::from_str(formula).unwrap_or_else(|err| {
        panic!("Failed to parse formula: `{simmetric}` from `{formula}`, error: {err:?}")
    });
    assert_eq!(
        parsed_formula, expected,
        "Failed to parse formula `{parsed_formula}` to the expected `{expected}` output `{formula:#?}`, got `{parsed_formula:#?}`"
    );
    assert_eq!(
        simmetric,
        &format!("{parsed_formula}"),
        "Failed to serialize formula: {formula}, got `{parsed_formula}`"
    );
}

#[test]
fn test_h2o() {
    test_parse(
        "H2O",
        DefaultTree::Sequence(vec![
            DefaultTree::Repeat(Box::new(Element::H.into()), 2),
            Element::O.into(),
        ]),
        Some("H₂O"),
    );
}

#[test]
fn test_nacl() {
    test_parse("NaCl", DefaultTree::Sequence(vec![Element::Na.into(), Element::Cl.into()]), None);
}

#[test]
fn test_mixture1() {
    let p1 = DefaultTree::element(Element::C).charge(4).unwrap();
    let p2 = DefaultTree::element(Element::H).repeat(2).unwrap();
    let expected = MolecularFormula::from(p1).mix(p2.into()).unwrap();

    test_parse("C+4.H2", expected, Some("C⁴⁺.H₂"));
}

#[test]
fn test_c12h22o11() {
    test_parse(
        "C12H22O11",
        DefaultTree::Sequence(vec![
            DefaultTree::Repeat(Box::new(Element::C.into()), 12),
            DefaultTree::Repeat(Box::new(Element::H.into()), 22),
            DefaultTree::Repeat(Box::new(Element::O.into()), 11),
        ]),
        Some("C₁₂H₂₂O₁₁"),
    );
}

#[test]
fn test_ch4_plus_3() {
    test_parse(
        "CH4+3",
        DefaultTree::Charge(
            Box::new(DefaultTree::Sequence(vec![
                Element::C.into(),
                DefaultTree::Repeat(Box::new(Element::H.into()), 4),
            ])),
            3,
        ),
        Some("CH₄³⁺"),
    );
}

#[test]
fn test_ion_h() {
    test_parse("H+", DefaultTree::Charge(Box::new(Element::H.into()), 1), Some("H⁺"));
    test_parse("H-", DefaultTree::Charge(Box::new(Element::H.into()), -1), Some("H⁻"));
}

#[test]
fn test_molecular_hydrogen() {
    test_parse("H2", DefaultTree::Repeat(Box::new(Element::H.into()), 2), Some("H₂"));
}

#[test]
fn test_hydrogen_molecular_ion() {
    test_parse(
        "H2+",
        DefaultTree::Charge(Box::new(DefaultTree::Repeat(Box::new(Element::H.into()), 2)), 1),
        Some("H₂⁺"),
    );
}

#[test]
fn test_triatomic_hidrogen() {
    test_parse("H3", DefaultTree::Repeat(Box::new(Element::H.into()), 3), Some("H₃"));
}

#[test]
fn test_formula_including_isotopes() {
    let c12 = Isotope::try_from((Element::C, 12u16)).unwrap();
    let h1 = Isotope::try_from((Element::H, 1u16)).unwrap();
    let pd106 = Isotope::try_from((Element::Pd, 106u16)).unwrap();
    let cl35 = Isotope::try_from((Element::Cl, 35u16)).unwrap();

    test_parse(
        "¹²C18¹H18¹⁰⁶Pd2³⁵Cl2",
        DefaultTree::Sequence(vec![
            DefaultTree::Repeat(c12.into(), 18),
            DefaultTree::Repeat(h1.into(), 18),
            DefaultTree::Repeat(pd106.into(), 2),
            DefaultTree::Repeat(cl35.into(), 2),
        ]),
        Some("[¹²C]₁₈[¹H]₁₈[¹⁰⁶Pd]₂[³⁵Cl]₂"),
    );
}

#[test]
fn test_irregular_ion_position1() {
    test_parse(
        "C+4H4",
        DefaultTree::Sequence(vec![
            DefaultTree::Charge(Box::new(Element::C.into()), 4),
            DefaultTree::Repeat(Box::new(Element::H.into()), 4),
        ]),
        Some("C⁴⁺H₄"),
    );
}

#[test]
fn test_irregular_ion_position2() {
    test_parse(
        "C²⁺H4",
        DefaultTree::Sequence(vec![
            DefaultTree::Charge(Element::C.into(), 2),
            DefaultTree::Repeat(Element::H.into(), 4),
        ]),
        Some("C²⁺H₄"),
    );
}

#[test]
fn test_irregular_ion_position3() {
    test_parse(
        "C²⁻H4",
        DefaultTree::Sequence(vec![
            DefaultTree::Charge(Element::C.into(), -2),
            DefaultTree::Repeat(Element::H.into(), 4),
        ]),
        Some("C²⁻H₄"),
    );
}

#[test]
fn test_ion1() {
    test_parse("C²⁻", DefaultTree::Charge(Box::new(Element::C.into()), -2), Some("C²⁻"));
}

#[test]
fn test_ion2() {
    test_parse("C²⁺", DefaultTree::Charge(Box::new(Element::C.into()), 2), Some("C²⁺"));
}

#[test]
fn test_methanion() {
    test_parse(
        "CH5+",
        DefaultTree::Charge(
            Box::new(DefaultTree::Sequence(vec![
                Element::C.into(),
                DefaultTree::Repeat(Box::new(Element::H.into()), 5),
            ])),
            1,
        ),
        Some("CH₅⁺"),
    );
}

#[test]
fn test_methane_cation() {
    test_parse(
        "CH4+",
        DefaultTree::Charge(
            Box::new(DefaultTree::Sequence(vec![
                Element::C.into(),
                DefaultTree::Repeat(Box::new(Element::H.into()), 4),
            ])),
            1,
        ),
        Some("CH₄⁺"),
    );
}

#[test]
fn test_h2so4() {
    test_parse(
        "H2SO4",
        DefaultTree::Sequence(vec![
            DefaultTree::Repeat(Box::new(Element::H.into()), 2),
            Element::S.into(),
            DefaultTree::Repeat(Box::new(Element::O.into()), 4),
        ]),
        Some("H₂SO₄"),
    );
}

#[test]
fn test_large_compound1() {
    let mgso4 = DefaultTree::Sequence(vec![
        Element::Mg.into(),
        Element::S.into(),
        DefaultTree::Repeat(Box::new(Element::O.into()), 4),
    ]);
    let h2o = DefaultTree::Sequence(vec![
        DefaultTree::Repeat(Box::new(Element::H.into()), 2),
        Element::O.into(),
    ]);

    let expected = MolecularFormula::from(mgso4).mix(h2o.into()).unwrap();

    test_parse("MgSO4.H2O", expected, Some("MgSO₄.H₂O"));

    let part1_inner = DefaultTree::Unit(
        Box::new(DefaultTree::Sequence(vec![
            DefaultTree::Repeat(Box::new(Element::C.into()), 17),
            DefaultTree::Repeat(Box::new(Element::H.into()), 23),
            Element::N.into(),
            DefaultTree::Repeat(Box::new(Element::O.into()), 3),
        ])),
        Bracket::Round,
    );

    let part1_mf = MolecularFormula::from(part1_inner.clone());
    let part1_mf_2 = part1_mf.clone().mix(part1_mf).unwrap();

    let h2o_val = DefaultTree::Sequence(vec![
        DefaultTree::Repeat(Box::new(Element::H.into()), 2),
        Element::O.into(),
    ]);
    let h2o_mf = MolecularFormula::from(h2o_val);

    let h2so4_val = DefaultTree::Sequence(vec![
        DefaultTree::Repeat(Box::new(Element::H.into()), 2),
        Element::S.into(),
        DefaultTree::Repeat(Box::new(Element::O.into()), 4),
    ]);
    let h2so4_mf = MolecularFormula::from(h2so4_val);

    let expected_big = part1_mf_2.mix(h2o_mf).unwrap().mix(h2so4_mf).unwrap();

    test_parse("2(C17H23NO3).H2O.H2SO4", expected_big, Some("2(C₁₇H₂₃NO₃).H₂O.H₂SO₄"));
}

#[test]
fn test_atropine() {
    let inner = DefaultTree::Sequence(vec![
        DefaultTree::Repeat(Box::new(Element::C.into()), 17),
        DefaultTree::Repeat(Box::new(Element::H.into()), 23),
        Element::N.into(),
        DefaultTree::Repeat(Box::new(Element::O.into()), 3),
    ]);

    test_parse("C17H23NO3", inner.clone(), Some("C₁₇H₂₃NO₃"));

    test_parse(
        "(C17H23NO3)",
        DefaultTree::Unit(Box::new(inner.clone()), Bracket::Round),
        Some("(C₁₇H₂₃NO₃)"),
    );

    {
        let unit = DefaultTree::Unit(Box::new(inner.clone()), Bracket::Round);
        let mf = MolecularFormula::from(unit.clone()).mix(MolecularFormula::from(unit)).unwrap();

        test_parse("2(C17H23NO3)", mf, Some("2(C₁₇H₂₃NO₃)"));
    }
}

#[test]
fn test_hexaamminecobalt_iii_chloride() {
    test_parse(
        "[Co(NH3)6]+3(Cl−)3",
        DefaultTree::Sequence(vec![
            DefaultTree::Charge(
                Box::new(DefaultTree::Unit(
                    Box::new(DefaultTree::Sequence(vec![
                        Element::Co.into(),
                        DefaultTree::Repeat(
                            Box::new(DefaultTree::Unit(
                                Box::new(DefaultTree::Sequence(vec![
                                    Element::N.into(),
                                    DefaultTree::Repeat(Box::new(Element::H.into()), 3),
                                ])),
                                Bracket::Round,
                            )),
                            6,
                        ),
                    ])),
                    Bracket::Square,
                )),
                3,
            ),
            DefaultTree::Repeat(
                Box::new(DefaultTree::Unit(
                    Box::new(DefaultTree::Charge(Box::new(Element::Cl.into()), -1)),
                    Bracket::Round,
                )),
                3,
            ),
        ]),
        Some("[Co(NH₃)₆]³⁺(Cl⁻)₃"),
    );
}
