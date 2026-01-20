use std::str::FromStr;

use molecular_formulas::MolecularFormula;

#[test]
fn test_me2_expansion() {
    // Me2O -> (CH3)2O -> C2H6O
    // Current expected behavior (suspected wrong): C H6 O
    // Desired behavior: C2 H6 O
    let formula = MolecularFormula::from_str("Me2O").unwrap();
    // Let's print it to see structure
    println!("{:?}", formula);

    // Check element counts
    let c_count = formula.element_count(elements_rs::Element::C);
    let h_count = formula.element_count(elements_rs::Element::H);
    let o_count = formula.element_count(elements_rs::Element::O);

    assert_eq!(c_count, 2, "Carbon count should be 2 for {formula}");
    assert_eq!(h_count, 6, "Hydrogen count should be 6 for {formula}");
    assert_eq!(o_count, 1, "Oxygen count should be 1 for {formula}");
}

#[test]
fn test_ethyl_compound() {
    // Et2O -> (C2H5)2O -> C4H10O
    let formula = MolecularFormula::from_str("Et2O").unwrap();
    assert_eq!(formula.element_count(elements_rs::Element::C), 4);
    assert_eq!(formula.element_count(elements_rs::Element::H), 10);
    assert_eq!(formula.element_count(elements_rs::Element::O), 1);
}

#[test]
fn test_phenyl_compounds() {
    // PhOH -> C6H5OH -> C6H6O
    let phenol = MolecularFormula::from_str("PhOH").unwrap();
    assert_eq!(phenol.element_count(elements_rs::Element::C), 6);
    assert_eq!(phenol.element_count(elements_rs::Element::H), 6);
    assert_eq!(phenol.element_count(elements_rs::Element::O), 1);

    // PhCOOH -> C6H5COOH -> C7H6O2
    let benzoic_acid = MolecularFormula::from_str("PhCOOH").unwrap();
    assert_eq!(benzoic_acid.element_count(elements_rs::Element::C), 7);
    assert_eq!(benzoic_acid.element_count(elements_rs::Element::H), 6);
    assert_eq!(benzoic_acid.element_count(elements_rs::Element::O), 2);
}

#[test]
fn test_benzyl_compounds() {
    // BnBr -> C7H7Br
    let benzyl_bromide = MolecularFormula::from_str("BnBr").unwrap();
    assert_eq!(benzyl_bromide.element_count(elements_rs::Element::C), 7);
    assert_eq!(benzyl_bromide.element_count(elements_rs::Element::H), 7);
    assert_eq!(benzyl_bromide.element_count(elements_rs::Element::Br), 1);
}

#[test]
fn test_cyclohexyl() {
    // CyOH -> C6H11OH -> C6H12O
    let cyclohexanol = MolecularFormula::from_str("CyOH").unwrap();
    assert_eq!(cyclohexanol.element_count(elements_rs::Element::C), 6);
    assert_eq!(cyclohexanol.element_count(elements_rs::Element::H), 12);
    assert_eq!(cyclohexanol.element_count(elements_rs::Element::O), 1);
}

#[test]
fn test_cyclopentadienyl() {
    // Cp2Fe -> (C5H5)2Fe -> C10H10Fe (Ferrocene)
    // Note: Cp is usually C5H5^-1, so Cp2Fe is (C5H5-)2 Fe(0) -> -2 charge in
    // parser logic
    let ferrocene = MolecularFormula::from_str("Cp2Fe").unwrap();
    assert_eq!(ferrocene.element_count(elements_rs::Element::C), 10);
    assert_eq!(ferrocene.element_count(elements_rs::Element::H), 10);
    assert_eq!(ferrocene.element_count(elements_rs::Element::Fe), 1);

    // Check charge: 2 * Cp(-1) + Fe(0) = -2
    assert_eq!(ferrocene.charge().unwrap(), -2);
}
