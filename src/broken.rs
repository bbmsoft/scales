use super::convert::*;
use super::*;
use crate::linear::*;

#[derive(Debug, Clone, PartialEq)]
pub struct BrokenScale<N>
where
    N: Sub<Output = N> + Add<Output = N> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
{
    delegate: LinearScale<N>,
    steps: Vec<(f64, f64)>,
}

impl<N> BrokenScale<N>
where
    N: Sub<Output = N> + Add<Output = N> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
{
    pub fn new(min: N, max: N, steps: &[(N, f64)]) -> BrokenScale<N> {
        let delegate = LinearScale::new(min, max);
        let steps = steps
            .iter()
            .map(|(abs, rel)| (delegate.to_relative(abs.clone()), *rel))
            .collect();
        BrokenScale { delegate, steps }
    }

    fn broken_y(&self, rel_x: f64) -> f64 {
        let mut from = (0.0, 0.0);
        let mut to = (1.0, 1.0);

        if rel_x >= 1.0 {
            if let Some((x, y)) = self.steps.iter().last() {
                from = (*x, *y);
            }
        } else {
            let closed_steps = self.steps.iter().chain(std::iter::once(&(1.0, 1.0)));

            for (x, y) in closed_steps {
                if x < &rel_x {
                    from = (*x, *y);
                } else {
                    to = (*x, *y);
                    break;
                }
            }
        }

        // y = m * x + t
        // m = dy/dx
        // t = y - m * x

        let dx = to.0 - from.0;
        let dy = to.1 - from.1;
        let m = dy / dx;
        let t = from.1 - m * from.0;

        m * rel_x + t
    }

    fn broken_x(&self, rel_y: f64) -> f64 {
        let mut from = (0.0, 0.0);
        let mut to = (1.0, 1.0);

        if rel_y >= 1.0 {
            if let Some((x, y)) = self.steps.iter().last() {
                from = (*x, *y);
            }
        } else {
            let closed_steps = self.steps.iter().chain(std::iter::once(&(1.0, 1.0)));

            for (x, y) in closed_steps {
                if y < &rel_y {
                    from = (*x, *y);
                } else {
                    to = (*x, *y);
                    break;
                }
            }
        }

        // y = m * x + t
        // m = dy/dx
        // t = y - m * x
        // x = (y - t) / m

        let dx = to.0 - from.0;
        let dy = to.1 - from.1;
        let m = dy / dx;
        let t = from.1 - m * from.0;

        (rel_y - t) / m
    }
}

impl<N> Scale<N> for BrokenScale<N>
where
    N: Sub<Output = N> + Add<Output = N> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
{
    fn to_relative(&self, absolute: N) -> f64 {
        let delegated_relative = dbg!(self.delegate.to_relative(absolute));
        self.broken_y(delegated_relative)
    }

    fn to_absolute(&self, relative: f64) -> N {
        let delegated_relative = self.broken_x(relative);
        self.delegate.to_absolute(delegated_relative)
    }

    fn max(&self) -> N {
        self.delegate.max()
    }

    fn min(&self) -> N {
        self.delegate.min()
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;
    use assert_approx_eq::*;

    #[test]
    fn test_broken_scale() {
        let broken = BrokenScale::new(-120_f64, 12_f64, &vec![]);

        assert_approx_eq!(-120.0, broken.to_absolute(0.0));
        assert_approx_eq!(0.0, broken.to_relative(-120.0));

        assert_approx_eq!(12.0, broken.to_absolute(1.0));
        assert_approx_eq!(1.0, broken.to_relative(12.0));

        assert_approx_eq!(-54.0, broken.to_absolute(0.5));
        assert_approx_eq!(0.5, broken.to_relative(-54.0));
    }

    #[test]
    fn test_broken_scale_converter() {
        let broken = BrokenScale::new(-120_f64, 12_f64, &vec![]);
        let linear = LinearScale::inverted(100_f64, 200_f64);
        let conv = (linear, broken);

        assert_approx_eq!(100.0, conv.convert_back(12.0));
        assert_approx_eq!(12.0, conv.convert(100.0));

        assert_approx_eq!(200.0, conv.convert_back(-120.0));
        assert_approx_eq!(-120.0, conv.convert(200.0));

        assert_approx_eq!(150.0, conv.convert_back(-54.0));
        assert_approx_eq!(-54.0, conv.convert(150.0));
    }

    #[test]
    fn test_broken_scale_converter_add() {
        let broken = BrokenScale::new(-120_f64, 12_f64, &vec![]);
        let linear = LinearScale::inverted(100_f64, 200_f64);
        let conv = (linear, broken);

        let d_lin = -10.0;
        let d_broke = conv.add_external(d_lin, -54.0);

        assert_approx_eq!(-40.8, d_broke);
    }

    #[test]
    fn test_broken_scale_converter_add_clamped() {
        let broken = BrokenScale::new(-120_f64, 12_f64, &vec![]);
        let linear = LinearScale::inverted(100_f64, 200_f64);
        let conv = (linear, broken);

        let d_lin = -10.0;
        let d_broke = conv.add_external_clamped(d_lin, -120.0);

        assert_approx_eq!(-106.8, d_broke);
    }

    #[test]
    fn test_broken_scale_converter_add_clamped_lower_bound() {
        let broken = BrokenScale::new(-120_f64, 12_f64, &vec![]);
        let linear = LinearScale::inverted(100_f64, 200_f64);
        let conv = (linear, broken);

        let d_lin = 10.0;
        let d_broke = conv.add_external_clamped(d_lin, -120.0);

        assert_approx_eq!(-120.0, d_broke);
    }

    #[test]
    fn test_broken_scale_converter_add_clamped_upper_bound() {
        let broken = BrokenScale::new(-120_f64, 12_f64, &vec![]);
        let linear = LinearScale::inverted(100_f64, 200_f64);
        let conv = (linear, broken);

        let d_lin = -10.0;
        let d_broke = conv.add_external_clamped(d_lin, 12.0);

        assert_approx_eq!(12.0, d_broke);
    }
}
