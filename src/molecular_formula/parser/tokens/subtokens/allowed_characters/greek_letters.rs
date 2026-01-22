//! Submodule handling the tokenization of greek letters in molecular formulas.

use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a greek letter in a molecular formula.
///
/// These are not ALL of the greek letters, but only those which are used in
/// mineral formulas, i.e. `'α'`, `'β'`, `'γ'`, `'δ'`, `'φ'`, `'ω'`, `'λ'`,
/// `'μ'`, and `'π'`.
pub enum GreekLetter {
    /// Alpha
    Alpha,
    /// Beta
    Beta,
    /// Gamma
    Gamma,
    /// Delta
    Delta,
    /// Phi
    Phi,
    /// Omega
    Omega,
    /// Lambda
    Lambda,
    /// Mu
    Mu,
    /// Pi
    Pi,
}

impl From<GreekLetter> for char {
    fn from(letter: GreekLetter) -> Self {
        match letter {
            GreekLetter::Alpha => '\u{03b1}',
            GreekLetter::Beta => 'β',
            GreekLetter::Gamma => '\u{03b3}',
            GreekLetter::Delta => 'δ',
            GreekLetter::Phi => 'φ',
            GreekLetter::Omega => 'ω',
            GreekLetter::Lambda => 'λ',
            GreekLetter::Mu => 'μ',
            GreekLetter::Pi => 'π',
        }
    }
}

impl Display for GreekLetter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

impl TryFrom<char> for GreekLetter {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '\u{03b1}' => Ok(GreekLetter::Alpha),
            'β' => Ok(GreekLetter::Beta),
            '\u{03b3}' => Ok(GreekLetter::Gamma),
            'δ' => Ok(GreekLetter::Delta),
            'φ' => Ok(GreekLetter::Phi),
            'ω' => Ok(GreekLetter::Omega),
            'λ' => Ok(GreekLetter::Lambda),
            'μ' => Ok(GreekLetter::Mu),
            'π' => Ok(GreekLetter::Pi),
            _ => Err(()),
        }
    }
}
