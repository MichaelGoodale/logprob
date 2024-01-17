#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    //missing_docs,
    rust_2018_idioms,
    nonstandard_style,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]

use std::borrow::Borrow;

pub use num_traits::{Float, Unsigned};
mod errors;
pub use errors::{FloatIsNanOrPositive, MultiplicandIsZero, ProbabilitiesSumToGreaterThanOne};
pub mod adding;
mod math;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
#[repr(transparent)]
pub struct LogProb<T>(T);
pub use adding::{log_sum_exp, log_sum_exp_clamped, log_sum_exp_float, LogSumExp};

impl<T: Float> LogProb<T> {
    ///Construct a new LogProb that is guarnteed to be negative (or +0.0).
    pub fn new(val: T) -> Result<Self, FloatIsNanOrPositive> {
        if val.is_nan() || (!val.is_zero() && val.is_sign_positive()) {
            Err(FloatIsNanOrPositive)
        } else {
            Ok(LogProb(val))
        }
    }

    ///Construct a new LogProb that is guarnteed to be negative (or +0.0) from a value in [0.0, 1.0].
    pub fn from_not_log(val: T) -> Result<Self, FloatIsNanOrPositive> {
        let val = val.ln();
        if val.is_nan() || (!val.is_zero() && val.is_sign_positive()) {
            Err(FloatIsNanOrPositive)
        } else {
            Ok(LogProb(val))
        }
    }

    /// Get the value out.
    #[inline]
    pub fn into_inner(self) -> T {
        self.0
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
