use super::{Float, FloatIsNanOrPositiveInfinity, LogProb, adding::Ln2};
use core::iter::Sum;

use alloc::vec::Vec;

///Returns an iterator with the softmax values of a slice of floats.
///# Errors
///Returns [`FloatIsNanOrPositiveInfinity`] if any float is NaN or positive infinity.
#[expect(clippy::missing_panics_doc)]
pub fn softmax<T: Float + Sum<T> + Ln2>(
    val: &[T],
) -> Result<impl Iterator<Item = LogProb<T>> + use<T>, FloatIsNanOrPositiveInfinity> {
    let v: Vec<_> = val
        .iter()
        .map(|x| {
            if x.is_nan() || (x.is_infinite() && x.is_sign_positive()) {
                Err(FloatIsNanOrPositiveInfinity)
            } else {
                Ok(*x)
            }
        })
        .collect::<Result<Vec<_>, FloatIsNanOrPositiveInfinity>>()?;
    let max: T = *val
        .iter()
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap_or(&T::ZERO);
    let s: T = v.iter().map(|&x| (x - max).exp()).sum::<T>().ln();

    Ok(v.into_iter().map(move |x| LogProb(x - s - max)))
}

///This trait allows iterators to have [`softmax`].
///
///# Example
///
/// ```
/// use logprob::{LogProb, LogSumExp, Softmax};
///
/// # fn main() -> anyhow::Result<()>{
/// let logits = [-1.0_f64, 0.0, 1.0, 2.0];
///
/// let probs: Vec<LogProb<_>> = logits
///     .iter()
///     .copied()
///     .softmax()?
///     .collect();
///
/// let total = probs.iter().log_sum_exp_float();
/// approx::assert_relative_eq!(total.exp(), 1.0, epsilon = 1e-9);
/// # Ok(())
/// # }
/// ```
pub trait Softmax: Iterator {
    ///Gets the softmax from an iterator as another iterator.
    ///
    ///# Errors
    ///Returns [`FloatIsNanOrPositiveInfinity`] if any float is NaN or positive infinity.
    fn softmax<T: Float + Sum<T> + Ln2>(
        self,
    ) -> Result<impl Iterator<Item = LogProb<T>>, FloatIsNanOrPositiveInfinity>
    where
        Self: Sized,
        Self: Iterator<Item = T>,
    {
        let v: Vec<_> = self.collect();
        softmax(&v)
    }
}

impl<I: ?Sized> Softmax for I where I: Iterator {}
