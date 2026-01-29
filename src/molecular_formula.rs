//! Properties that can be computed from molecular formulas.

use core::{fmt::Display, iter::repeat_n};

use crate::{ChargeLike, ChargedMolecularTree, CountLike, MolecularTree, prelude::Element};

mod chemical_formula;
mod inchi_formula;
mod mineral_formula;
mod residual_formula;
pub use chemical_formula::*;
use elements_rs::Isotope;
pub use inchi_formula::*;
pub use mineral_formula::*;
use num_traits::{CheckedAdd, CheckedMul, ConstZero};
pub use residual_formula::*;

/// Trait defining metadata associated with a molecular formula.
pub trait MolecularFormulaMetadata: Sized {
    /// The count type used in the molecular formula.
    type Count: CountLike;
}

/// Trait for computing various molecular properties.
pub trait MolecularFormula: MolecularFormulaMetadata + Display + From<Element> + Clone {
    /// The tree type used in the molecular formula.
    type Tree: MolecularTree<Self::Count>;

    /// Iterates over the counted mixtures in the molecular formula.
    ///
    /// # Implementation Notes
    ///
    /// Returns an iterator over the mixtures in the formula, each represented
    /// as a tuple of (`Self::Tree`, `Self::Count`), where the first element is
    /// the tree representing the mixture, and the second element is its
    /// count.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use molecular_formulas::prelude::*;
    ///
    /// let formula: ChemicalFormula = ChemicalFormula::from_str("NaCl.2H2O").unwrap();
    /// let mixtures: Vec<_> = formula.counted_mixtures().collect();
    /// assert_eq!(mixtures.len(), 2);
    /// let (count, tree) = &mixtures[1];
    /// assert_eq!(*count, 2);
    /// ```
    fn counted_mixtures(&self) -> impl Iterator<Item = (Self::Count, &Self::Tree)>;

    /// Iterates mutably over the counted mixtures in the molecular formula.
    fn counted_mixtures_mut(&mut self) -> impl Iterator<Item = (Self::Count, &mut Self::Tree)>;

    /// Into iterates over the counted mixtures in the molecular formula.
    ///
    /// # Implementation Notes
    ///
    /// Returns an iterator over the mixtures in the formula, each represented
    /// as a tuple of (`Self::Tree`, `Self::Count`), where the first element is
    /// the tree representing the mixture, and the second element is its
    /// count.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use molecular_formulas::prelude::*;
    ///
    /// let formula: ChemicalFormula = ChemicalFormula::from_str("NaCl.2H2O").unwrap();
    ///
    /// let mixtures: Vec<_> = formula.clone().into_counted_mixtures().collect();
    /// assert_eq!(mixtures.len(), 2);
    /// let (count, tree) = &mixtures[1];
    /// assert_eq!(*count, 2);
    /// ```
    fn into_counted_mixtures(self) -> impl Iterator<Item = (Self::Count, Self::Tree)>;

    /// Iterates over the mixtures in the molecular formula, repeating them
    /// according to their counts.
    fn mixtures(&self) -> impl Iterator<Item = &Self::Tree> {
        self.counted_mixtures().flat_map(|(count, tree)| {
            repeat_n(tree, count.try_into().ok().expect("Count type cannot be converted to usize - do you have an extremely large mixture count?"))
        })
    }

    /// Returns the number of mixtures in the molecular formula.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use molecular_formulas::prelude::*;
    ///
    /// let formula: ChemicalFormula = ChemicalFormula::from_str("NaCl.2H2O").unwrap();
    /// assert_eq!(formula.number_of_mixtures(), 3);
    /// ```
    fn number_of_mixtures(&self) -> usize {
        self.counted_mixtures()
            .map(|(count, _)| {
                let count: usize =
                    count.try_into().ok().expect("Count type cannot be converted to usize - do you have an extremely large mixture count?");
                count
            })
            .sum()
    }

    /// Returns the number of elements present in the molecular formula,
    /// counting repeating units according to their counts.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use molecular_formulas::prelude::*;
    ///
    /// let formula: ChemicalFormula = ChemicalFormula::from_str("C6H12O6").unwrap();
    /// assert_eq!(formula.number_of_elements(), 24);
    /// ```
    fn number_of_elements(&self) -> usize {
        self.counted_mixtures()
            .map(|(count, tree)| {
                let count: usize =
                    count.try_into().ok().expect("Count type cannot be converted to usize - do you have an extremely large mixture count?");
                count * tree.number_of_elements()
            })
            .sum()
    }

