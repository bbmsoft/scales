use super::convert::*;
use super::linear::*;
use super::*;

#[derive(Debug, Clone)]
pub struct LogarithmicScale<N> {
    min: N,
    max: N,
    linear_delegate: LinearScale<N>,
}

impl<N> LogarithmicScale<N>
where
    N: Sub<Output = N> + Add<Output = N> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
{
    pub fn new(min: N, max: N) -> LogarithmicScale<N> {
        LogarithmicScale {
            min: min.clone(),
            max: max.clone(),
            linear_delegate: LinearScale::new(apply_to(min, f64::log10), apply_to(max, f64::log10)),
        }
    }
    pub fn inverted(min: N, max: N) -> LogarithmicScale<N> {
        LogarithmicScale {
            min: min.clone(),
            max: max.clone(),
            linear_delegate: LinearScale::inverted(
                apply_to(min, f64::log10),
                apply_to(max, f64::log10),
            ),
        }
    }
}

impl<N> Scale<N> for LogarithmicScale<N>
where
    N: Sub<Output = N> + Add<Output = N> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
{
    fn to_relative(&self, absolute: N) -> f64 {
        let abs_log = apply_to(absolute, f64::log10);
        self.linear_delegate.to_relative(abs_log)
    }

    fn to_absolute(&self, relative: f64) -> N {
        let abs_log = self.linear_delegate.to_absolute(relative);
        apply_to(abs_log, |f| 10f64.powf(f))
    }

    fn max(&self) -> N {
        self.max.clone()
    }

    fn min(&self) -> N {
        self.min.clone()
    }
}

