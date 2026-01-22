#![doc = include_str!("../README.md")]

pub mod errors;
mod molecular_formula;
/// Re-exports from the elements_rs crate.
pub use elements_rs::{Element, ElementVariant, Isotope, MassNumber};
pub use molecular_formula::{
    AllowedCharacter, AllowedCharacterError, Bracket, CharacterMarker, ChargeLike, Complex,
    CountLike, DefaultTree, Digit, GreekLetter, InstantiableTree, LargestTree, MolecularFormula,
    NoResidualsTree, ParseError, ParserError, Radical, Residual, ResidualFormula, SmallestTree,
    SubTokenError, SuperscriptMinus, SuperscriptPlus, Terminator, Token, TokenError, Tree,
};
pub mod index;
pub mod is_hill_sorted;
pub mod iter_elements;