    /// Returns the number of non-hydrogen elements present in the molecular
    /// formula, counting repeating units according to their counts.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use molecular_formulas::prelude::*;
    ///
    /// let formula: ChemicalFormula = ChemicalFormula::from_str("C6H12O6").unwrap();
    ///
    /// assert_eq!(formula.number_of_non_hydrogens(), 12);
    /// ```
    fn number_of_non_hydrogens(&self) -> usize
    where
        usize: From<Self::Count>,
    {
        self.number_of_elements() - self.count_of_element::<usize>(Element::H).unwrap_or(0)
    }

    /// Iterates over the elements in the molecular formula.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use elements_rs::Element;
    /// use molecular_formulas::prelude::*;
    ///
    /// let formula: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();
    /// let elements: Vec<_> = formula.elements().collect();
    /// assert_eq!(elements, vec![Element::H, Element::H, Element::O]);
    /// ```
    fn elements(&self) -> impl Iterator<Item = Element> {
        self.counted_mixtures().flat_map(|(count, tree)| {
            repeat_n(tree, count.try_into().ok().expect("Count type cannot be converted to usize - do you have an extremely large mixture count?")).flat_map(MolecularTree::elements)
        })
    }

    /// Iterates over the elements in the molecular formula, ignoring hydrogens.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use elements_rs::Element;
    /// use molecular_formulas::prelude::*;
    ///
    /// let formula: ChemicalFormula = ChemicalFormula::from_str("CH4").unwrap();
    /// let elements: Vec<_> = formula.non_hydrogens().collect();
    /// assert_eq!(elements, vec![Element::C]);
    /// ```
    fn non_hydrogens(&self) -> impl Iterator<Item = Element> {
        self.counted_mixtures().flat_map(|(count, tree)| {
            repeat_n(tree, count.try_into().ok().expect("Count type cannot be converted to usize - do you have an extremely large mixture count?")).flat_map(MolecularTree::non_hydrogens)
        })
    }

    /// Returns whether the molecular formula contains any elements.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use molecular_formulas::prelude::*;
    ///
    /// let formula: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();
    /// assert!(formula.contains_elements());
    /// ```
    fn contains_elements(&self) -> bool {
        self.counted_mixtures().any(|(_, tree)| tree.contains_elements())
    }

    /// Returns whether the molecular formula contains any non-hydrogen
    /// elements.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use molecular_formulas::prelude::*;
    ///
    /// let formula: ChemicalFormula = ChemicalFormula::from_str("CH4").unwrap();
    /// assert!(formula.contains_non_hydrogens());
    /// let h2: ChemicalFormula = ChemicalFormula::from_str("H2").unwrap();
    /// assert!(!h2.contains_non_hydrogens());
    /// ```
    fn contains_non_hydrogens(&self) -> bool {
        self.counted_mixtures().any(|(_, tree)| tree.contains_non_hydrogens())
    }

    /// Returns whether the molecular formula contains the provided element.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use elements_rs::Element;
    /// use molecular_formulas::prelude::*;
    ///
    /// let formula: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();
    /// assert!(formula.contains_element(Element::O));
    /// assert!(!formula.contains_element(Element::C));
    /// ```
    fn contains_element(&self, element: Element) -> bool {
        self.counted_mixtures().any(|(_, tree)| tree.contains_element(element))
    }

    /// Returns whether the molecular formula contains any isotopes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use molecular_formulas::prelude::*;
    ///
    /// let formula: ChemicalFormula = ChemicalFormula::from_str("[13C]H4").unwrap();
    /// assert!(formula.contains_isotopes());
    /// let formula_no_iso: ChemicalFormula = ChemicalFormula::from_str("CH4").unwrap();
    /// assert!(!formula_no_iso.contains_isotopes());
    /// ```
    fn contains_isotopes(&self) -> bool {
        self.counted_mixtures().any(|(_, tree)| tree.contains_isotopes())
    }

    /// Returns whether the molecular formula contains the provided isotope.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use elements_rs::{Element, Isotope};
    /// use molecular_formulas::prelude::*;
    ///
    /// let formula: ChemicalFormula = ChemicalFormula::from_str("[13C]H4").unwrap();
    /// assert!(formula.contains_isotope(Isotope::try_from((Element::C, 13u16)).unwrap()));
    /// ```
    fn contains_isotope(&self, isotope: Isotope) -> bool {
        self.counted_mixtures().any(|(_, tree)| tree.contains_isotope(isotope))
    }

