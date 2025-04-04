use std::borrow::Borrow;

use super::{Float, LogProb, ProbabilitiesSumToGreaterThanOne};

pub trait Ln2: Sized {
    const LN_2: Self;
    const ZERO: Self;
    const NEG_INFINITY: Self;
}

impl Ln2 for f32 {
    const LN_2: Self = std::f32::consts::LN_2;
    const ZERO: Self = 0.0;
    const NEG_INFINITY: Self = f32::NEG_INFINITY;
}
impl Ln2 for f64 {
    const LN_2: Self = std::f64::consts::LN_2;
    const ZERO: Self = 0.0;
    const NEG_INFINITY: Self = f64::NEG_INFINITY;
}

impl<T: Float + Ln2> LogProb<T> {
    fn add_log_prob_internal(x: T, y: T) -> T {
        if x > y {
            x + (y - x).exp().ln_1p()
        } else if x < y {
            y + (x - y).exp().ln_1p()
        } else {
            x + T::LN_2
        }
    }
    /// Adds `[LogProb]` as raw probabilities and return the new log probability.
    #[inline(always)]
    pub fn add_log_prob(
        &self,
        y: LogProb<T>,
    ) -> Result<LogProb<T>, ProbabilitiesSumToGreaterThanOne> {
        let y = y.0;
        Ok(LogProb::new(Self::add_log_prob_internal(self.0, y))?)
    }

    /// Adds log probabilities but clamping at 0.0.
    #[inline(always)]
    pub fn add_log_prob_clamped(&self, y: LogProb<T>) -> LogProb<T> {
        match self.add_log_prob(y) {
            Ok(x) => x,
            Err(_err) => LogProb(T::ZERO),
        }
    }

    /// Adds log probabilities and return a float (so can be greater than 0.0).
    #[inline(always)]
    pub fn add_log_prob_float(&self, y: LogProb<T>) -> T {
        Self::add_log_prob_internal(self.0, y.0)
    }
}

fn log_sum_exp_allocate_inner<
    T: Float + Ln2 + std::iter::Sum,
    L: Borrow<LogProb<T>>,
    I: Iterator<Item = L>,
>(
    iterable: I,
) -> T {
    let mut max = LogProb(T::NEG_INFINITY);
    let v: Vec<_> = iterable
        .map(|x| {
            let x = x.borrow();
            if x > &max {
                max = *x;
            }
            *x
        })
        .collect();
    log_sum_exp_inner(&v, max)
}

fn log_sum_exp_inner<T: Float + std::iter::Sum + Ln2, L: Borrow<LogProb<T>>>(
    val: &[L],
    max: LogProb<T>,
) -> T {
    val.iter()
        .map(|x| (x.borrow().0 - max.0).exp())
        .sum::<T>()
        .ln()
        + max.0
}

///Adds up a slice of [`LogProb`] (as raw probabilities) and returns a new `Result<LogProb,
///ProbabilitiesSumToGreaterThanOne>`. Will only return `Ok` if the sum could be a valid
///[`LogProb`]
pub fn log_sum_exp<T: Float + std::iter::Sum + Ln2, L: Borrow<LogProb<T>> + Ord>(
    val: &[L],
) -> Result<LogProb<T>, ProbabilitiesSumToGreaterThanOne> {
    match val.iter().max() {
        Some(max) => Ok(LogProb::new(log_sum_exp_inner(val, *max.borrow()))?),
        None => Ok(LogProb(T::NEG_INFINITY)),
    }
}

///Adds up a slice of [`LogProb`] (as raw probabilities) and returns a [`LogProb`] where any values greater than 0.0 will
///be clamped at 0.0
pub fn log_sum_exp_clamped<T: Float + std::iter::Sum + Ln2, L: Borrow<LogProb<T>> + Ord>(
    val: &[L],
) -> LogProb<T> {
    match val.iter().max() {
        Some(max) => match LogProb::new(log_sum_exp_inner(val, *max.borrow())) {
            Ok(x) => x,
            Err(_) => LogProb(T::ZERO),
        },
        None => LogProb(T::NEG_INFINITY),
    }
}

///Adds up a slice of [`LogProb`] (as raw probabilities) and returns a float with their sum,
///regardless of if it would be a valid [`LogProb`].
pub fn log_sum_exp_float<T: Float + std::iter::Sum + Ln2, L: Borrow<LogProb<T>> + Ord>(
    val: &[L],
) -> T {
    match val.iter().max() {
        Some(max) => log_sum_exp_inner(val, *max.borrow()),
        None => T::NEG_INFINITY,
    }
}

