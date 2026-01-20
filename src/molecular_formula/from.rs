//! Submodule implementing several `From` traits for the `MolecularFormula`
//! struct

use elements_rs::{Element, Isotope};

use super::MolecularFormula;
use crate::{
    Ion,
    token::{Complex, greek_letters::GreekLetter},
};

impl From<Element> for MolecularFormula {
    fn from(element: Element) -> Self {
        MolecularFormula::Element(element)
    }
}

impl From<Element> for Box<MolecularFormula> {
    fn from(element: Element) -> Self {
        MolecularFormula::Element(element).into()
    }
}

impl From<Isotope> for MolecularFormula {
    fn from(isotope: Isotope) -> Self {
        MolecularFormula::Isotope(isotope)
    }
}

impl From<Isotope> for Box<MolecularFormula> {
    fn from(isotope: Isotope) -> Self {
        MolecularFormula::Isotope(isotope).into()
    }
}

impl From<GreekLetter> for MolecularFormula {
    fn from(greek_letter: GreekLetter) -> Self {
        MolecularFormula::Greek(greek_letter)
    }
}

impl From<Ion<Box<MolecularFormula>>> for MolecularFormula {
    fn from(ion: Ion<Box<MolecularFormula>>) -> Self {
        MolecularFormula::Ion(ion)
    }
}

impl From<Ion<MolecularFormula>> for MolecularFormula {
    fn from(ion: Ion<MolecularFormula>) -> Self {
        MolecularFormula::Ion(ion.into())
    }
}

impl From<Ion<Element>> for MolecularFormula {
    fn from(ion: Ion<Element>) -> Self {
        MolecularFormula::Ion(ion.into())
    }
}

impl From<Ion<Element>> for Box<MolecularFormula> {
    fn from(ion: Ion<Element>) -> Self {
        Box::new(ion.into())
    }
}

impl From<Vec<MolecularFormula>> for MolecularFormula {
    fn from(sequence: Vec<MolecularFormula>) -> Self {
        MolecularFormula::Sequence(sequence)
    }
}

impl From<Vec<MolecularFormula>> for Box<MolecularFormula> {
    fn from(sequence: Vec<MolecularFormula>) -> Self {
        Box::new(MolecularFormula::Sequence(sequence))
    }
}

impl From<Complex> for MolecularFormula {
    fn from(complex: Complex) -> Self {
        match complex {
            Complex::Benzyl => {
                MolecularFormula::RepeatingUnit(Box::new(
                    vec![
                        MolecularFormula::Count(Element::C.into(), 7),
                        MolecularFormula::Count(Element::H.into(), 7),
                    ]
                    .into(),
                ))
            }
            Complex::Butyl => {
                MolecularFormula::RepeatingUnit(Box::new(
                    vec![
                        MolecularFormula::Count(Element::C.into(), 4),
                        MolecularFormula::Count(Element::H.into(), 9),
                    ]
                    .into(),
                ))
            }
            Complex::Phenyl => {
                MolecularFormula::RepeatingUnit(Box::new(
                    vec![
                        MolecularFormula::Count(Element::C.into(), 6),
                        MolecularFormula::Count(Element::H.into(), 5),
                    ]
                    .into(),
                ))
            }
            Complex::Cyclohexyl => {
                MolecularFormula::RepeatingUnit(Box::new(
                    vec![
                        MolecularFormula::Count(Element::C.into(), 6),
                        MolecularFormula::Count(Element::H.into(), 11),
                    ]
                    .into(),
                ))
            }
            Complex::Ethyl => {
                MolecularFormula::RepeatingUnit(Box::new(
                    vec![
                        MolecularFormula::Count(Element::C.into(), 2),
                        MolecularFormula::Count(Element::H.into(), 5),
                    ]
                    .into(),
                ))
            }
            Complex::Methyl => {
                MolecularFormula::RepeatingUnit(Box::new(
                    vec![Element::C.into(), MolecularFormula::Count(Element::H.into(), 3)].into(),
                ))
            }
            Complex::Cyclopentadienyl => {
                Ion::from_formula(
                    vec![
                        MolecularFormula::Count(Element::C.into(), 5),
                        MolecularFormula::Count(Element::H.into(), 5),
                    ]
                    .into(),
                    -1,
                )
                .expect("Failed to create Ion")
                .into()
            }
        }
    }
}