    /// Returns the number of elements of a specific type in the molecular
    /// formula.
    ///
    /// Returns None if the provided data type C cannot represent the count.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use elements_rs::Element;
    /// use molecular_formulas::prelude::*;
    ///
    /// let formula: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();
    /// assert_eq!(formula.count_of_element::<u32>(Element::H), Some(2));
    /// assert_eq!(formula.count_of_element::<u32>(Element::O), Some(1));
    /// ```
    fn count_of_element<C>(&self, element: Element) -> Option<C>
    where
        C: From<Self::Count> + CheckedAdd + CheckedMul + ConstZero,
    {
        let mut total: C = C::zero();
        for (count, tree) in self.counted_mixtures() {
            total = total.checked_add(
                &C::from(count).checked_mul(&C::from(tree.count_of_element(element)?))?,
            )?;
        }
        Some(total)
    }

    /// Returns the number of isotopes of a specific type in the molecular
    /// formula.
    ///
    /// Returns None if the provided data type C cannot represent the count.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use elements_rs::{Element, Isotope};
    /// use molecular_formulas::prelude::*;
    ///
    /// let formula: ChemicalFormula = ChemicalFormula::from_str("[13C]2H4").unwrap();
    /// assert_eq!(
    ///     formula.count_of_isotope::<u32>(Isotope::try_from((Element::C, 13u16)).unwrap()),
    ///     Some(2)
    /// );
    /// ```
    fn count_of_isotope<C>(&self, isotope: Isotope) -> Option<C>
    where
        C: From<Self::Count> + CheckedAdd + CheckedMul + ConstZero,
    {
        let mut total: C = C::zero();
        for (count, tree) in self.counted_mixtures() {
            total = total.checked_add(
                &C::from(count).checked_mul(&C::from(tree.count_of_isotope(isotope)?))?,
            )?;
        }
        Some(total)
    }

    /// Returns the isotopologue mass of the molecular formula without
    /// considering any charge.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use molecular_formulas::prelude::*;
    ///
    /// let formula: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();
    /// let mass = formula.isotopologue_mass();
    /// assert!(mass > 18.0 && mass < 18.1); // atomic mass of H ~ 1.008, O ~ 15.999
    /// ```
    fn isotopologue_mass(&self) -> f64 {
        let mut total_mass = 0.0;
        for (count, tree) in self.counted_mixtures() {
            let count: f64 = count.into();
            total_mass += count * tree.isotopologue_mass();
        }
        total_mass
    }

    /// Returns whether the molecular formula is a noble gas compound.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use molecular_formulas::prelude::*;
    ///
    /// let formula: ChemicalFormula = ChemicalFormula::from_str("He").unwrap();
    /// assert!(formula.is_noble_gas_compound());
    /// let water: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();
    /// assert!(!water.is_noble_gas_compound());
    /// ```
    fn is_noble_gas_compound(&self) -> bool {
        self.counted_mixtures().all(|(_, tree)| tree.is_noble_gas_compound())
    }

