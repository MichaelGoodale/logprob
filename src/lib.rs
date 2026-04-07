//! This crate defines a basic [`LogProb`] wrapper for floats. The struct is designed so
//! that only values that are coherent for a log-probability are acceptable. This means that
//! [`LogProb`] can store:
//!
//! - Any finite negative float value (e.g. -0.23, -32535.05, -66.0).
//! - Negative infinity (corresponding to 0.0 probability)
//! - 0.0 *and* -0.0.
//!
//! If any other value is passed, [`LogProb::new`] returns a [`FloatIsNanOrPositive`] error.
//! You can also construct new [`LogProb`] from values in \[0,1\] by using
//! [`LogProb::from_raw_prob`]
//!
//! The crate also includes the ability to add log probabilities (equivalent take the product of
//! their corresponding raw probabilities):
//!
//! ```
//! use logprob::LogProb;
//! let x = LogProb::from_raw_prob(0.5).unwrap();
//! let y = LogProb::from_raw_prob(0.5).unwrap();
//! let z = x + y;
//! assert_eq!(z, LogProb::from_raw_prob(0.25).unwrap());
//! ```
//!
//! It is also possible to take product of a [`LogProb`] and an unsigned integer, which
//! corresponds to taking the exponent of the log-probability to the power of the integer.
//! ```
//! # use logprob::LogProb;
//! let x = LogProb::from_raw_prob(0.5_f64).unwrap();
//! let y: u8 = 2;
//! let z = x * y;
//! assert_eq!(z, LogProb::from_raw_prob(0.25).unwrap());
//! ```
//!
//!Finally, the crate also includes reasonably efficient implementations of
//![LogSumExp](https://en.wikipedia.org/wiki/LogSumExp) so that one can take the sum of
//!raw-probabilities directly with [`LogProb`].
//!
//! ```
//! # use logprob::LogProb;
//! let x = LogProb::from_raw_prob(0.5_f64).unwrap();
//! let y = LogProb::from_raw_prob(0.25).unwrap();
//! let z = x.add_log_prob(y).unwrap();
//! assert_eq!(z, LogProb::from_raw_prob(0.75).unwrap());
//! ```
//!
//! This can also work for slices or iterators (by importing [`log_sum_exp`] or the trait,
//! [`LogSumExp`] respectively. Note that for empty vectors or iterators, the
//! functions return a [`LogProb`] with negative infinity, corresponding to 0 probability.
//! ```
//! # use logprob::LogProb;
//! use logprob::{LogSumExp, log_sum_exp};
//! let x = LogProb::from_raw_prob(0.5_f64).unwrap();
//! let y = LogProb::from_raw_prob(0.25).unwrap();
//! # #[cfg(feature = "alloc")]
//! let z = [x,y].iter().log_sum_exp().unwrap();
//! # #[cfg(not(feature = "alloc"))]
//! # let z = [x,y].iter().log_sum_exp_no_alloc().unwrap();
//! assert_eq!(z, LogProb::from_raw_prob(0.75).unwrap());
//! let v = log_sum_exp(&[x,y]).unwrap();
//! assert_eq!(z, LogProb::from_raw_prob(0.75).unwrap());
//! ```
//!
//! By default, the both [`log_sum_exp`] and [`LogProb::add_log_prob`] return a
//! [`ProbabilitiesSumToGreaterThanOne`] error if the sum is overflows what is a possible
//! [`LogProb`] value. However, one can use either the `clamped` or `float` versions of these
//! functions to return either a value clamped at 0.0 or the underlying float value which may be
//! greater than 0.0.
//! ```
//! # use logprob::LogProb;
//! # use logprob::{LogSumExp, log_sum_exp};
//! let x = LogProb::from_raw_prob(0.5_f64).unwrap();
//! let y = LogProb::from_raw_prob(0.75).unwrap();
//! # #[cfg(feature = "alloc")]
//! let z = [x,y].iter().log_sum_exp_clamped();
//! # #[cfg(not(feature = "alloc"))]
//! # let z = [x,y].iter().log_sum_exp_clamped_no_alloc();
//! assert_eq!(z, LogProb::new(0.0).unwrap());
//!
//! # #[cfg(feature = "alloc")]
//! let z = [x,y].into_iter().log_sum_exp_float();
//! # #[cfg(not(feature = "alloc"))]
//! # let z = [x,y].into_iter().log_sum_exp_float_no_alloc();
//!
//! approx::assert_relative_eq!(z, (1.25_f64).ln());
//!
//! ```

#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    nonstandard_style,
    single_use_lifetimes,
    rustdoc::broken_intra_doc_links,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

use core::borrow::Borrow;
use core::hash::Hash;
use num_traits::Float;
mod errors;
pub use errors::{
    FloatIsNanOrPositive, FloatIsNanOrPositiveInfinity, LogProbSubtractionError,
    ProbabilitiesSumToGreaterThanOne,
};
mod adding;
mod math;

#[cfg(feature = "alloc")]
mod softmax;

