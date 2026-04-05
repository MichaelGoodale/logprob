use num_traits::Float;

use crate::errors::LogProbSubtractionError;

use super::LogProb;
use core::ops::{Add, AddAssign, Mul, Sub, SubAssign};

impl<T: Add> Add for LogProb<T> {
    type Output = LogProb<T::Output>;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        LogProb((self.0).add(other.0))
    }
}

impl<'a, T> Add<&'a Self> for LogProb<T>
where
    T: Add<&'a T>,
{
    type Output = LogProb<<T as Add<&'a T>>::Output>;

    #[inline]
    fn add(self, other: &'a Self) -> Self::Output {
        LogProb((self.0).add(&other.0))
    }
}

impl<'a, T> Add<LogProb<T>> for &'a LogProb<T>
where
    &'a T: Add<T>,
{
    type Output = LogProb<<&'a T as Add<T>>::Output>;

    #[inline]
    fn add(self, other: LogProb<T>) -> Self::Output {
        LogProb((self.0).add(other.0))
    }
}

impl<'a, 'b, T> Add<&'b LogProb<T>> for &'a LogProb<T>
where
    &'a T: Add<T>,
    T: Copy,
{
    type Output = LogProb<<&'a T as Add<T>>::Output>;

    #[inline]
    fn add(self, other: &'b LogProb<T>) -> Self::Output {
        LogProb((self.0).add(other.0))
    }
}

impl<T: AddAssign> AddAssign for LogProb<T> {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        (self.0).add_assign(other.0);
    }
}

impl<'a, T: AddAssign<&'a T>> AddAssign<&'a Self> for LogProb<T> {
    #[inline]
    fn add_assign(&mut self, other: &'a Self) {
        (self.0).add_assign(&other.0);
    }
}

impl<T: Sub + Float + SubAssign> SubAssign for LogProb<T> {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        if *self > other {
            panic!("Numerator is greater than denominator")
        }
        if !other.0.is_finite() {
            panic!("Division by zero in prob space");
        }
        self.0 -= other.0;
    }
}

impl<'a, T: Sub + Float + SubAssign<T>> SubAssign<&'a Self> for LogProb<T> {
    #[inline]
    fn sub_assign(&mut self, other: &'a Self) {
        if *self > *other {
            panic!("Numerator is greater than denominator")
        }
        if !other.0.is_finite() {
            panic!("Division by zero in prob space");
        }
        self.0 -= other.0;
    }
}

impl<T: Sub + Float> Sub for LogProb<T> {
    type Output = LogProb<<T as Sub>::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self > rhs {
            panic!("Numerator is greater than denominator")
        }
        if !rhs.0.is_finite() {
            panic!("Division by zero in prob space");
        }
        LogProb(self.0 - rhs.0)
    }
}
impl<'a, T> Sub<&'a Self> for LogProb<T>
where
    T: Sub<&'a T> + Float,
{
    type Output = LogProb<<T as Sub<&'a T>>::Output>;

    #[inline]
    fn sub(self, rhs: &'a Self) -> Self::Output {
        if self > *rhs {
            panic!("Numerator is greater than denominator")
        }
        if !rhs.0.is_finite() {
            panic!("Division by zero in prob space");
        }
        LogProb(self.0.sub(&rhs.0))
    }
}

impl<'a, T> Sub<LogProb<T>> for &'a LogProb<T>
where
    &'a T: Sub<T>,
    T: Float,
{
    type Output = LogProb<T>;

    #[inline]
    fn sub(self, rhs: LogProb<T>) -> Self::Output {
        if *self > rhs {
            panic!("Numerator is greater than denominator")
        }
        if !rhs.0.is_finite() {
            panic!("Division by zero in prob space");
        }
        LogProb(self.0.sub(rhs.0))
    }
}

impl<'a, 'b, T> Sub<&'b LogProb<T>> for &'a LogProb<T>
where
    &'a T: Sub<T>,
    T: Copy + Float,
{
    type Output = LogProb<T>;

    #[inline]
    fn sub(self, rhs: &'b LogProb<T>) -> Self::Output {
        if self > rhs {
            panic!("Numerator is greater than denominator")
        }
        if !rhs.0.is_finite() {
            panic!("Division by zero in prob space");
        }

        LogProb(self.0.sub(rhs.0))
    }
}

