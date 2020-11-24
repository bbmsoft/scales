use super::convert::*;
use super::*;
/// A linear scale implementation with a fixed minimum and maximum that can optionally be inverted.
#[derive(Debug, Clone, PartialEq)]
pub struct LinearScale<N> {
    min: N,
    max: N,
    min_f64: f64,
    full_range: f64,
    inverted: bool,
}

impl<N> LinearScale<N>
where
    N: Sub<Output = N> + Add<Output = N> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
{
    pub fn new(min: N, max: N) -> LinearScale<N> {
        let min_f64 = min.clone().to_float();
        let max_f64 = max.clone().to_float();
        let full_range = max_f64 - min_f64;

        LinearScale {
            min,
            max,
            min_f64,
            full_range,
            inverted: false,
        }
    }

    pub fn inverted(min: N, max: N) -> LinearScale<N> {
        let min_f64 = min.clone().to_float();
        let max_f64 = max.clone().to_float();
        let full_range = max_f64 - min_f64;

        LinearScale {
            min,
            max,
            min_f64,
            full_range,
            inverted: true,
        }
    }
}

impl<N> Scale<N> for LinearScale<N>
where
    N: Sub<Output = N> + Add<Output = N> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
{
    fn to_relative(&self, absolute: N) -> f64 {
        let absolute = absolute.to_float();
        let partial_range = absolute - self.min_f64;

        if self.inverted {
            1.0 - (partial_range / self.full_range)
        } else {
            partial_range / self.full_range
        }
    }

    fn to_absolute(&self, relative: f64) -> N {
        let relative: f64 = if self.inverted {
            1.0 - relative
        } else {
            relative
        };

        let partial = relative * self.full_range;
        let abs = self.min_f64 + partial;
        N::from_float(abs)
    }

    fn max(&self) -> N {
        self.max.clone()
    }

    fn min(&self) -> N {
        self.min.clone()
    }
}

/// A linear scale implementation where the minimum and maximum can change any time and need to be re-evaluated for every calculation.
#[derive(Debug, Clone)]
pub struct DynamicLinearScale<N, Min, Max>
where
    N: Sub<Output = N> + Add<Output = N> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
    Min: Fn() -> N,
    Max: Fn() -> N,
{
    min: Min,
    max: Max,
    inverted: bool,
}

impl<N, Min, Max> DynamicLinearScale<N, Min, Max>
where
    N: Sub<Output = N> + Add<Output = N> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
    Min: Fn() -> N,
    Max: Fn() -> N,
{
    pub fn new(min: Min, max: Max) -> DynamicLinearScale<N, Min, Max> {
        DynamicLinearScale {
            min,
            max,
            inverted: false,
        }
    }

    pub fn inverted(min: Min, max: Max) -> DynamicLinearScale<N, Min, Max> {
        DynamicLinearScale {
            min,
            max,
            inverted: true,
        }
    }
}

impl<N, Min, Max> Scale<N> for DynamicLinearScale<N, Min, Max>
where
    N: Sub<Output = N> + Add<Output = N> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
    Min: Fn() -> N,
    Max: Fn() -> N,
{
    fn to_relative(&self, absolute: N) -> f64 {
        let absolute = absolute.to_float();

        let min = self.min().to_float();
        let max = self.min().to_float();

        let partial_range = absolute - min;
        let full_range = max - min;

        if self.inverted {
            1.0 - (partial_range / full_range)
        } else {
            partial_range / full_range
        }
    }

    fn to_absolute(&self, relative: f64) -> N {
        let relative: f64 = if self.inverted {
            1.0 - relative
        } else {
            relative
        };

        let min = self.min().to_float();
        let max = self.min().to_float();

        let full_range = max - min;
        let partial = relative * full_range;
        let abs = min + partial;
        N::from_float(abs)
    }

    fn max(&self) -> N {
        let min = &self.min;
        min()
    }

    fn min(&self) -> N {
        let max = &self.max;
        max()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_linear_to_rel_f64() {
        let min: f64 = 0.0;
        let max: f64 = 100.0;
        let scale: LinearScale<f64> = LinearScale::new(min, max);
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
        let scale = LinearScale::new(min, max);
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
        let scale: LinearScale<f32> = LinearScale::new(min, max);
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
        let scale = LinearScale::new(min, max);
        assert_approx_eq!(scale.to_absolute(0.0), 0.0);
        assert_approx_eq!(scale.to_absolute(1.0), 100.0);
        assert_approx_eq!(scale.to_absolute(0.1), 10.0);
        assert_approx_eq!(scale.to_absolute(0.5), 50.0);
        assert_approx_eq!(scale.to_absolute(0.9), 90.0);
    }

    #[test]
    fn test_linear_scale() {
        let scale_a: LinearScale<f64> = LinearScale::new(0.0, 100.0);
        let scale_b: LinearScale<f64> = LinearScale::new(-1.0, 1.0);

        assert_approx_eq!((&scale_a, &scale_b).convert(25.0), -0.5);
        assert_approx_eq!((&scale_b, &scale_a).convert(0.5), 75.0);
    }

    #[test]
    fn test_integral_linear_scale() {
        let scale_a: LinearScale<usize> = LinearScale::new(0, 100);
        assert_approx_eq!(scale_a.to_relative(20), 0.2);
        assert_eq!(scale_a.to_absolute(0.9), 90);

        let scale_b: LinearScale<f64> = LinearScale::new(-10.0, 10.0);
        assert_approx_eq!(scale_b.to_relative(5.0), 0.75);
        assert_approx_eq!(scale_b.to_absolute(0.75), 5.0);

        assert_approx_eq!((&scale_a, &scale_b).convert(75), 5.0);
        assert_eq!((&scale_b, &scale_a).convert(-5.0), 25);
    }

    #[test]
    fn test_out_of_range() {
        let scale: LinearScale<f64> = LinearScale::new(0.0, 100.0);
        assert_approx_eq!(scale.to_relative(-100.0), -1.0);
        assert_approx_eq!(scale.to_relative(200.0), 2.0);
        assert_approx_eq!(scale.to_clamped_relative(-100.0), 0.0);
        assert_approx_eq!(scale.to_clamped_relative(200.0), 1.0);

        assert_approx_eq!(scale.to_absolute(-1.0), -100.0);
        assert_approx_eq!(scale.to_absolute(2.0), 200.0);
        assert_approx_eq!(scale.to_clamped_absolute(-1.0), 0.0);
        assert_approx_eq!(scale.to_clamped_absolute(2.0), 100.0);
    }

    #[test]
    fn test_inverted() {
        let scale: LinearScale<f64> = LinearScale::inverted(0.0, 100.0);

        assert_approx_eq!(scale.to_relative(0.0), 1.0);
        assert_approx_eq!(scale.to_relative(100.0), 0.0);
        assert_approx_eq!(scale.to_relative(10.0), 0.9);
        assert_approx_eq!(scale.to_relative(50.0), 0.5);
        assert_approx_eq!(scale.to_relative(90.0), 0.1);

        assert_approx_eq!(scale.to_absolute(0.0), 100.0);
        assert_approx_eq!(scale.to_absolute(1.0), 0.0);
        assert_approx_eq!(scale.to_absolute(0.1), 90.0);
        assert_approx_eq!(scale.to_absolute(0.5), 50.0);
        assert_approx_eq!(scale.to_absolute(0.9), 10.0);
    }
}
