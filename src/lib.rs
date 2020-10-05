pub mod convert;
pub mod linear;
pub mod logarithmic;

use convert::*;
use std::ops::*;

pub trait Scale<N, F>
where
    N: Sub<Output = N> + Add<Output = N> + PartialOrd + FromFloat<F> + ToFloat<F> + Clone,
    F: FromFloat<f64> + ToFloat<f64> + Clone,
{
    fn to_relative(&self, absolute: N) -> F;
    fn to_absolute(&self, relative: F) -> N;
    fn get_max(&self) -> N;
    fn get_min(&self) -> N;

    fn to_clamped_relative(&self, absolute: N) -> F {
        let absolute = if absolute > self.get_max() {
            self.get_max()
        } else if absolute < self.get_min() {
            self.get_min()
        } else {
            absolute
        };

        self.to_relative(absolute)
    }

    fn to_clamped_absolute(&self, relative: F) -> N {
        let relative: f64 = relative.to_float();
        let relative = if relative > 1.0 {
            1.0
        } else if relative < 0.0 {
            0.0
        } else {
            relative
        };

        self.to_absolute(F::from_float(relative))
    }

    fn to_relative_delta(&self, absolute_delta: N, relative_pos: F) -> F {
        let absolute_pos = self.to_absolute(relative_pos.clone());
        let rel_pos_out = self.to_relative(absolute_pos + absolute_delta);
        F::from_float(rel_pos_out.to_float() - relative_pos.to_float())
    }

    fn to_absolute_delta(&self, relative_delta: F, absolute_pos: N) -> N {
        let relative_pos = self.to_relative(absolute_pos.clone());
        let abs_pos_out = self.to_absolute(F::from_float(
            relative_pos.to_float() + relative_delta.to_float(),
        ));
        abs_pos_out - absolute_pos
    }

    fn convert<T>(&self, absolute: N, to_other: &impl Scale<T, F>) -> T
    where
        T: Sub<Output = T> + Add<Output = T> + PartialOrd + FromFloat<F> + ToFloat<F> + Clone,
    {
        let rel = self.to_relative(absolute);
        to_other.to_absolute(rel)
    }
}

#[cfg(test)]
#[macro_use]
extern crate assert_approx_eq;