impl<T: Float> LogProb<T> {
    ///Subtracts two [`LogProb`]s (equivalent to division in prob-space) and returns [`LogProbSubtractionError`] if the
    ///resulting value isn't a valid [`LogProb`].
    ///```
    ///# use logprob::{LogProb, LogProbSubtractionError};
    ///# fn main() -> anyhow::Result<()> {
    ///let x = LogProb::new(-3.0)?.try_sub(LogProb::new(-2.0)?);
    ///assert_eq!(x, Ok(LogProb::new(-1.0)?));
    ///let x = LogProb::new(-2.0)?.try_sub(LogProb::new(-3.0)?);
    ///assert_eq!(x, Err(LogProbSubtractionError::NumeratorBiggerThanDenominator));
    ///let x = LogProb::new(-2.0)?.try_sub(LogProb::prob_of_zero());
    ///assert_eq!(x, Err(LogProbSubtractionError::NumeratorBiggerThanDenominator));
    ///let x = LogProb::<f32>::prob_of_zero().try_sub(LogProb::prob_of_zero());
    //assert_eq!(x, Err(LogProbSubtractionError::DivideByZero));
    ///# Ok(())
    ///# }
    ///```
    ///# Errors
    /// - [`LogProbSubtractionError::NumeratorBiggerThanDenominator`] if `self > rhs`
    /// - [`LogProbSubtractionError::DivideByZero`] if `rhs` is negative infinity
    pub fn try_sub(&self, rhs: LogProb<T>) -> Result<LogProb<T>, LogProbSubtractionError> {
        if *self > rhs {
            Err(LogProbSubtractionError::NumeratorBiggerThanDenominator)
        } else if !rhs.0.is_finite() {
            Err(LogProbSubtractionError::DivideByZero)
        } else {
            Ok(LogProb(self.0.sub(rhs.0)))
        }
    }

    ///Subtracts two [`LogProb`]s (equivalent to division in prob-space) and returns [`None`] if the
    ///resulting value isn't a valid [`LogProb`].
    ///
    ///```
    ///# use logprob::LogProb;
    ///# fn main() -> anyhow::Result<()> {
    ///let x = LogProb::new(-3.0)?.checked_sub(LogProb::new(-2.0)?);
    ///assert_eq!(x, Some(LogProb::new(-1.0)?));
    ///let x = LogProb::new(-2.0)?.checked_sub(LogProb::new(-3.0)?);
    ///assert_eq!(x, None);
    ///let x = LogProb::new(-2.0)?.checked_sub(LogProb::prob_of_zero());
    ///assert_eq!(x, None);
    ///# Ok(())
    ///# }
    ///```
    pub fn checked_sub(&self, rhs: LogProb<T>) -> Option<LogProb<T>> {
        if *self > rhs || !rhs.0.is_finite() {
            None
        } else {
            Some(LogProb(self.0.sub(rhs.0)))
        }
    }

    ///Subtracts two [`LogProb`]s (equivalent to division in prob-space) and returns a [`LogProb`] of
    ///0.0 if the numerator is greater than the denominator and a value of negative infinity is the
    ///denominator is negative infinity.
    ///
    ///
    ///```
    ///# use logprob::LogProb;
    ///# fn main() -> anyhow::Result<()> {
    ///let x = LogProb::new(-3.0)?.saturating_sub(LogProb::new(-2.0)?);
    ///assert_eq!(x, LogProb::new(-1.0)?);
    ///let x = LogProb::new(-2.0)?.saturating_sub(LogProb::new(-3.0)?);
    ///assert_eq!(x, LogProb::new(0.0)?);
    ///let x = LogProb::new(-2.0)?.saturating_sub(LogProb::prob_of_zero());
    ///assert_eq!(x, LogProb::new(0.0)?);
    ///let x = LogProb::prob_of_zero().saturating_sub(LogProb::prob_of_zero());
    ///assert_eq!(x, LogProb::new(0.0)?);
    ///# Ok(())
    ///# }
    ///```
    #[must_use]
    pub fn saturating_sub(&self, rhs: LogProb<T>) -> LogProb<T> {
        if self.0.is_infinite() && rhs.0.is_infinite() {
            LogProb(T::zero())
        } else {
            let x = self.0 - rhs.0;
            if x > T::zero() {
                LogProb(T::zero())
            } else {
                LogProb(x)
            }
        }
    }
}

