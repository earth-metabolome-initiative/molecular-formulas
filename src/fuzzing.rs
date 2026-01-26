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

#[cfg(test)]
mod tests {
    use alloc::string::ToString;
    use core::marker::PhantomData;

    use super::*;
    use crate::{Empty, Residual};

    #[test]
    fn test_fuzz_formula_manual_residual() {
        let formula_str = "H2O";
        let fuzz_formula = FuzzFormula::<u16, i16, Residual> {
            formula: formula_str.to_string(),
            _marker: PhantomData,
        };

        assert_eq!(fuzz_formula.as_ref(), formula_str);
        assert_eq!(String::from(fuzz_formula), formula_str);
    }

    #[test]
    fn test_fuzz_formula_manual_empty() {
        let formula_str = "H2O";
        let fuzz_formula = FuzzFormula::<u16, i16, Empty> {
            formula: formula_str.to_string(),
            _marker: PhantomData,
        };

        assert_eq!(fuzz_formula.as_ref(), formula_str);
        assert_eq!(String::from(fuzz_formula), formula_str);
    }

    #[test]
    #[allow(clippy::cast_possible_truncation)]
    fn test_fuzz_formula_arbitrary_residual() {
        // Use pseudo-random bytes to ensure non-empty vectors are generated
        let mut bytes = Vec::with_capacity(4096);
        for i in 0u32..4096 {
            bytes.push(i.wrapping_mul(31).wrapping_add(17) as u8);
        }
        let mut u = Unstructured::new(&bytes);

        let mut hit_non_empty = false;
        // Consume as much as possible
        while !u.is_empty() {
            if let Ok(f) = FuzzFormula::<u16, i16, Residual>::arbitrary(&mut u) {
                let s: String = f.clone().into();
                assert_eq!(s, f.as_ref());

                if !s.is_empty() {
                    hit_non_empty = true;
                }
            } else {
                break;
            }
        }
        assert!(hit_non_empty, "Should have generated at least one non-empty formula");
    }

    #[test]
    #[allow(clippy::cast_possible_truncation)]
    fn test_fuzz_formula_arbitrary_empty() {
        // Use pseudo-random bytes to ensure non-empty vectors are generated
        let mut bytes = Vec::with_capacity(4096);
        for i in 0u32..4096 {
            bytes.push(i.wrapping_mul(31).wrapping_add(17) as u8);
        }
        let mut u = Unstructured::new(&bytes);

        let mut hit_non_empty = false;
        // Consume as much as possible
        while !u.is_empty() {
            if let Ok(f) = FuzzFormula::<u16, i16, Empty>::arbitrary(&mut u) {
                let s: String = f.clone().into();
                assert_eq!(s, f.as_ref());

                if !s.is_empty() {
                    hit_non_empty = true;
                }
            } else {
                break;
            }
        }
        assert!(hit_non_empty, "Should have generated at least one non-empty formula");
    }
}
