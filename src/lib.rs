pub mod convert;
pub mod linear;
pub mod logarithmic;

use convert::*;
use std::ops::*;

pub trait Scale<N>
where
    N: Sub<Output = N> + Add<Output = N> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
{
    fn to_relative(&self, absolute: N) -> f64;
    fn to_absolute(&self, relative: f64) -> N;
    fn max(&self) -> N;
    fn min(&self) -> N;

    fn to_clamped_relative(&self, absolute: N) -> f64 {
        let absolute = if absolute > self.max() {
            self.max()
        } else if absolute < self.min() {
            self.min()
        } else {
            absolute
        };

        self.to_relative(absolute)
    }

    fn to_clamped_absolute(&self, relative: f64) -> N {
        let relative: f64 = relative.to_float();
        let relative = if relative > 1.0 {
            1.0
        } else if relative < 0.0 {
            0.0
        } else {
            relative
        };

        self.to_absolute(relative)
    }

    fn to_relative_delta(&self, absolute_delta: N, relative_pos: f64) -> f64 {
        let absolute_pos = self.to_absolute(relative_pos.clone());
        let rel_pos_out = self.to_relative(absolute_pos + absolute_delta);
        rel_pos_out - relative_pos
    }

    fn to_absolute_delta(&self, relative_delta: f64, absolute_pos: N) -> N {
        let relative_pos = self.to_relative(absolute_pos.clone());
        let abs_pos_out = self.to_absolute(relative_pos + relative_delta);
        abs_pos_out - absolute_pos
    }

    fn convert<T>(&self, absolute: N, to_other: &impl Scale<T>) -> T
    where
        T: Sub<Output = T> + Add<Output = T> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
    {
        let rel = self.to_relative(absolute);
        to_other.to_absolute(rel)
    }
}

pub trait Converter<I, E>
where
    E: Sub<Output = E> + Add<Output = E> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
    I: Sub<Output = I> + Add<Output = I> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
{
    fn to_internal(&self, external_value: E) -> I;
    fn to_external(&self, internal_value: I) -> E;
}

impl<I, E, SI, SE> Converter<I, E> for (SE, SI)
where
    E: Sub<Output = E> + Add<Output = E> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
    I: Sub<Output = I> + Add<Output = I> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
    SI: Scale<I>,
    SE: Scale<E>,
{
    fn to_internal(&self, external_value: E) -> I {
        let external = &self.0;
        let internal = &self.1;
        external.convert(external_value, internal)
    }

    fn to_external(&self, internal_value: I) -> E {
        let external = &self.0;
        let internal = &self.1;
        internal.convert(internal_value, external)
    }
}

#[cfg(test)]
#[macro_use]
extern crate assert_approx_eq;

#[cfg(test)]
mod test {

    use super::linear::*;
    use super::logarithmic::*;
    use super::*;

    #[test]
    fn test_converter() {
        let converter = (
            LinearScale::new(0.0, 100.0),
            LogarithmicScale::new(20.0, 24_000.0),
        );

        assert_approx_eq!(converter.to_internal(0.0), 20f64);
        assert_approx_eq!(converter.to_internal(100.0), 24_000f64);
    }
}