macro_rules! impl_mul {
    ($unsigned: ty, $float: ty) => {
        impl Mul<LogProb<$float>> for $unsigned {
            type Output = LogProb<$float>;

            fn mul(self, rhs: LogProb<$float>) -> Self::Output {
                let s: $float = self.into();
                LogProb(s * rhs.0)
            }
        }

        impl Mul<$unsigned> for LogProb<$float> {
            type Output = LogProb<$float>;

            fn mul(self, rhs: $unsigned) -> Self::Output {
                let s: $float = rhs.into();
                LogProb(s * self.0)
            }
        }

        impl<'a> Mul<&'a $unsigned> for LogProb<$float> {
            type Output = LogProb<$float>;

            fn mul(self, rhs: &'a $unsigned) -> Self::Output {
                let s: $float = (*rhs).into();
                LogProb(s * self.0)
            }
        }

        impl<'a> Mul<&'a LogProb<$float>> for $unsigned {
            type Output = LogProb<$float>;

            fn mul(self, rhs: &'a LogProb<$float>) -> Self::Output {
                let s: $float = self.into();
                LogProb(s * rhs.0)
            }
        }

        impl Mul<LogProb<$float>> for &$unsigned {
            type Output = LogProb<$float>;

            fn mul(self, rhs: LogProb<$float>) -> Self::Output {
                let s: $float = (*self).into();
                LogProb(s * rhs.0)
            }
        }
        impl Mul<$unsigned> for &LogProb<$float> {
            type Output = LogProb<$float>;

            fn mul(self, rhs: $unsigned) -> Self::Output {
                let s: $float = rhs.into();
                LogProb(s * self.0)
            }
        }
        impl Mul<&LogProb<$float>> for &$unsigned {
            type Output = LogProb<$float>;

            fn mul(self, rhs: &LogProb<$float>) -> Self::Output {
                let s: $float = (*self).into();
                LogProb(s * rhs.0)
            }
        }

        impl Mul<&$unsigned> for &LogProb<$float> {
            type Output = LogProb<$float>;

            fn mul(self, rhs: &$unsigned) -> Self::Output {
                let s: $float = (*rhs).into();
                LogProb(s * self.0)
            }
        }
    };
}

impl_mul!(u8, f32);
impl_mul!(u8, f64);
impl_mul!(u16, f32);
impl_mul!(u16, f64);
impl_mul!(u32, f64);

macro_rules! impl_mul_lossy {
    ($unsigned: ty, $float: ty) => {
        #[expect(clippy::cast_precision_loss)]
        impl Mul<LogProb<$float>> for $unsigned {
            type Output = LogProb<$float>;

            fn mul(self, rhs: LogProb<$float>) -> Self::Output {
                let s: $float = self as $float;
                LogProb(s * rhs.0)
            }
        }

        #[expect(clippy::cast_precision_loss)]
        impl Mul<$unsigned> for LogProb<$float> {
            type Output = LogProb<$float>;

            fn mul(self, rhs: $unsigned) -> Self::Output {
                let s: $float = rhs as $float;
                LogProb(s * self.0)
            }
        }

        #[expect(clippy::cast_precision_loss)]
        impl Mul<&$unsigned> for LogProb<$float> {
            type Output = LogProb<$float>;

            fn mul(self, rhs: &$unsigned) -> Self::Output {
                let s: $float = (*rhs) as $float;
                LogProb(s * self.0)
            }
        }

        #[expect(clippy::cast_precision_loss)]
        impl Mul<&LogProb<$float>> for $unsigned {
            type Output = LogProb<$float>;

            fn mul(self, rhs: &LogProb<$float>) -> Self::Output {
                let s: $float = self as $float;
                LogProb(s * rhs.0)
            }
        }

        #[expect(clippy::cast_precision_loss)]
        impl Mul<LogProb<$float>> for &$unsigned {
            type Output = LogProb<$float>;

            fn mul(self, rhs: LogProb<$float>) -> Self::Output {
                let s: $float = (*self) as $float;
                LogProb(s * rhs.0)
            }
        }

        #[expect(clippy::cast_precision_loss)]
        impl Mul<$unsigned> for &LogProb<$float> {
            type Output = LogProb<$float>;

            fn mul(self, rhs: $unsigned) -> Self::Output {
                let s: $float = rhs as $float;
                LogProb(s * self.0)
            }
        }

        #[expect(clippy::cast_precision_loss)]
        impl Mul<&LogProb<$float>> for &$unsigned {
            type Output = LogProb<$float>;

            fn mul(self, rhs: &LogProb<$float>) -> Self::Output {
                let s: $float = (*self) as $float;
                LogProb(s * rhs.0)
            }
        }

        #[expect(clippy::cast_precision_loss)]
        impl Mul<&$unsigned> for &LogProb<$float> {
            type Output = LogProb<$float>;

            fn mul(self, rhs: &$unsigned) -> Self::Output {
                let s: $float = (*rhs) as $float;
                LogProb(s * self.0)
            }
        }
    };
}

impl_mul_lossy!(usize, f64);
impl_mul_lossy!(usize, f32);
impl_mul_lossy!(u64, f64);
impl_mul_lossy!(u64, f32);
impl_mul_lossy!(u32, f32);
impl_mul_lossy!(u128, f32);
impl_mul_lossy!(u128, f64);
