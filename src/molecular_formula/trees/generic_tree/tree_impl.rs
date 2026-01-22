//! Submodule iplementing `Tree` for `GenericTree`

use elements_rs::{Element, ElementVariant, Isotope};

use crate::{ChargeLike, CountLike, Tree, molecular_formula::GenericTree};

impl<S: ChargeLike + TryFrom<U>, U: CountLike, E: Tree<Unsigned = U, Signed = S>> Tree
    for GenericTree<S, U, E>
{
    type Unsigned = U;
    type Signed = S;

    fn iter_elements(&self) -> Box<dyn Iterator<Item = Element> + '_> {
        match self {
            Self::Element(element) => Box::new(std::iter::once(*element)),
            Self::Isotope(isotope) => Box::new(std::iter::once(isotope.element())),
            Self::Sequence(formulas) => Box::new(formulas.iter().flat_map(|f| f.iter_elements())),
            Self::Repeat(inner, _)
            | Self::Charge(inner, _)
            | Self::Unit(inner, _)
            | Self::Radical(inner, _) => inner.iter_elements(),
            Self::Extension(ext) => ext.iter_elements(),
        }
    }

    fn iter_counted_elements(&self) -> Box<dyn Iterator<Item = Element> + '_> {
        match self {
            Self::Element(element) => Box::new(std::iter::once(*element)),
            Self::Isotope(isotope) => Box::new(std::iter::once(isotope.element())),
            Self::Sequence(formulas) => {
                Box::new(formulas.iter().flat_map(|f| f.iter_counted_elements()))
            }
            Self::Repeat(inner, count) => {
                let n: u64 = (*count).into();
                Box::new((0..n).flat_map(move |_| inner.iter_counted_elements()))
            }
            Self::Charge(inner, _) | Self::Unit(inner, _) | Self::Radical(inner, _) => {
                inner.iter_counted_elements()
            }
            Self::Extension(ext) => ext.iter_counted_elements(),
        }
    }

    fn element_count(&self, target: Element) -> Option<u64> {
        match self {
            Self::Element(element) => Some(u64::from(*element == target)),
            Self::Isotope(isotope) => Some(u64::from(isotope.element() == target)),
            Self::Sequence(formulas) => {
                formulas.iter().try_fold(0u64, |acc, f| acc.checked_add(f.element_count(target)?))
            }
            Self::Repeat(inner, count) => {
                let n: u64 = (*count).into();
                n.checked_mul(inner.element_count(target)?)
            }
            Self::Charge(inner, _) | Self::Unit(inner, _) | Self::Radical(inner, _) => {
                inner.element_count(target)
            }
            Self::Extension(ext) => ext.element_count(target),
        }
    }

    fn iter_isotopes(&self) -> Box<dyn Iterator<Item = Isotope> + '_> {
        match self {
            Self::Element(_) => Box::new(std::iter::empty()),
            Self::Isotope(isotope) => Box::new(std::iter::once(*isotope)),
            Self::Sequence(formulas) => Box::new(formulas.iter().flat_map(|f| f.iter_isotopes())),
            Self::Repeat(inner, _)
            | Self::Charge(inner, _)
            | Self::Unit(inner, _)
            | Self::Radical(inner, _) => inner.iter_isotopes(),
            Self::Extension(ext) => ext.iter_isotopes(),
        }
    }

    fn isotope_count(&self, target: Isotope) -> Option<u64> {
        match self {
            Self::Element(_) => Some(0),
            Self::Isotope(isotope) => Some(u64::from(*isotope == target)),
            Self::Sequence(formulas) => {
                formulas.iter().try_fold(0u64, |acc, f| acc.checked_add(f.isotope_count(target)?))
            }
            Self::Repeat(inner, count) => {
                let n: u64 = (*count).into();
                n.checked_mul(inner.isotope_count(target)?)
            }
            Self::Charge(inner, _) | Self::Unit(inner, _) | Self::Radical(inner, _) => {
                inner.isotope_count(target)
            }
            Self::Extension(ext) => ext.isotope_count(target),
        }
    }

    /// # Implementation details
    ///
    /// ## Proof of No Overflow in `total` Accumulation
    ///
    /// **Proposition**: The `total` variable in the `Sequence` branch cannot
    /// overflow `u64` while we are still looping.
    ///
    /// **Proof**:
    /// 1. Let $I_0$ be the initial `index` value (of type `u64`).
    /// 2. Let $T_k$ be the value of `total` after processing $k$ sub-formulas.
    /// 3. Let $I_k$ be the value of `index` after processing $k$ sub-formulas.
    /// 4. **Invariant**: $I_0 = T_k + I_k$ at every step.
    ///     * Base case ($k=0$): $T_0 = 0$, $I_k = I_0$. Holds.
    ///     * Step: When we advance past a formula of size $S$, it implies $I_k
    ///       \ge S$. We update: $I_{k+1} = I_k - S$ and $T_{k+1} = T_k + S$.
    ///     * Check: $T_{k+1} + I_{k+1} = (T_k + S) + (I_k - S) = T_k + I_k =
    ///       I_0$.
    /// 5. **Contradiction**:
    ///     * Assume `total` overflows `u64`. Then $T_k > \text{u64::MAX}$.
    ///     * Since $I_k$ is unsigned, $I_k \ge 0$.
    ///     * From invariant: $I_0 = T_k + I_k \ge T_k > \text{u64::MAX}$.
    ///     * This implies $I_0 > \text{u64::MAX}$, which is impossible since
    ///       $I_0$ is a `u64`.
    /// 6. **Conclusion**: `total` never exceeds `u64::MAX` inside the loop.
    fn get_counted_element_or_size(&self, mut index: u64) -> Result<Element, u64> {
        match self {
            Self::Element(element) => {
                if index == 0 {
                    Ok(*element)
                } else {
                    Err(1)
                }
            }
            Self::Isotope(isotope) => {
                if index == 0 {
                    Ok(isotope.element())
                } else {
                    Err(1)
                }
            }
            Self::Sequence(formulas) => {
                let mut total: u64 = 0;
                for f in formulas {
                    match f.get_counted_element_or_size(index) {
                        Ok(element) => return Ok(element),
                        Err(count) => {
                            // Check subtract just in case logic is flawed elsewhere.
                            index = index.checked_sub(count).unwrap();
                            total = total.checked_add(count).expect("Tree size overflowed u64");
                        }
                    }
                }
                Err(total)
            }
            Self::Repeat(inner, count) => {
                let n: u64 = (*count).into();
                match inner.get_counted_element_or_size(index) {
                    Ok(element) => Ok(element),
                    Err(inner_size) => {
                        let total_opt = n.checked_mul(inner_size);
                        if let Some(total) = total_opt {
                            if index < total {
                                let inner_index = index % inner_size;
                                inner.get_counted_element(inner_index).ok_or(total)
                            } else {
                                Err(total)
                            }
                        } else {
                            // Overflow. index < total implied.
                            let inner_index = index % inner_size;
                            // We return u64::MAX as error if somehow not found? Impossible.
                            inner.get_counted_element(inner_index).ok_or(u64::MAX)
                        }
                    }
                }
            }
            Self::Charge(inner, _) | Self::Unit(inner, _) | Self::Radical(inner, _) => {
                inner.get_counted_element_or_size(index)
            }
            Self::Extension(ext) => ext.get_counted_element_or_size(index),
        }
    }
}
