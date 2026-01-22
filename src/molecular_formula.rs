//! Represents each molecular formula that can be parsed.

use core::fmt::Debug;

use num_traits::CheckedAdd;
mod charge;
mod contains_elements;
mod contains_isotopes;
mod contains_mixtures;
mod contains_residual;
mod display;
mod element_count;
mod from;
mod from_str;
mod trees;
pub use trees::{
    EmptyTree, GenericResidualTree, GenericTree, InstantiableTree, NoResidualsTree, Tree,
};
mod isotopologue_mass;
mod isotopologue_mass_over_charge;
mod molar_mass;
mod noble_gasses;
pub mod parser;
mod serde;
mod try_from;
pub use parser::{
    AllowedCharacter, AllowedCharacterError, Bracket, CharacterMarker, ChargeLike, Complex,
    CountLike, Digit, GreekLetter, ParseError, ParserError, Radical, Residual, SubToken,
    SubTokenError, SuperscriptMinus, SuperscriptPlus, Terminator, Token, TokenError,
};

const ELECTRON_MASS: f64 = 5.485_799_090_65e-4;

use crate::errors::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Represents the side in a molecular formula.
pub enum Side {
    /// The left side.
    Left,
    /// The right side.
    Right,
}

/// Type alias for the smallest molecular formula tree.
pub type SmallestTree = GenericTree<i8, u8, EmptyTree<i8, u8>>;

/// Type alias for the default molecular formula.
pub type DefaultTree = GenericTree<i16, u16, EmptyTree<i16, u16>>;

/// Type alias for the largest molecular formula tree.
pub type LargestTree = GenericTree<i32, u32, EmptyTree<i32, u32>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents a molecular formula, which can be an element, an ion, a solvate,
/// or a count of molecules.
pub struct MolecularFormula<T: Tree = DefaultTree> {
    /// The molecular formula tree.
    mixtures: Vec<(T::Unsigned, T)>,
    /// Optional greek letter decorator.
    greek: Option<GreekLetter>,
}

/// Type alias for molecular formulas that can contain residuals.
pub type ResidualFormula = MolecularFormula<GenericResidualTree<i32, u32>>;

impl<T: Tree> AsRef<[(T::Unsigned, T)]> for MolecularFormula<T> {
    fn as_ref(&self) -> &[(T::Unsigned, T)] {
        &self.mixtures
    }
}

impl<T: Tree> MolecularFormula<T> {
    /// Adds a single mixture to the molecular formula.
    ///
    /// # Errors
    ///
    /// Returns an error if the Greek letter is not supported or if the count of
    /// mixtures overflows the unsigned type.
    pub fn mix(self, mixture: Self) -> Result<Self, Error<T::Signed, T::Unsigned>> {
        if mixture.greek.is_some() {
            return Err(Error::GreekLetterNotSupported);
        }

        let mut new_mixtures = self.mixtures;

        for (repeats, component) in mixture.mixtures {
            let mut found_duplicate = false;
            for (existing_repeats, existing_component) in &mut new_mixtures {
                if *existing_component == component {
                    found_duplicate = true;
                    *existing_repeats = existing_repeats
                        .checked_add(&repeats)
                        .ok_or(Error::InsufficientUnsignedTypeForCount)?;
                }
            }
            if !found_duplicate {
                new_mixtures.push((repeats, component));
            }
        }

        Ok(MolecularFormula { mixtures: new_mixtures, greek: self.greek })
    }
}
