use anyhow::Result;
use logprob::{log_sum_exp, log_sum_exp_clamped, log_sum_exp_float, LogProb, LogSumExp};

#[test]
fn basic_construction() -> Result<()> {
    LogProb::new(-3.0)?;
    LogProb::new(f64::NEG_INFINITY)?;
    LogProb::new(0.0_f64)?;
    LogProb::new(-0.0_f64)?;
    assert!(LogProb::new(3.0).is_err());
    assert!(LogProb::new(f64::NAN).is_err());
    assert!(LogProb::new(f64::INFINITY).is_err());
    assert!(LogProb::new(23434.3432).is_err());

    let x: LogProb<f32> = (-3.0_f32).try_into()?;
    assert_eq!(x, LogProb::new(-3.0_f32)?);
    let x: LogProb<f64> = f64::NEG_INFINITY.try_into()?;
    assert_eq!(x, LogProb::new(f64::NEG_INFINITY)?);

    LogProb::new(-3.0_f64)?;
    LogProb::new(f32::NEG_INFINITY)?;
    LogProb::new(0.0_f32)?;
    LogProb::new(-0.0_f32)?;
    assert!(LogProb::new(3.0_f32).is_err());
    assert!(LogProb::new(f32::NAN).is_err());
    assert!(LogProb::new(f32::INFINITY).is_err());
    assert!(LogProb::new(23434.34_f32).is_err());
    Ok(())
}

#[test]
fn addition() -> Result<()> {
    let x = LogProb::new(-3.0)? + LogProb::new(-3.0)?;
    assert_eq!(x, LogProb::new(-6.0)?);

    let x = LogProb::new(-0.0)? + LogProb::new(0.0)?;
    assert_eq!(x, LogProb::new(-0.0)?);
    assert_eq!(x, LogProb::new(0.0)?);

    #[allow(clippy::op_ref)]
    let x = &(LogProb::new(-0.0)?) + LogProb::new(0.0)?;
    assert_eq!(x, LogProb::new(-0.0)?);
    assert_eq!(x, LogProb::new(0.0)?);

    #[allow(clippy::op_ref)]
    let mut x = &(LogProb::new(-0.0)?) + &(LogProb::new(0.0)?);
    assert_eq!(x, LogProb::new(-0.0)?);
    assert_eq!(x, LogProb::new(0.0)?);

    x += LogProb::new(-0.5)?;
    assert_eq!(x, LogProb::new(-0.5)?);

    x += &LogProb::new(-0.5)?;
    assert_eq!(x, LogProb::new(-1.0)?);

    let x = LogProb::new(-0.5)? + LogProb::new(f64::NEG_INFINITY)?;
    assert_eq!(x, LogProb::new(f64::NEG_INFINITY)?);
    Ok(())
}

#[test]
fn multiplication() -> Result<()> {
    let x: LogProb<f64> = LogProb::new(-3.0)? * 4_u32;
    assert_eq!(x, LogProb::new(-12.0)?);

    let x: LogProb<f64> = 4_u8 * LogProb::new(-3.0)?;
    assert_eq!(x, LogProb::new(-12.0)?);

    #[allow(clippy::op_ref)]
    let x: LogProb<f64> = &4_u8 * LogProb::new(-3.0)?;
    assert_eq!(x, LogProb::new(-12.0)?);

    #[allow(clippy::op_ref)]
    let x: LogProb<f64> = &4_u8 * &LogProb::new(-3.0)?;
    assert_eq!(x, LogProb::new(-12.0)?);

    #[allow(clippy::op_ref)]
    let x: LogProb<f64> = 4_u8 * &LogProb::new(-3.0)?;
    assert_eq!(x, LogProb::new(-12.0)?);

    #[allow(clippy::op_ref)]
    let x: LogProb<f64> = LogProb::new(-3.0)? * &4_u8;
    assert_eq!(x, LogProb::new(-12.0)?);

    #[allow(clippy::op_ref)]
    let x: LogProb<f64> = &LogProb::new(-3.0)? * &4_u8;
    assert_eq!(x, LogProb::new(-12.0)?);

    #[allow(clippy::op_ref)]
    let x: LogProb<f64> = &LogProb::new(-3.0)? * 4_u8;
    assert_eq!(x, LogProb::new(-12.0)?);
    Ok(())
}

