//! Module defining allowed lowercase letters in a molecular formula.

use std::fmt::Display;

/// Enumeration of allowed lowercase letters in a molecular formula.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(non_camel_case_types)]
pub enum AllowedLowercaseLetter {
    /// The letter 'a'
    a,
    /// The letter 'b'
    b,
    /// The letter 'c'
    c,
    /// The letter 'd'
    d,
    /// The letter 'e'
    e,
    /// The letter 'f'
    f,
    /// The letter 'g'
    g,
    /// The letter 'h'
    h,
    /// The letter 'i'
    i,
    /// The letter 'l'
    l,
    /// The letter 'k'
    k,
    /// The letter 'm'
    m,
    /// The letter 'n'
    n,
    /// The letter 'o'
    o,
    /// The letter 'p'
    p,
    /// The letter 'r'
    r,
    /// The letter 's'
    s,
    /// The letter 't'
    t,
    /// The letter 'u'
    u,
    /// The letter 'v'
    v,
    /// The letter 'y'
    y,
}

impl From<AllowedLowercaseLetter> for char {
    fn from(letter: AllowedLowercaseLetter) -> Self {
        match letter {
            AllowedLowercaseLetter::a => 'a',
            AllowedLowercaseLetter::b => 'b',
            AllowedLowercaseLetter::c => 'c',
            AllowedLowercaseLetter::d => 'd',
            AllowedLowercaseLetter::e => 'e',
            AllowedLowercaseLetter::f => 'f',
            AllowedLowercaseLetter::g => 'g',
            AllowedLowercaseLetter::h => 'h',
            AllowedLowercaseLetter::i => 'i',
            AllowedLowercaseLetter::l => 'l',
            AllowedLowercaseLetter::k => 'k',
            AllowedLowercaseLetter::m => 'm',
            AllowedLowercaseLetter::n => 'n',
            AllowedLowercaseLetter::o => 'o',
            AllowedLowercaseLetter::p => 'p',
            AllowedLowercaseLetter::r => 'r',
            AllowedLowercaseLetter::s => 's',
            AllowedLowercaseLetter::t => 't',
            AllowedLowercaseLetter::u => 'u',
            AllowedLowercaseLetter::v => 'v',
            AllowedLowercaseLetter::y => 'y',
        }
    }
}

impl Display for AllowedLowercaseLetter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

/// Error enumeration when converting a character to an
/// `AllowedLowercaseLetter`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AllowedLowercaseLetterError {
    /// The character is not an allowed lowercase letter.
    #[error("The character '{0}' is not an allowed lowercase letter.")]
    NotAllowedLowercaseLetter(char),
}

impl TryFrom<char> for AllowedLowercaseLetter {
    type Error = AllowedLowercaseLetterError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'a' => Ok(AllowedLowercaseLetter::a),
            'b' => Ok(AllowedLowercaseLetter::b),
            'c' => Ok(AllowedLowercaseLetter::c),
            'd' => Ok(AllowedLowercaseLetter::d),
            'e' => Ok(AllowedLowercaseLetter::e),
            'f' => Ok(AllowedLowercaseLetter::f),
            'g' => Ok(AllowedLowercaseLetter::g),
            'h' => Ok(AllowedLowercaseLetter::h),
            'i' => Ok(AllowedLowercaseLetter::i),
            'k' => Ok(AllowedLowercaseLetter::k),
            'l' => Ok(AllowedLowercaseLetter::l),
            'm' => Ok(AllowedLowercaseLetter::m),
            'n' => Ok(AllowedLowercaseLetter::n),
            'o' => Ok(AllowedLowercaseLetter::o),
            'p' => Ok(AllowedLowercaseLetter::p),
            'r' => Ok(AllowedLowercaseLetter::r),
            's' => Ok(AllowedLowercaseLetter::s),
            't' => Ok(AllowedLowercaseLetter::t),
            'u' => Ok(AllowedLowercaseLetter::u),
            'v' => Ok(AllowedLowercaseLetter::v),
            'y' => Ok(AllowedLowercaseLetter::y),
            _ => Err(AllowedLowercaseLetterError::NotAllowedLowercaseLetter(c)),
        }
    }
}
