use super::convert::*;
use super::Scale;
use std::cmp::Ordering;
use std::ops::*;

pub trait Converter<E, I>
where
    E: Sub<Output = E> + Add<Output = E> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
    I: Sub<Output = I> + Add<Output = I> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
{
    fn convert(&self, external_value: E) -> I;
    fn convert_back(&self, internal_value: I) -> E;

    fn add_external(&self, external_delta: E, internal_value: I) -> I {
        let external_value = self.convert_back(internal_value);
        let new_internal_value = self.convert(external_value + external_delta);
        new_internal_value
    }

    fn add_internal(&self, internal_delta: I, external_value: E) -> E {
        let internal_value = self.convert(external_value);
        let new_external_value = self.convert_back(internal_value + internal_delta);
        new_external_value
    }
}

pub trait ClampingConverter<E, I>: Converter<E, I>
where
    E: Sub<Output = E> + Add<Output = E> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
    I: Sub<Output = I> + Add<Output = I> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
{
    fn external_max(&self) -> E;
    fn external_min(&self) -> E;
    fn internal_max(&self) -> I;
    fn internal_min(&self) -> I;

    fn add_external_clamped(&self, external_delta: E, internal_value: I) -> I {
        let min = self.internal_min();
        let max = self.internal_max();
        let val = self.add_external(external_delta, internal_value);

        match min.partial_cmp(&val) {
            Some(Ordering::Greater) => min,
            _ => match max.partial_cmp(&val) {
                Some(Ordering::Less) => max,
                _ => val,
            },
        }
    }

    fn add_internal_clamped(&self, internal_delta: I, external_value: E) -> E {
        let min = self.external_min();
        let max = self.external_max();
        let val = self.add_internal(internal_delta, external_value);

        match min.partial_cmp(&val) {
            Some(Ordering::Greater) => min,
            _ => match max.partial_cmp(&val) {
                Some(Ordering::Less) => max,
                _ => val,
            },
        }
    }
}

impl<E, I, SE, SI> Converter<E, I> for (SE, SI)
where
    E: Sub<Output = E> + Add<Output = E> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
    I: Sub<Output = I> + Add<Output = I> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
    SE: Scale<E>,
    SI: Scale<I>,
{
    fn convert(&self, external_value: E) -> I {
        let external = &self.0;
        let internal = &self.1;
        let rel = external.to_relative(external_value);
        internal.to_absolute(rel)
    }

    fn convert_back(&self, internal_value: I) -> E {
        let external = &self.0;
        let internal = &self.1;
        let rel = internal.to_relative(internal_value);
        external.to_absolute(rel)
    }
}

impl<E, I, SE, SI> ClampingConverter<E, I> for (SE, SI)
where
    E: Sub<Output = E> + Add<Output = E> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
    I: Sub<Output = I> + Add<Output = I> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
    SE: Scale<E>,
    SI: Scale<I>,
{
    fn external_max(&self) -> E {
        self.0.max()
    }

    fn external_min(&self) -> E {
        self.0.min()
    }

    fn internal_max(&self) -> I {
        self.1.max()
    }

    fn internal_min(&self) -> I {
        self.1.min()
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;
    use assert_approx_eq::*;

    #[test]
    fn test_converter() {
        let lin = LinearScale::new(0.0, 100.0);
        let log = LogarithmicScale::new(20.0, 24_000.0);

        assert_approx_eq!((&lin, &log).convert(0.0), 20f64);
        assert_approx_eq!((lin, log).convert(100.0), 24_000f64);
    }

    #[test]
    fn example_from_readme() {
        let slider = Slider;
        let parameter = Parameter;

        let relative = (slider.value() - slider.min()) / (slider.max() - slider.min());
        let log_range = parameter.max().log10() - parameter.min().log10();
        let exp = parameter.min().log10() + relative * log_range;
        let new_value = 10f64.powf(exp);
        parameter.set(new_value);

        let slider_scale = LinearScale::new(slider.min(), slider.max());
        let parameter_scale = LogarithmicScale::new(parameter.min(), parameter.max());

        let new_value = (&slider_scale, &parameter_scale).convert(slider.value());
        parameter.set(new_value);
    }

    struct Slider;
    impl Slider {
        fn value(&self) -> f64 {
            21.0
        }
        fn min(&self) -> f64 {
            0.0
        }
        fn max(&self) -> f64 {
            100.0
        }
    }

    struct Parameter;
    impl Parameter {
        fn set(&self, _: f64) {}
        fn min(&self) -> f64 {
            10.0
        }
        fn max(&self) -> f64 {
            500.0
        }
    }
}
