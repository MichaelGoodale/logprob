use std::error::Error;
/// An error for when a [`LogProb`](super::LogProb) is passed a value that isn't negative.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct FloatIsNanOrPositive;

impl Error for FloatIsNanOrPositive {}

impl std::fmt::Display for FloatIsNanOrPositive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LogProb constructed with positive or NaN value")
    }
}

/// An error for when a [`LogProb`](super::LogProb)  is passed a value that isn't negative.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ProbabilitiesSumToGreaterThanOne;

impl Error for ProbabilitiesSumToGreaterThanOne {}

impl std::fmt::Display for ProbabilitiesSumToGreaterThanOne {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The sum is greater than 1.0 (improper distribution)")
    }
}

impl From<FloatIsNanOrPositive> for ProbabilitiesSumToGreaterThanOne {
    fn from(_value: FloatIsNanOrPositive) -> Self {
        ProbabilitiesSumToGreaterThanOne
    }
}
