//! Submodule implementing the `NoResidualsTree` for `GenericTree`.

use elements_rs::{ElementVariant, RelativeAtomicMass};

use crate::{
    ChargeLike, CountLike, NoResidualsTree,
    molecular_formula::{ELECTRON_MASS, GenericTree},
};

impl<S: ChargeLike + TryFrom<U>, U: CountLike, T: NoResidualsTree<Signed = S, Unsigned = U>>
    NoResidualsTree for GenericTree<S, U, T>
{
    fn total_charge(&self) -> f64 {
        match self {
            Self::Charge(_, charge) => (*charge).into(),
            Self::Sequence(formulas) => {
                let mut total_charge = 0.0;
                for formula in formulas {
                    total_charge += formula.total_charge();
                }
                total_charge
            }
            Self::Unit(inner, _) | Self::Radical(inner, _) => inner.total_charge(),
            Self::Repeat(inner, repeat) => {
                let inner_charge = inner.total_charge();
                let n: f64 = (*repeat).into();
                inner_charge * n
            }
            Self::Element(_) | Self::Isotope(_) => 0.0,
            Self::Extension(ext) => ext.total_charge(),
        }
    }

    fn isotopologue_mass_with_charge(&self) -> f64 {
        match self {
            Self::Element(element) => element.relative_atomic_mass(),
            Self::Isotope(isotope) => isotope.relative_atomic_mass(),
            Self::Charge(inner, charge) => {
                let mass = inner.isotopologue_mass_with_charge();
                let c: f64 = (*charge).into();
                mass - c * ELECTRON_MASS
            }
            Self::Repeat(inner, count) => {
                let mass = inner.isotopologue_mass_with_charge();
                let n: f64 = (*count).into();
                mass * n
            }
            Self::Sequence(formulas) => {
                let mut total_mass = 0.0;
                for formula in formulas {
                    total_mass += formula.isotopologue_mass_with_charge();
                }
                total_mass
            }
            Self::Unit(inner, _) | Self::Radical(inner, _) => inner.isotopologue_mass_with_charge(),
            Self::Extension(ext) => ext.isotopologue_mass_with_charge(),
        }
    }

    fn isotopologue_mass_without_charge(&self) -> f64 {
        match self {
            Self::Element(element) => element.relative_atomic_mass(),
            Self::Isotope(isotope) => isotope.relative_atomic_mass(),
            Self::Charge(inner, _) | Self::Unit(inner, _) | Self::Radical(inner, _) => {
                inner.isotopologue_mass_without_charge()
            }
            Self::Repeat(inner, count) => {
                let mass = inner.isotopologue_mass_without_charge();
                let n: f64 = (*count).into();
                mass * n
            }
            Self::Sequence(formulas) => {
                let mut total_mass = 0.0;
                for formula in formulas {
                    total_mass += formula.isotopologue_mass_without_charge();
                }
                total_mass
            }
            Self::Extension(ext) => ext.isotopologue_mass_without_charge(), // Fix for last arm
        }
    }

    fn molar_mass(&self) -> f64 {
        match self {
            Self::Element(element) => element.standard_atomic_weight(),
            Self::Isotope(isotope) => isotope.element().standard_atomic_weight(),
            Self::Charge(inner, charge) => {
                let mass = inner.molar_mass();
                let c: f64 = (*charge).into();
                mass - c * ELECTRON_MASS
            }
            Self::Repeat(inner, count) => {
                let mass = inner.molar_mass();
                let n: f64 = (*count).into();
                mass * n
            }
            Self::Sequence(formulas) => {
                let mut total_mass = 0.0;
                for formula in formulas {
                    total_mass += formula.molar_mass();
                }
                total_mass
            }
            Self::Unit(inner, _) | Self::Radical(inner, _) => inner.molar_mass(),
            Self::Extension(ext) => ext.molar_mass(),
        }
    }
}