#[test]
fn add_probs_test() -> Result<()> {
    let x = LogProb::from_raw_prob(0.5)?;
    let y = LogProb::from_raw_prob(0.25)?;
    assert_eq!(x.into_inner(), 0.5_f64.ln());
    assert_eq!(y.into_inner(), 0.25_f64.ln());

    let z = x.add_log_prob(y)?;
    assert_eq!(z, LogProb::from_raw_prob(0.75)?);
    assert!(x.add_log_prob(z).is_err());
    assert_eq!(x.add_log_prob_clamped(z), LogProb::new(0.0)?);
    approx::assert_relative_eq!(x.add_log_prob_float(z), (0.75 + 0.5_f64).ln());

    let x = LogProb::from_raw_prob(0.5)?;
    let y = LogProb::from_raw_prob(0.5)?;
    let z = x.add_log_prob(y)?;
    assert_eq!(z, LogProb::new(0.0)?);

    let sum = [0.5, 0.2, 0.3]
        .map(LogProb::from_raw_prob)
        .map(|x| x.unwrap())
        .into_iter()
        .log_sum_exp_no_alloc()?;
    approx::assert_relative_eq!(sum.into_inner(), LogProb::new(0.0)?.into_inner());

    let sum = [0.5, 0.2, 0.3]
        .map(LogProb::from_raw_prob)
        .map(|x| x.unwrap())
        .into_iter()
        .log_sum_exp()?;
    approx::assert_relative_eq!(sum.into_inner(), LogProb::new(0.0)?.into_inner());

    assert!([0.5, 0.5, 0.3]
        .map(LogProb::from_raw_prob)
        .map(|x| x.unwrap())
        .into_iter()
        .log_sum_exp_no_alloc()
        .is_err());

    assert!([0.5, 0.5, 0.3]
        .map(LogProb::from_raw_prob)
        .map(|x| x.unwrap())
        .into_iter()
        .log_sum_exp()
        .is_err());

    let sum = [0.5, 0.5, 0.5]
        .map(LogProb::from_raw_prob)
        .map(|x| x.unwrap())
        .into_iter()
        .log_sum_exp_clamped_no_alloc();
    approx::assert_relative_eq!(sum.into_inner(), 0.0);

    let sum = [0.5, 0.5, 0.5]
        .map(LogProb::from_raw_prob)
        .map(|x| x.unwrap())
        .into_iter()
        .log_sum_exp_clamped();
    approx::assert_relative_eq!(sum.into_inner(), 0.0);

    let sum = [0.5, 0.3, 0.0, 0.0]
        .map(LogProb::from_raw_prob)
        .map(|x| x.unwrap())
        .into_iter()
        .log_sum_exp_clamped_no_alloc();
    approx::assert_relative_eq!(sum.into_inner(), 0.8_f64.ln());

    let sum = [0.5, 0.3, 0.0, 0.0]
        .map(LogProb::from_raw_prob)
        .map(|x| x.unwrap())
        .into_iter()
        .log_sum_exp_clamped();
    approx::assert_relative_eq!(sum.into_inner(), 0.8_f64.ln());

    let sum = [0.5, 0.5, 0.5, 0.0]
        .map(LogProb::from_raw_prob)
        .map(|x| x.unwrap())
        .into_iter()
        .log_sum_exp_float_no_alloc();
    approx::assert_relative_eq!(sum, 1.5_f64.ln());

    let sum = [0.5, 0.5, 0.5, 0.0]
        .map(LogProb::from_raw_prob)
        .map(|x| x.unwrap())
        .into_iter()
        .log_sum_exp_float();
    approx::assert_relative_eq!(sum, 1.5_f64.ln());

    let v: Vec<_> = [0.5, 0.5, 0.5]
        .map(LogProb::from_raw_prob)
        .map(|x| x.unwrap())
        .into_iter()
        .collect();

    assert!(log_sum_exp(&v).is_err());
    approx::assert_relative_eq!(log_sum_exp_float(&v), 1.5_f64.ln());
    approx::assert_relative_eq!(log_sum_exp_clamped(&v).into_inner(), 0.0);

    let v_2: Vec<_> = v.iter().collect();

    assert!(log_sum_exp(&v_2).is_err());
    approx::assert_relative_eq!(log_sum_exp_float(&v_2), 1.5_f64.ln());
    approx::assert_relative_eq!(log_sum_exp_clamped(&v_2).into_inner(), 0.0);

    let v: Vec<LogProb<f64>> = vec![];
    assert_eq!(log_sum_exp(&v)?, LogProb::new(f64::NEG_INFINITY)?);
    assert_eq!(
        v.iter().log_sum_exp_no_alloc()?,
        LogProb::new(f64::NEG_INFINITY)?
    );
    assert_eq!(log_sum_exp_clamped(&v), LogProb::new(f64::NEG_INFINITY)?);
    assert_eq!(
        v.iter().log_sum_exp_clamped_no_alloc(),
        LogProb::new(f64::NEG_INFINITY)?
    );
    assert_eq!(log_sum_exp_float(&v), f64::NEG_INFINITY);
    assert_eq!(v.iter().log_sum_exp_float_no_alloc(), f64::NEG_INFINITY);

    assert_eq!(v.iter().log_sum_exp()?, LogProb::new(f64::NEG_INFINITY)?);
    assert_eq!(v.iter().log_sum_exp_float(), f64::NEG_INFINITY);
    assert_eq!(
        v.iter().log_sum_exp_clamped(),
        LogProb::new(f64::NEG_INFINITY)?
    );
    Ok(())
}
