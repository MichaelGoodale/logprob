# Changelog

## 0.4.0 - 2026-04-08

### Changed

- **Breaking:** Bump Rust Edition edition to Rust 2024 ([#9](https://github.com/MichaelGoodale/logprob/pull/9); [@MichaelGoodale])

### Added

- Add logprob subtraction methods (equivalent to prob-space division): `Sub`, `SubAssign`, `saturating_sub`, `unchecked_sub`, `checked_sub`, `try_sub` - ([#7](https://github.com/MichaelGoodale/logprob/pull/7); [@lenianiva], [@MichaelGoodale])

### Fixed

- Broken hashing for `LogProb<f32>` and `LogProb<f64>` ([#5](https://github.com/MichaelGoodale/logprob/pull/5); [@lenianiva])

[@lenianiva]: https://github.com/lenianiva
[@MichaelGoodale]: https://github.com/MichaelGoodale
