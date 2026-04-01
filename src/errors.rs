use core::error::Error;
/// An error for when a [`LogProb`](super::LogProb) is passed a value that isn't negative.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct FloatIsNanOrPositive;

impl Error for FloatIsNanOrPositive {}

impl core::fmt::Display for FloatIsNanOrPositive {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "LogProb constructed with positive or NaN value")
    }
}

/// An error for when a [`LogProb`](super::LogProb)  is passed a value that isn't negative.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ProbabilitiesSumToGreaterThanOne;

impl Error for ProbabilitiesSumToGreaterThanOne {}

impl core::fmt::Display for ProbabilitiesSumToGreaterThanOne {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "The sum is greater than 1.0 (improper distribution)")
    }
}

impl From<FloatIsNanOrPositive> for ProbabilitiesSumToGreaterThanOne {
    fn from(_value: FloatIsNanOrPositive) -> Self {
        ProbabilitiesSumToGreaterThanOne
    }
}

/// An error for when [`softmax`](super::softmax) is passed a value that is NaN or infinity.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct FloatIsNanOrPositiveInfinity;

impl Error for FloatIsNanOrPositiveInfinity {}

impl core::fmt::Display for FloatIsNanOrPositiveInfinity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "LogProb constructed with positive or NaN value")
    }
}
