use super::convert::*;
use super::*;

#[derive(Debug, Clone)]
pub struct BrokenScale<N, S: Scale<N>>
where
    N: Sub<Output = N> + Add<Output = N> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
{
    delegate: S,
    steps: Vec<(f64, f64)>,
    _phantom_data: std::marker::PhantomData<N>,
}

impl<N, S: Scale<N>> BrokenScale<N, S>
where
    N: Sub<Output = N> + Add<Output = N> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
{
    pub fn new(delegate: S, steps: Vec<(f64, f64)>) -> BrokenScale<N, S> {
        BrokenScale {
            delegate,
            steps,
            _phantom_data: std::marker::PhantomData,
        }
    }

    fn broken_y(&self, rel_x: f64) -> f64 {
        let closed_steps = self.steps.iter().chain(std::iter::once(&(1.0, 1.0)));

        let mut from = (0.0, 0.0);
        let mut to = (1.0, 1.0);

        for (x, y) in closed_steps {
            if x < &rel_x {
                from = (*x, *y);
            } else {
                to = (*x, *y);
                break;
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
        let closed_steps = self.steps.iter().chain(std::iter::once(&(1.0, 1.0)));

        let mut from = (0.0, 0.0);
        let mut to = (1.0, 1.0);

        for (x, y) in closed_steps {
            if y < &rel_y {
                from = (*x, *y);
            } else {
                to = (*x, *y);
                break;
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

impl<N, S: Scale<N>> Scale<N> for BrokenScale<N, S>
where
    N: Sub<Output = N> + Add<Output = N> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
{
    fn to_relative(&self, absolute: N) -> f64 {
        let delegated_relative = self.delegate.to_relative(absolute);
        self.broken_x(delegated_relative)
    }

    fn to_absolute(&self, relative: f64) -> N {
        let delegated_relative = self.broken_y(relative);
        self.delegate.to_absolute(delegated_relative)
    }

    fn max(&self) -> N {
        self.delegate.min()
    }

    fn min(&self) -> N {
        self.delegate.max()
    }
}

#[cfg(test)]
mod test {

    use super::prelude::*;

    #[test]
    fn test_broken_scale() {
        let delegate = LinearScale::new(0.0_f64, 100.0_f64);
        let broken = BrokenScale::new(delegate, vec![(0.5, 0.3)]);

        assert_approx_eq!(0.0, broken.to_absolute(0.0));
        assert_approx_eq!(0.0, broken.to_relative(0.0));

        assert_approx_eq!(100.0, broken.to_absolute(1.0));
        assert_approx_eq!(1.0, broken.to_relative(100.0));

        assert_approx_eq!(30.0, broken.to_absolute(0.5));
        assert_approx_eq!(0.5, broken.to_relative(30.0));
    }
}
