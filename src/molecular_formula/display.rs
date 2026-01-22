//! Submodule implementing the `Display` trait for the `MolecularFormula` struct

use std::fmt::Display;

use num_traits::ConstOne;

use super::MolecularFormula;
use crate::Tree;

impl<T: Tree + Display> Display for MolecularFormula<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // If the formula has a greek letter, print it first
        if let Some(greek) = &self.greek {
            write!(f, "{greek}-")?;
        }

        let mut first = true;
        for (count, mixture) in &self.mixtures {
            if !first {
                write!(f, ".")?;
            }
            if count != &<T::Unsigned as ConstOne>::ONE {
                write!(f, "{count}")?;
            }
            first = false;
            write!(f, "{mixture}")?;
        }

        Ok(())
    }
}
