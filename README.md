# logprob

This crate defines a basic `LogProb` wrapper for floats. The struct is designed so
that only values that are coherent for a log-probability are acceptable. This means that
`LogProb` can store:

- Any finite negative float value (e.g. -0.23, -32535.05, -66.0).
- Negative infinity (corresponding to 0.0 probability)
- 0.0 _and_ -0.0.

The crate is intended for careful implementations of computations involving log-probabilities.

## Features:

- A way to add `LogProb`s (equivalent take the product of their corresponding raw probabilities)
- Take the product of a `LogProb` and an unsigned integer (e.g. equivalent to p^n).
- `Ord` and `Eq` trait on `LogProb` as there is no `NaN`.
- A relatively efficient implementation of [LogSumExp](https://en.wikipedia.org/wiki/LogSumExp) for slices and iterators.

For examples, see the documentation.
