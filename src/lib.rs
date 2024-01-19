//! This crate defines a basic [`LogProb`] wrapper for floats. The struct is designed so
//! that only values that are coherent for a log-probability are acceptable. This means that
//! [`LogProb`] can store:
//!     - Any finite negative float value (e.g. -0.23, -32535.05, -66.0).
//!     - Negative infinity (corresponding to 0.0 probability)
//!     - 0.0 *and* -0.0.
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
//! let z = [x,y].iter().log_sum_exp().unwrap();
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
//! let z = [x,y].into_iter().log_sum_exp_clamped();
//! assert_eq!(z, LogProb::new(0.0).unwrap());
//! let z = [x,y].into_iter().log_sum_exp_float();
//! approx::assert_relative_eq!(z, (1.25_f64).ln());
//!
//! ```
//!

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

use std::borrow::Borrow;

use num_traits::Float;
mod errors;
pub use errors::{FloatIsNanOrPositive, ProbabilitiesSumToGreaterThanOne};
mod adding;
mod math;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
#[repr(transparent)]

///Struct that can only hold float values that correspond to negative log
///probabilities.
pub struct LogProb<T>(T);
pub use adding::{log_sum_exp, log_sum_exp_clamped, log_sum_exp_float, LogSumExp};

impl<T: Float> LogProb<T> {
    ///Construct a new [`LogProb`] that is guaranteed to be negative (or +0.0).
    pub fn new(val: T) -> Result<Self, FloatIsNanOrPositive> {
        if val.is_nan() || (!val.is_zero() && val.is_sign_positive()) {
            Err(FloatIsNanOrPositive)
        } else {
            Ok(LogProb(val))
        }
    }

    ///Construct a new [`LogProb`] that is guaranteed to be negative (or +0.0) from a value in [0.0, 1.0].
    pub fn from_raw_prob(val: T) -> Result<Self, FloatIsNanOrPositive> {
        let val = val.ln();
        if val.is_nan() || (!val.is_zero() && val.is_sign_positive()) {
            Err(FloatIsNanOrPositive)
        } else {
            Ok(LogProb(val))
        }
    }

    /// Gets out the value.
    #[inline]
    pub fn into_inner(self) -> T {
        self.0
    }

    /// Get the equivalent non-log probability
    /// ```
    /// # use logprob::LogProb;
    /// let x = LogProb::from_raw_prob(0.25).unwrap();
    /// assert_eq!(x.raw_prob(), 0.25);
    /// ```
    #[inline]
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
    pub fn opposite_prob(&self) -> Self {
        LogProb((-self.0.exp()).ln_1p())
    }
}

impl<T: Float + std::fmt::Display> std::fmt::Display for LogProb<T> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
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