fn apply_to<N>(n: N, fun: impl Fn(f64) -> f64) -> N
where
    N: ToFloat<f64> + FromFloat<f64>,
{
    N::from_float(fun(n.to_float()))
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::time::*;

    #[test]
    fn test_log() {
        let scale: LogarithmicScale<f64> = LogarithmicScale::new(10.0, 10240.0);
        assert_approx_eq!(scale.to_absolute(0.0), 10.0);
        assert_approx_eq!(scale.to_absolute(0.1), 20.0);
        assert_approx_eq!(scale.to_absolute(0.2), 40.0);
        assert_approx_eq!(scale.to_absolute(0.3), 80.0);
        assert_approx_eq!(scale.to_absolute(0.4), 160.0);
        assert_approx_eq!(scale.to_absolute(0.5), 320.0);
        assert_approx_eq!(scale.to_absolute(0.6), 640.0);
        assert_approx_eq!(scale.to_absolute(0.7), 1280.0);
        assert_approx_eq!(scale.to_absolute(0.8), 2560.0);
        assert_approx_eq!(scale.to_absolute(0.9), 5120.0);
        assert_approx_eq!(scale.to_absolute(1.0), 10240.0);

        assert_approx_eq!(scale.to_relative(10.0), 0.0);
        assert_approx_eq!(scale.to_relative(20.0), 0.1);
        assert_approx_eq!(scale.to_relative(40.0), 0.2);
        assert_approx_eq!(scale.to_relative(80.0), 0.3);
        assert_approx_eq!(scale.to_relative(160.0), 0.4);
        assert_approx_eq!(scale.to_relative(320.0), 0.5);
        assert_approx_eq!(scale.to_relative(640.0), 0.6);
        assert_approx_eq!(scale.to_relative(1280.0), 0.7);
        assert_approx_eq!(scale.to_relative(2560.0), 0.8);
        assert_approx_eq!(scale.to_relative(5120.0), 0.9);
        assert_approx_eq!(scale.to_relative(10240.0), 1.0);
    }

    #[test]
    fn test_log_inverted() {
        let scale: LogarithmicScale<f64> = LogarithmicScale::inverted(10.0, 10240.0);
        assert_approx_eq!(scale.to_absolute(0.0), 10240.0);
        assert_approx_eq!(scale.to_absolute(0.1), 5120.0);
        assert_approx_eq!(scale.to_absolute(0.2), 2560.0);
        assert_approx_eq!(scale.to_absolute(0.3), 1280.0);
        assert_approx_eq!(scale.to_absolute(0.4), 640.0);
        assert_approx_eq!(scale.to_absolute(0.5), 320.0);
        assert_approx_eq!(scale.to_absolute(0.6), 160.0);
        assert_approx_eq!(scale.to_absolute(0.7), 80.0);
        assert_approx_eq!(scale.to_absolute(0.8), 40.0);
        assert_approx_eq!(scale.to_absolute(0.9), 20.0);
        assert_approx_eq!(scale.to_absolute(1.0), 10.0);

        assert_approx_eq!(scale.to_relative(10.0), 1.0);
        assert_approx_eq!(scale.to_relative(20.0), 0.9);
        assert_approx_eq!(scale.to_relative(40.0), 0.8);
        assert_approx_eq!(scale.to_relative(80.0), 0.7);
        assert_approx_eq!(scale.to_relative(160.0), 0.6);
        assert_approx_eq!(scale.to_relative(320.0), 0.5);
        assert_approx_eq!(scale.to_relative(640.0), 0.4);
        assert_approx_eq!(scale.to_relative(1280.0), 0.3);
        assert_approx_eq!(scale.to_relative(2560.0), 0.2);
        assert_approx_eq!(scale.to_relative(5120.0), 0.1);
        assert_approx_eq!(scale.to_relative(10240.0), 0.0);
    }

    #[test]
    fn test_log_out_of_bounds() {
        let scale: LogarithmicScale<f64> = LogarithmicScale::new(10.0, 10240.0);
        assert_approx_eq!(scale.to_absolute(-0.1), 5.0);
        assert_approx_eq!(scale.to_absolute(-1.0), 0.0097656);
        assert_approx_eq!(scale.to_absolute(-2.0), 0.0000095);
        assert_approx_eq!(scale.to_absolute(1.1), 20480.0);

        assert_approx_eq!(scale.to_relative(1.0), -0.3321928);
        assert!(scale.to_relative(-1.0).is_nan());

        let neg_inf = scale.to_relative(0.0);
        assert!(neg_inf.is_infinite() && neg_inf.is_sign_negative());

        assert_approx_eq!(scale.to_clamped_absolute(0.0), 10.0);
        assert_approx_eq!(scale.to_clamped_absolute(-1.0), 10.0);
        assert_approx_eq!(scale.to_clamped_absolute(1.1), 10240.0);

        assert_approx_eq!(scale.to_clamped_relative(1.0), 0.0);
        assert_approx_eq!(scale.to_clamped_relative(0.0), 0.0);
        assert_approx_eq!(scale.to_clamped_relative(-1.0), 0.0);
        assert_approx_eq!(scale.to_clamped_relative(20240.0), 1.0);
    }

    // #[test]
    fn _benchmark() {
        let loops = 100_000_000;

        let upper = loops as f64;
        let step = 1.0 / upper;

        // reference run

        let mut results = Vec::new();
        let min = 10f64.log10();
        let max = 1_000f64.log10();
        let range = max - min;

        let start = Instant::now();
        for i in 0..loops {
            let relative = i as f64 * step;
            results.push(10f64.powf(min + relative * range));
        }
        let duration = start.elapsed();

        let sample: Vec<&f64> = results.iter().take(10).collect();

        eprintln!("{}", duration.as_millis());
        eprintln!("{:?}", sample);

        // actual run

        let mut results = Vec::new();
        let scale: LogarithmicScale<f64> = LogarithmicScale::new(10.0, 1_000.0);

        let start = Instant::now();
        for i in 0..loops {
            let relative = i as f64 * step;
            let result = scale.to_absolute(relative);
            results.push(result);
        }
        let duration = start.elapsed();

        let sample: Vec<&f64> = results.iter().take(10).collect();

        eprintln!("{}", duration.as_millis());
        eprintln!("{:?}", sample);
    }
}
