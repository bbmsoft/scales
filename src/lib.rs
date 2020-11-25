pub mod prelude;

mod broken;
mod convert;
mod converter;
mod linear;
mod logarithmic;

use convert::*;
use std::cell::RefCell;
use std::ops::*;
use std::rc::Rc;
use std::sync::Arc;

/// A scale is a mapping of an arbitrary, not necessarily linear, continuous and monotonically
/// increasing range of numbers to a relative value between 0.0 and 1.0.
/// It's useful for converting corresponding values between different coordinate spaces, for example for
/// processing input from or rendering to a graphical user interface. A typical example would
/// be calculating the position of a slider knob that controls a logarithmically scaled parameter.
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
}

impl<N, SN> Scale<N> for &SN
where
    N: Sub<Output = N> + Add<Output = N> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
    SN: Scale<N>,
{
    fn to_relative(&self, absolute: N) -> f64 {
        SN::to_relative(self, absolute)
    }

    fn to_absolute(&self, relative: f64) -> N {
        SN::to_absolute(self, relative)
    }

    fn max(&self) -> N {
        SN::max(self)
    }

    fn min(&self) -> N {
        SN::min(self)
    }
}

impl<N, SN> Scale<N> for Box<SN>
where
    N: Sub<Output = N> + Add<Output = N> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
    SN: Scale<N>,
{
    fn to_relative(&self, absolute: N) -> f64 {
        SN::to_relative(self, absolute)
    }

    fn to_absolute(&self, relative: f64) -> N {
        SN::to_absolute(self, relative)
    }

    fn max(&self) -> N {
        SN::max(self)
    }

    fn min(&self) -> N {
        SN::min(self)
    }
}

impl<N, SN> Scale<N> for Rc<SN>
where
    N: Sub<Output = N> + Add<Output = N> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
    SN: Scale<N>,
{
    fn to_relative(&self, absolute: N) -> f64 {
        SN::to_relative(self, absolute)
    }

    fn to_absolute(&self, relative: f64) -> N {
        SN::to_absolute(self, relative)
    }

    fn max(&self) -> N {
        SN::max(self)
    }

    fn min(&self) -> N {
        SN::min(self)
    }
}

impl<N, SN> Scale<N> for RefCell<SN>
where
    N: Sub<Output = N> + Add<Output = N> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
    SN: Scale<N>,
{
    fn to_relative(&self, absolute: N) -> f64 {
        SN::to_relative(self.borrow().deref(), absolute)
    }

    fn to_absolute(&self, relative: f64) -> N {
        SN::to_absolute(self.borrow().deref(), relative)
    }

    fn max(&self) -> N {
        SN::max(self.borrow().deref())
    }

    fn min(&self) -> N {
        SN::min(self.borrow().deref())
    }
}

impl<N, SN> Scale<N> for Arc<SN>
where
    N: Sub<Output = N> + Add<Output = N> + PartialOrd + FromFloat<f64> + ToFloat<f64> + Clone,
    SN: Scale<N>,
{
    fn to_relative(&self, absolute: N) -> f64 {
        SN::to_relative(self, absolute)
    }

    fn to_absolute(&self, relative: f64) -> N {
        SN::to_absolute(self, relative)
    }

    fn max(&self) -> N {
        SN::max(self)
    }

    fn min(&self) -> N {
        SN::min(self)
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;
    use std::rc::Rc;

    #[test]
    fn test_boxed_scale() {
        // just checking if blanket implementations compile, no assertions here

        let a: LinearScale<f64> = LinearScale::new(0.0, 100.0);
        let a = Box::new(&a);
        a.to_absolute(0.5);

        let b: LogarithmicScale<f64> = LogarithmicScale::new(1.0, 10.0);
        let b = Box::new(b);
        b.to_relative(5.0);

        let conv = (a, b);
        conv.convert(32.0);

        let a: LinearScale<f64> = LinearScale::new(0.0, 100.0);
        let a = Rc::new(a);
        a.to_absolute(0.4);

        let b: LogarithmicScale<f64> = LogarithmicScale::new(1.0, 10.0);
        let b = RefCell::new(b);

        let conv = (a, b);
        conv.convert(32.0);

        let a: LinearScale<f64> = LinearScale::new(0.0, 100.0);
        let a = Arc::new(a);
        a.to_absolute(0.4);

        let b: LogarithmicScale<f64> = LogarithmicScale::new(1.0, 10.0);
        let b = RefCell::new(b);

        let conv = (a, b);
        conv.convert(32.0);
    }
}
