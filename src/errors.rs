use std::error::Error;
/// An error for when a LogProb is passed a value that isn't negative.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct FloatIsNanOrPositive;

impl Error for FloatIsNanOrPositive {
    fn description(&self) -> &str {
        "LogProb constructed with positive or NaN value"
    }
}

impl std::fmt::Display for FloatIsNanOrPositive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LogProb constructed with positive or NaN value")
    }
}

/// An error for when a LogProb is passed a value that isn't negative.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ProbabilitiesSumToGreaterThanOne;

impl Error for ProbabilitiesSumToGreaterThanOne {
    fn description(&self) -> &str {
        "The sum is greater than 1.0 (improper distribution)"
    }
}

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

/// An error for when a logprob is multiplied by zero.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct MultiplicandIsZero;

impl Error for MultiplicandIsZero {
    fn description(&self) -> &str {
        "LogProb cannot be multiplied by zero"
    }
}

impl std::fmt::Display for MultiplicandIsZero {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LogProb cannot be multiplied by zero")
    }
}
