use super::LogProb;
use std::ops::{Add, AddAssign, Mul};

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
        impl Mul<LogProb<$float>> for $unsigned {
            type Output = LogProb<$float>;

            fn mul(self, rhs: LogProb<$float>) -> Self::Output {
                let s: $float = self as $float;
                LogProb(s * rhs.0)
            }
        }

        impl Mul<$unsigned> for LogProb<$float> {
            type Output = LogProb<$float>;

            fn mul(self, rhs: $unsigned) -> Self::Output {
                let s: $float = rhs as $float;
                LogProb(s * self.0)
            }
        }

        impl Mul<&$unsigned> for LogProb<$float> {
            type Output = LogProb<$float>;

            fn mul(self, rhs: &$unsigned) -> Self::Output {
                let s: $float = (*rhs) as $float;
                LogProb(s * self.0)
            }
        }

        impl Mul<&LogProb<$float>> for $unsigned {
            type Output = LogProb<$float>;

            fn mul(self, rhs: &LogProb<$float>) -> Self::Output {
                let s: $float = self as $float;
                LogProb(s * rhs.0)
            }
        }

        impl Mul<LogProb<$float>> for &$unsigned {
            type Output = LogProb<$float>;

            fn mul(self, rhs: LogProb<$float>) -> Self::Output {
                let s: $float = (*self) as $float;
                LogProb(s * rhs.0)
            }
        }
        impl Mul<$unsigned> for &LogProb<$float> {
            type Output = LogProb<$float>;

            fn mul(self, rhs: $unsigned) -> Self::Output {
                let s: $float = rhs as $float;
                LogProb(s * self.0)
            }
        }
        impl Mul<&LogProb<$float>> for &$unsigned {
            type Output = LogProb<$float>;

            fn mul(self, rhs: &LogProb<$float>) -> Self::Output {
                let s: $float = (*self) as $float;
                LogProb(s * rhs.0)
            }
        }

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
