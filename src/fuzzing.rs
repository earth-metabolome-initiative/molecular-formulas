//! Module providing fuzzing utilities for molecular formulas.
#![cfg(feature = "fuzzing")]
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::fmt::Display;

use arbitrary::{Arbitrary, Result, Unstructured};

use crate::{ChargeLike, CountLike, Token};

#[derive(Debug, Clone)]
/// Wrapper struct for fuzzing molecular formulas.
pub struct FuzzFormula<Count: CountLike, Charge: ChargeLike, Extension> {
    /// The string representation of the molecular formula.
    formula: String,
    _marker: core::marker::PhantomData<(Count, Charge, Extension)>,
}

impl<Count: CountLike, Charge: ChargeLike, Extension> From<FuzzFormula<Count, Charge, Extension>>
    for String
{
    fn from(fuzz_formula: FuzzFormula<Count, Charge, Extension>) -> Self {
        fuzz_formula.formula
    }
}

impl<Count: CountLike, Charge: ChargeLike, Extension> AsRef<str>
    for FuzzFormula<Count, Charge, Extension>
{
    fn as_ref(&self) -> &str {
        &self.formula
    }
}

impl<'a, Count: CountLike, Charge: ChargeLike, Extension: Display> Arbitrary<'a>
    for FuzzFormula<Count, Charge, Extension>
where
    Token<Count, Charge, Extension>: Arbitrary<'a>,
{
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let tokens: Vec<Token<Count, Charge, Extension>> = u.arbitrary()?;
        let mut s = String::new();
        for token in tokens {
            s.push_str(&token.to_string());
        }
        Ok(FuzzFormula { formula: s, _marker: core::marker::PhantomData })
    }
}