#[cfg(feature = "alloc")]
pub use softmax::{softmax, Softmax};

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
///Struct that can only hold float values that correspond to negative log
///probabilities.
///
///## Subtraction and Addition
///[`LogProb`] implements both [`Add`](core::ops::Add) and [`Sub`](core::ops::Sub) to represent multiplication and divisions of
///probabilities respectively.
///
///[`Sub`](core::ops::Sub) can panic if the denominator is negative infinity (as this is division by zero) or if
///the numerator is greater than the denominator (as this would lead to a number greater than 1.0
///in probability space).
///
///Both of these will panic in debug mode, while in release they will silenty saturate (see [`LogProb::saturating_sub`] for details).
///```should_panic
///# use logprob::LogProb;
///let _ = LogProb::new(f32::NEG_INFINITY).unwrap() - LogProb::new(f32::NEG_INFINITY).unwrap();
///```
///
///```should_panic
///# use logprob::LogProb;
///let _ = LogProb::new(-3.0).unwrap() - LogProb::new(-4.0).unwrap();
///```
///
///## `Ord` and `Hash`
///`LogProb` implements both `Hash` and `Ord` since we no longer have `NaN` values.
///However, one should always remember that floating point numbers won't necessarily correspond to
///the exact real number that one might expect when doing different mathematical operations.
///
/// ```
/// # #[cfg(feature = "std")]
/// # use std::collections::HashSet;
/// # #[cfg(feature = "std")]
/// # use std::collections::BTreeSet;
/// # use logprob::LogProb;
/// # fn main() -> anyhow::Result<()> {
/// let a = LogProb::new(-0.1_f64 - 0.2_f64).unwrap();
/// let b = LogProb::new(-0.3_f64).unwrap();
///
/// # #[cfg(feature = "std")]
/// let set = HashSet::from([a, b]);
/// # #[cfg(feature = "std")]
/// let o_set =BTreeSet::from([a,b]);
///
/// //Since the floats aren't exactly equal like one might expect,
/// //we have 2 elements in both collections
/// # #[cfg(feature = "std")]
/// assert_eq!(set.len(), 2);
/// # #[cfg(feature = "std")]
/// assert_eq!(o_set.len(), 2);
/// # Ok(())
/// # }
/// ```
#[repr(transparent)]
pub struct LogProb<T>(T);
pub use adding::{log_sum_exp, log_sum_exp_clamped, log_sum_exp_float, LogSumExp};

impl<T: Float> LogProb<T> {
    ///Construct a new [`LogProb`] that is guaranteed to be negative (or +0.0).
    ///
    ///# Errors
    ///Returns [`FloatIsNanOrPositive`] if the float is NaN or positive.
    pub fn new(val: T) -> Result<Self, FloatIsNanOrPositive> {
        if val.is_nan() || (!val.is_zero() && val.is_sign_positive()) {
            Err(FloatIsNanOrPositive)
        } else {
            Ok(LogProb(val))
        }
    }

    ///Construct a new [`LogProb`] that is guaranteed to be negative (or +0.0) from a value in [0.0, 1.0].
    ///# Errors
    ///Returns [`FloatIsNanOrPositive`] if the *logarithm* of the float is NaN or positive.
    pub fn from_raw_prob(val: T) -> Result<Self, FloatIsNanOrPositive> {
        let val = val.ln();
        if val.is_nan() || (!val.is_zero() && val.is_sign_positive()) {
            Err(FloatIsNanOrPositive)
        } else {
            Ok(LogProb(val))
        }
    }

    ///Constructs a new `LogProb` which corresponds to a probability of zero (e.g. neg infinity)
    #[must_use]
    pub fn prob_of_zero() -> Self {
        LogProb(T::neg_infinity())
    }

    ///Constructs a new `LogProb` which corresponds to a probability of one (e.g. the log prob is
    ///equal to 0)
    #[must_use]
    pub fn prob_of_one() -> Self {
        LogProb(T::zero())
    }

    /// Gets out the value.
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> T {
        self.0
    }

    /// Get the equivalent non-log probability
    /// ```
    /// # use logprob::LogProb;
    /// let x = LogProb::from_raw_prob(0.25).unwrap();
    /// assert_eq!(x.raw_prob(), 0.25);
    /// ```
    #[inline]
    #[must_use]
    pub fn raw_prob(&self) -> T {
        self.0.exp()
    }

    /// Calculates the probability of the complement of this log-probability
    /// ```
    /// # use logprob::LogProb;
    /// let x = LogProb::from_raw_prob(0.25).unwrap();
    /// let y = LogProb::from_raw_prob(0.75).unwrap();
    /// assert_eq!(x.opposite_prob(), y);
    /// ```
    #[must_use]
    pub fn opposite_prob(&self) -> Self {
        LogProb((-self.0.exp()).ln_1p())
    }
}

impl<T: Float + core::fmt::Display> core::fmt::Display for LogProb<T> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl Borrow<f32> for LogProb<f32> {
    #[inline]
    fn borrow(&self) -> &f32 {
        &self.0
    }
}

impl Borrow<f64> for LogProb<f64> {
    #[inline]
    fn borrow(&self) -> &f64 {
        &self.0
    }
}

impl<T: Float> Eq for LogProb<T> {}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl<T: Float> Ord for LogProb<T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}
impl Hash for LogProb<f32> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}
impl Hash for LogProb<f64> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl From<LogProb<f32>> for f32 {
    #[inline]
    fn from(f: LogProb<f32>) -> f32 {
        f.0
    }
}

impl From<LogProb<f64>> for f64 {
    #[inline]
    fn from(f: LogProb<f64>) -> f64 {
        f.0
    }
}
impl TryFrom<f64> for LogProb<f64> {
    type Error = FloatIsNanOrPositive;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        LogProb::new(value)
    }
}

impl TryFrom<f32> for LogProb<f32> {
    type Error = FloatIsNanOrPositive;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        LogProb::new(value)
    }
}
