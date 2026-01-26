//! Subroutines for displaying tokens.

use elements_rs::{ElementVariant, Isotope, MassNumber};

use crate::{SuperscriptMinus, SuperscriptPlus, superscript_digits_ltr};

/// Displays an isotope in the format `[<superscript_mass><element>]`.
///
/// # Arguments
/// * `isotope` - The isotope to display.
/// * `f` - The formatter to write to.
pub(crate) fn display_isotope(
    isotope: Isotope,
    f: &mut core::fmt::Formatter<'_>,
) -> core::fmt::Result {
    write!(f, "[")?;
    for superscript in superscript_digits_ltr(isotope.mass_number()) {
        write!(f, "{superscript}")?;
    }
    write!(f, "{}", isotope.element())?;
    write!(f, "]")
}

/// Displays a charge in the format `<magnitude><sign>` using superscript digits.
///
/// The magnitude is displayed only if it is greater than 1.
///
/// # Arguments
/// * `charge` - The charge value (will be converted to i64).
/// * `f` - The formatter to write to.
pub(crate) fn display_charge<C: Into<i64>>(
    charge: C,
    f: &mut core::fmt::Formatter<'_>,
) -> core::fmt::Result {
    // We convert the charge into i64 to avoid potential overflows when
    // executing the `abs` method on smaller integer types.
    let charge: i64 = charge.into();
    if charge.abs() > 1 {
        for digit in superscript_digits_ltr(charge) {
            write!(f, "{digit}")?;
        }
    }
    if charge < 0 { write!(f, "{SuperscriptMinus}") } else { write!(f, "{SuperscriptPlus}") }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::fmt::Display;
    use elements_rs::Element;
    use alloc::string::ToString;

    struct IsotopeWrapper(Isotope);
    impl Display for IsotopeWrapper {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            display_isotope(self.0, f)
        }
    }

    struct ChargeWrapper(i32);
    impl Display for ChargeWrapper {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            display_charge(self.0, f)
        }
    }

    #[test]
    fn test_display_isotope() {
        let c13 = Isotope::try_from((Element::C, 13_u16)).unwrap();
        assert_eq!(IsotopeWrapper(c13).to_string(), "[¹³C]");
        
        let h2 = Isotope::try_from((Element::H, 2_u16)).unwrap();
        assert_eq!(IsotopeWrapper(h2).to_string(), "[²H]");
    }

    #[test]
    fn test_display_charge() {
        assert_eq!(ChargeWrapper(1).to_string(), "⁺");
        assert_eq!(ChargeWrapper(-1).to_string(), "⁻");
        assert_eq!(ChargeWrapper(2).to_string(), "²⁺");
        assert_eq!(ChargeWrapper(-2).to_string(), "²⁻");
        assert_eq!(ChargeWrapper(10).to_string(), "¹⁰⁺");
        assert_eq!(ChargeWrapper(-10).to_string(), "¹⁰⁻");
    }
}