///This trait allows iterators to have `LogSumExp`.
pub trait LogSumExp: Iterator {
    ///Adds up an iterator of [`LogProb`] (as raw probabilities) and returns a new `Result<LogProb,
    ///ProbabilitiesSumToGreaterThanOne>`. Will only return `Ok` if the sum could be a valid
    ///[`LogProb`]. It does not allocate a vector.
    fn log_sum_exp_no_alloc<T: Float + Ln2, L: Borrow<LogProb<T>>>(
        mut self,
    ) -> Result<LogProb<T>, ProbabilitiesSumToGreaterThanOne>
    where
        Self: Sized,
        Self: Iterator<Item = L>,
    {
        match self.next() {
            Some(first) => self.try_fold(*first.borrow(), |acc, x| (*x.borrow()).add_log_prob(acc)),
            None => Ok(LogProb(T::NEG_INFINITY)),
        }
    }

    ///Adds up an iterator of [`LogProb`] (as raw probabilities) and returns a new [`LogProb`] clamping values greater than 0.0.
    ///Will only return `Ok` if the sum could be a valid [`LogProb`]. It does not allocate a vector and will often be faster than [`log_sum_exp_clamped`] if you expect there to be clamping as the iterator can short-circuit.
    fn log_sum_exp_clamped_no_alloc<T: Float + Ln2, L: Borrow<LogProb<T>>>(mut self) -> LogProb<T>
    where
        Self: Sized,
        Self: Iterator<Item = L>,
    {
        match self.next() {
            Some(first) => {
                match self.try_fold(*first.borrow(), |acc, x| x.borrow().add_log_prob(acc)) {
                    Ok(x) => x,
                    Err(_) => LogProb(T::ZERO),
                }
            }
            None => LogProb(T::NEG_INFINITY),
        }
    }

    ///Adds up an iterator of [`LogProb`] (as raw probabilities) and returns a float with their sum,
    ///regardless of if it would be a valid [`LogProb`]. It does not allocate a vector.
    fn log_sum_exp_float_no_alloc<T: Float + Ln2, L: Borrow<LogProb<T>>>(mut self) -> T
    where
        Self: Sized,
        Self: Iterator<Item = L>,
    {
        match self.next() {
            Some(x) => {
                let first: T = (*x.borrow()).into_inner();
                self.fold(first, |acc, x| {
                    LogProb::<T>::add_log_prob_internal(x.borrow().0, acc)
                })
            }
            None => T::NEG_INFINITY,
        }
    }

    ///Adds up an iterator of [`LogProb`] (as raw probabilities) and returns a new `Result<LogProb,
    ///ProbabilitiesSumToGreaterThanOne>`. Will only return `Ok` if the sum could be a valid
    ///[`LogProb`]. It does allocate a vector, but will usually be faster for n>10.
    fn log_sum_exp<T: Float + Ln2 + std::iter::Sum, L: Borrow<LogProb<T>>>(
        self,
    ) -> Result<LogProb<T>, ProbabilitiesSumToGreaterThanOne>
    where
        Self: Sized,
        Self: Iterator<Item = L>,
    {
        Ok(LogProb::new(log_sum_exp_allocate_inner(self))?)
    }

    ///Adds up an iterator of [`LogProb`] (as raw probabilities) and returns a float with their sum,
    ///regardless of if it would be a valid [`LogProb`]. It does allocate a vector, but is usally
    ///slower than [`Self::log_sum_exp_clamped_no_alloc`] if you expect clamping.
    fn log_sum_exp_clamped<T: Float + Ln2 + std::iter::Sum, L: Borrow<LogProb<T>>>(
        self,
    ) -> LogProb<T>
    where
        Self: Sized,
        Self: Iterator<Item = L>,
    {
        match LogProb::new(log_sum_exp_allocate_inner(self)) {
            Ok(x) => x,
            Err(_) => LogProb(T::ZERO),
        }
    }

    ///Adds up an iterator of [`LogProb`] (as raw probabilities) and returns a float with their sum,
    ///regardless of if it would be a valid [`LogProb`]. It does allocate a vector, but will usually be faster for n>10.
    fn log_sum_exp_float<T: Float + Ln2 + std::iter::Sum, L: Borrow<LogProb<T>>>(self) -> T
    where
        Self: Sized,
        Self: Iterator<Item = L>,
    {
        log_sum_exp_allocate_inner(self)
    }
}

impl<I: ?Sized> LogSumExp for I where I: Iterator {}
