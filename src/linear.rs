use super::convert::*;
use super::*;

pub struct LinearScale<N, F> {
    min: N,
    max: N,
    min_f64: f64,
    full_range: f64,
    rasterizer: Option<Box<dyn Fn(N) -> N>>,
    _phantom: std::marker::PhantomData<F>,
}

impl<N, F> LinearScale<N, F>
where
    N: Sub<Output = N> + Add<Output = N> + PartialOrd + FromFloat<F> + ToFloat<F> + Clone,
    F: FromFloat<f64> + ToFloat<f64> + Clone,
{
    pub fn with_min_max(min: N, max: N) -> LinearScale<N, F> {
        let min_f64 = to_f64(min.clone());
        let max_f64 = to_f64(max.clone());
        let full_range = max_f64 - min_f64;

        LinearScale {
            min,
            max,
            min_f64,
            full_range,
            rasterizer: None,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn with_min_max_and_rasterizer(
        min: N,
        max: N,
        rasterizer: impl Fn(N) -> N + 'static,
    ) -> LinearScale<N, F> {
        let min_f64 = to_f64(min.clone());
        let max_f64 = to_f64(max.clone());
        let full_range = max_f64 - min_f64;

        LinearScale {
            min,
            max,
            min_f64,
            full_range,
            rasterizer: Some(Box::new(rasterizer)),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<N, F> Scale<N, F> for LinearScale<N, F>
where
    N: Sub<Output = N> + Add<Output = N> + PartialOrd + FromFloat<F> + ToFloat<F> + Clone,
    F: FromFloat<f64> + ToFloat<f64> + Clone,
{
    fn to_relative(&self, absolute: N) -> F {
        let absolute = if let Some(rasterizer) = self.rasterizer.as_ref() {
            rasterizer(absolute)
        } else {
            absolute
        };
        let absolute: f64 = to_f64(absolute);
        let partial_range = absolute - self.min_f64;
        F::from_float(partial_range / self.full_range)
    }

    fn to_absolute(&self, relative: F) -> N {
        let relative: f64 = relative.to_float();

        let partial = relative * self.full_range;
        let abs = self.min_f64 + partial;
        let abs: N = from_f64(abs);
        if let Some(rasterizer) = self.rasterizer.as_ref() {
            rasterizer(abs)
        } else {
            abs
        }
    }

    fn get_max(&self) -> N {
        self.max.clone()
    }

    fn get_min(&self) -> N {
        self.min.clone()
    }
}

fn to_f64<N, F>(n: N) -> f64
where
    N: ToFloat<F>,
    F: ToFloat<f64>,
{
    n.to_float().to_float()
}

fn from_f64<N, F>(f: f64) -> N
where
    N: FromFloat<F>,
    F: FromFloat<f64>,
{
    N::from_float(F::from_float(f))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_linear_to_rel_f64() {
        let min: f64 = 0.0;
        let max: f64 = 100.0;
        let scale: LinearScale<f64, f64> = LinearScale::with_min_max(min, max);
        assert_approx_eq!(scale.to_relative(0.0), 0.0);
        assert_approx_eq!(scale.to_relative(100.0), 1.0);
        assert_approx_eq!(scale.to_relative(10.0), 0.1);
        assert_approx_eq!(scale.to_relative(50.0), 0.5);
        assert_approx_eq!(scale.to_relative(90.0), 0.9);
    }

    #[test]
    fn test_linear_to_abs_f64() {
        let min: f64 = 0.0;
        let max: f64 = 100.0;
        let scale = LinearScale::with_min_max(min, max);
        assert_approx_eq!(scale.to_absolute(0.0), 0.0);
        assert_approx_eq!(scale.to_absolute(1.0), 100.0);
        assert_approx_eq!(scale.to_absolute(0.1), 10.0);
        assert_approx_eq!(scale.to_absolute(0.5), 50.0);
        assert_approx_eq!(scale.to_absolute(0.9), 90.0);
    }

    #[test]
    fn test_linear_to_rel_f32() {
        let min: f32 = 0.0;
        let max: f32 = 100.0;
        let scale: LinearScale<f32, f32> = LinearScale::with_min_max(min, max);
        assert_approx_eq!(scale.to_relative(0.0), 0.0);
        assert_approx_eq!(scale.to_relative(100.0), 1.0);
        assert_approx_eq!(scale.to_relative(10.0), 0.1);
        assert_approx_eq!(scale.to_relative(50.0), 0.5);
        assert_approx_eq!(scale.to_relative(90.0), 0.9);
    }

    #[test]
    fn test_linear_to_abs_f32() {
        let min: f32 = 0.0;
        let max: f32 = 100.0;
        let scale = LinearScale::with_min_max(min, max);
        assert_approx_eq!(scale.to_absolute(0.0), 0.0);
        assert_approx_eq!(scale.to_absolute(1.0), 100.0);
        assert_approx_eq!(scale.to_absolute(0.1), 10.0);
        assert_approx_eq!(scale.to_absolute(0.5), 50.0);
        assert_approx_eq!(scale.to_absolute(0.9), 90.0);
    }

    #[test]
    fn test_linear_scale() {
        let scale_a: LinearScale<f64, f64> = LinearScale::with_min_max(0.0, 100.0);
        let scale_b: LinearScale<f64, f64> = LinearScale::with_min_max(-1.0, 1.0);

        assert_approx_eq!(scale_a.convert(25.0, &scale_b), -0.5);
        assert_approx_eq!(scale_b.convert(0.5, &scale_a), 75.0);
    }

    #[test]
    fn test_rasterizer() {
        let min: f32 = 0.0;
        let max: f32 = 1_000.0;
        let step = 10.0;
        let scale: LinearScale<f32, f32> =
            LinearScale::with_min_max_and_rasterizer(min, max, move |u| (u / step).round() * step);
        assert_approx_eq!(scale.to_absolute(0.085), 90.0);
        assert_approx_eq!(scale.to_relative(85.0), 0.09);
    }

    #[test]
    fn test_integral_linear_scale() {
        let scale_a: LinearScale<usize, f32> = LinearScale::with_min_max(0, 100);
        assert_approx_eq!(scale_a.to_relative(20), 0.2);
        assert_eq!(scale_a.to_absolute(0.9), 90);

        let scale_b: LinearScale<f64, f32> = LinearScale::with_min_max(-10.0, 10.0);
        assert_approx_eq!(scale_b.to_relative(5.0), 0.75);
        assert_approx_eq!(scale_b.to_absolute(0.75), 5.0);

        assert_approx_eq!(scale_a.convert(75, &scale_b), 5.0);
        assert_eq!(scale_b.convert(-5.0, &scale_a), 25);
    }

    #[test]
    fn test_out_of_range() {
        let scale: LinearScale<f64, f64> = LinearScale::with_min_max(0.0, 100.0);
        assert_approx_eq!(scale.to_relative(-100.0), -1.0);
        assert_approx_eq!(scale.to_relative(200.0), 2.0);
        assert_approx_eq!(scale.to_clamped_relative(-100.0), 0.0);
        assert_approx_eq!(scale.to_clamped_relative(200.0), 1.0);

        assert_approx_eq!(scale.to_absolute(-1.0), -100.0);
        assert_approx_eq!(scale.to_absolute(2.0), 200.0);
        assert_approx_eq!(scale.to_clamped_absolute(-1.0), 0.0);
        assert_approx_eq!(scale.to_clamped_absolute(2.0), 100.0);
    }
}