    /// Returns whether the molecular formula is sorted according to Hill
    /// system.
    ///
    /// If the formula contains carbon atoms, they must be listed first,
    /// followed by hydrogen atoms, and then all other elements in
    /// alphabetical order. If the formula does not contain carbon atoms,
    /// all elements must be listed in alphabetical order, including hydrogen.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use molecular_formulas::prelude::*;
    ///
    /// let formula1: ChemicalFormula = ChemicalFormula::from_str("C6H12O6").unwrap();
    /// assert!(formula1.is_hill_sorted(), "Formula `C6H12O6` should be Hill sorted");
    /// let formula2: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();
    /// assert!(formula2.is_hill_sorted(), "Formula `H2O` should be Hill sorted");
    /// let formula3: ChemicalFormula = ChemicalFormula::from_str("C2H5OH").unwrap();
    /// assert!(!formula3.is_hill_sorted(), "Formula `C2H5OH` should not be Hill sorted");
    /// let formula4: ChemicalFormula = ChemicalFormula::from_str("NaCl").unwrap();
    /// assert!(!formula4.is_hill_sorted(), "Formula `NaCl` should not be Hill sorted");
    /// let formula5: ChemicalFormula = ChemicalFormula::from_str("C2H6O").unwrap();
    /// assert!(formula5.is_hill_sorted(), "Formula `C2H6O` should be Hill sorted");
    /// let formula6: ChemicalFormula = ChemicalFormula::from_str("C6H8O6").unwrap();
    /// assert!(formula6.is_hill_sorted(), "Formula `C6H8O6` should be Hill sorted");
    /// let formula7: ChemicalFormula = ChemicalFormula::from_str("C16H25NS").unwrap();
    /// assert!(formula7.is_hill_sorted(), "Formula `C16H25NS` should be Hill sorted");
    /// let formula8: ChemicalFormula = ChemicalFormula::from_str("C28H23ClO7").unwrap();
    /// assert!(formula8.is_hill_sorted(), "Formula `{formula8}` should be Hill sorted");
    /// let formula9: ChemicalFormula = ChemicalFormula::from_str("CBr2F2").unwrap();
    /// assert!(formula9.is_hill_sorted(), "Formula `CBr2F2` should be Hill sorted");
    /// let formula10: ChemicalFormula = ChemicalFormula::from_str("C").unwrap();
    /// assert!(formula10.is_hill_sorted(), "Formula `C` should be Hill sorted");
    /// let formula11: ChemicalFormula = ChemicalFormula::from_str("H").unwrap();
    /// assert!(formula11.is_hill_sorted(), "Formula `H` should be Hill sorted");
    /// let formula12: ChemicalFormula = ChemicalFormula::from_str("C2").unwrap();
    /// assert!(formula12.is_hill_sorted(), "Formula `C2` should be Hill sorted");
    /// let mixture: ChemicalFormula = ChemicalFormula::from_str("C32H34N4O4.Ni").unwrap();
    /// assert!(mixture.is_hill_sorted(), "Mixture `C32H34N4O4.Ni` should be Hill sorted");
    /// let mixture2: ChemicalFormula = ChemicalFormula::from_str("ClH.Na").unwrap();
    /// assert!(mixture2.is_hill_sorted(), "Mixture `ClH.Na` should be Hill sorted");
    /// let mixture3: ChemicalFormula = ChemicalFormula::from_str("C20H18F3N4O8P.Na").unwrap();
    /// assert!(mixture3.is_hill_sorted(), "Mixture `{mixture3}` should be Hill sorted");
    /// let unsorted_mixture1: ChemicalFormula = ChemicalFormula::from_str("C32H34O4N4.Ni").unwrap();
    /// assert!(
    ///     !unsorted_mixture1.is_hill_sorted(),
    ///     "Mixture `C32H34O4N4.Ni` should not be Hill sorted"
    /// );
    /// let unsorted_mixture2: ChemicalFormula = ChemicalFormula::from_str("HCl.Na").unwrap();
    /// assert!(!unsorted_mixture2.is_hill_sorted(), "Mixture `HCl.Na` should not be Hill sorted");
    /// let unsorted_mixture3: ChemicalFormula =
    ///     ChemicalFormula::from_str("C15H18O7.C15O6H16").unwrap();
    /// assert!(
    ///     !unsorted_mixture3.is_hill_sorted(),
    ///     "Mixture `C15H18O7.C15O6H16` should not be Hill sorted"
    /// );
    /// let unsorted_formula: ChemicalFormula = ChemicalFormula::from_str("CH2SCl2O3").unwrap();
    /// assert!(!unsorted_formula.is_hill_sorted(), "Formula `CH2SCl2O3` should not be Hill sorted");
    /// let unsorted_formula2: ChemicalFormula = ChemicalFormula::from_str("C6H18NaNSi4").unwrap();
    /// assert!(!unsorted_formula2.is_hill_sorted(), "Formula `C6H18NaNSi4` should not be Hill sorted");
    /// ```
    #[must_use]
    fn is_hill_sorted(&self) -> bool {
        self.counted_mixtures().all(|(_, tree)| tree.is_hill_sorted())
    }

    /// Returns the element at the specified index in the molecular formula,
    /// counting repeating units according to their counts.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use elements_rs::Element;
    /// use molecular_formulas::prelude::*;
    ///
    /// let formula: ChemicalFormula = ChemicalFormula::try_from("H2O")?;
    /// assert_eq!(formula.get_element(0), Some(Element::H));
    /// assert_eq!(formula.get_element(1), Some(Element::H));
    /// assert_eq!(formula.get_element(2), Some(Element::O));
    /// assert_eq!(formula.get_element(3), None);
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    fn get_element(&self, index: usize) -> Option<Element> {
        self.elements().enumerate().find_map(
            |(i, element)| {
                if i == index { Some(element) } else { None }
            },
        )
    }

    /// Returns the element at the specified index in the molecular formula,
    /// not counting repeating units according to their counts, and ignoring
    /// any hydrogens (used typically for InchI parsing).
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use elements_rs::Element;
    /// use molecular_formulas::prelude::*;
    ///
    /// let formula: ChemicalFormula = ChemicalFormula::try_from("H2O")?;
    /// assert_eq!(formula.get_non_hydrogen(0), Some(Element::O));
    /// assert_eq!(formula.get_non_hydrogen(1), None);
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    fn get_non_hydrogen(&self, index: usize) -> Option<Element> {
        self.non_hydrogens()
            .enumerate()
            .find_map(|(i, element)| if i == index { Some(element) } else { None })
    }

    /// Returns a version of the molecular formula with all isotopes converted
    /// to their elemental forms.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use molecular_formulas::prelude::*;
    ///
    /// let formula: ChemicalFormula = ChemicalFormula::from_str("[13C]H4").unwrap();
    /// let elemental = formula.isotopic_normalization();
    /// assert_eq!(elemental.to_string(), "CH₄");
    ///
    /// let mixed_formula: ChemicalFormula = ChemicalFormula::from_str("[13C]T4.O[18O]").unwrap();
    /// let mixed_elemental = mixed_formula.isotopic_normalization();
    /// assert_eq!(mixed_elemental.to_string(), "CH₄.OO");
    /// ```
    #[must_use]
    fn isotopic_normalization(&self) -> Self {
        let mut formula = self.clone();
        for (_, tree) in formula.counted_mixtures_mut() {
            *tree = tree.isotopic_normalization();
        }
        formula
    }
}

/// A molecular formula that can hold a charge.
pub trait ChargedMolecularFormulaMetadata: MolecularFormulaMetadata {
    /// The charge type used in the molecular formula.
    type Charge: ChargeLike + TryFrom<Self::Count>;
}

/// Trait for computing various charged molecular properties.
pub trait ChargedMolecularFormula:
    MolecularFormula<Tree: ChargedMolecularTree<Self::Count, Self::Charge>>
    + ChargedMolecularFormulaMetadata
{
    /// Returns the overall charge of the molecular formula.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use molecular_formulas::prelude::*;
    ///
    /// let formula: ChemicalFormula = ChemicalFormula::from_str("H2O+2").unwrap();
    /// assert_eq!(formula.charge(), 2.0);
    /// let neutral: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();
    /// assert_eq!(neutral.charge(), 0.0);
    /// ```
    fn charge(&self) -> f64 {
        self.counted_mixtures()
            .map(|(count, tree)| {
                let count: f64 = count.into();
                count * tree.charge()
            })
            .sum()
    }

    /// Returns the isotopologue mass with charge considered.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use molecular_formulas::prelude::*;
    ///
    /// let neutral: ChemicalFormula = ChemicalFormula::from_str("H").unwrap();
    /// let cation: ChemicalFormula = ChemicalFormula::from_str("H+").unwrap();
    ///
    /// // Mass of H+ should be less than neutral H (electrons have mass)
    /// assert!(cation.isotopologue_mass_with_charge() < neutral.isotopologue_mass_with_charge());
    /// ```
    fn isotopologue_mass_with_charge(&self) -> f64 {
        self.counted_mixtures()
            .map(|(count, tree)| {
                let count: f64 = count.into();
                count * tree.isotopologue_mass_with_charge()
            })
            .sum()
    }

    /// Returns the isotopologue mass over charge ratio.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use molecular_formulas::prelude::*;
    ///
    /// let formula: ChemicalFormula = ChemicalFormula::from_str("H2O+2").unwrap();
    /// let mz = formula.isotopologue_mass_over_charge();
    /// // Mass ~18, charge 2, so m/z ~9
    /// assert!(mz > 9.0 && mz < 9.1);
    /// ```
    fn isotopologue_mass_over_charge(&self) -> f64 {
        self.isotopologue_mass_with_charge() / self.charge()
    }

    /// Returns the molar mass.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use molecular_formulas::prelude::*;
    ///
    /// let formula: ChemicalFormula = ChemicalFormula::from_str("H2O").unwrap();
    /// let molar_mass = formula.molar_mass();
    /// // Molar mass of water is approx 18.015 g/mol
    /// assert!(molar_mass > 18.0 && molar_mass < 18.02);
    /// ```
    fn molar_mass(&self) -> f64 {
        self.counted_mixtures()
            .map(|(count, tree)| {
                let count: f64 = count.into();
                count * tree.molar_mass()
            })
            .sum()
    }
}

impl<M> ChargedMolecularFormula for M where
    M: MolecularFormula<Tree: ChargedMolecularTree<M::Count, M::Charge>>
        + ChargedMolecularFormulaMetadata
{
}
