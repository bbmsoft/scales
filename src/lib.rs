use std::ops::*;

pub trait Scale<N, F> {
    fn to_relative(&self, absolute: N) -> F;
    fn to_absolute(&self, relative: F) -> N;
}

pub fn scale<N, T, F>(absolute: N, from: &impl Scale<N, F>, to: &impl Scale<T, F>) -> T {
    let rel = from.to_relative(absolute);
    to.to_absolute(rel)
}

pub struct LinearScale<N, F> {
    min: N,
    max: N,
    rasterizer: Option<Box<dyn Fn(N) -> N>>,
    _phantom: std::marker::PhantomData<F>,
}

impl<N, F> LinearScale<N, F> {
    pub fn with_min_max(min: N, max: N) -> LinearScale<N, F> {
        LinearScale {
            min,
            max,
            rasterizer: None,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn with_min_max_and_rasterizer(
        min: N,
        max: N,
        rasterizer: impl Fn(N) -> N + 'static,
    ) -> LinearScale<N, F> {
        LinearScale {
            min,
            max,
            rasterizer: Some(Box::new(rasterizer)),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<N, F> Scale<N, F> for LinearScale<N, F>
where
    N: Add<N, Output = N> + Sub<N, Output = N> + ToFloat<F> + Clone,
    F: Mul<F, Output = F> + Div<F, Output = F> + ToFloat<N> + Clone,
{
    fn to_relative(&self, absolute: N) -> F {
        let absolute = if let Some(rasterizer) = self.rasterizer.as_ref() {
            rasterizer(absolute)
        } else {
            absolute
        };
        let partial_range: F = (absolute - self.min.clone()).to_float();
        let full_range: F = (self.max.clone() - self.min.clone()).to_float();
        partial_range / full_range
    }

    fn to_absolute(&self, relative: F) -> N {
        let full_range: F = (self.max.clone() - self.min.clone()).to_float();
        let partial: N = (relative * full_range).to_float();
        let abs = self.min.clone() + partial;
        if let Some(rasterizer) = self.rasterizer.as_ref() {
            rasterizer(abs)
        } else {
            abs
        }
    }
}

pub struct IntegralLinearScale<N, F> {
    min: N,
    max: N,
    _phantom: std::marker::PhantomData<F>,
}

impl<N, F> IntegralLinearScale<N, F> {
    pub fn with_min_max(min: N, max: N) -> IntegralLinearScale<N, F> {
        IntegralLinearScale {
            min,
            max,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<N, F> Scale<N, F> for IntegralLinearScale<N, F>
where
    N: Add<N, Output = N> + Sub<N, Output = N> + ToFloat<F> + Clone,
    F: Mul<F, Output = F> + Div<F, Output = F> + ToInt<N> + Clone,
{
    fn to_relative(&self, absolute: N) -> F {
        let partial_range: F = (absolute - self.min.clone()).to_float();
        let full_range: F = (self.max.clone() - self.min.clone()).to_float();
        partial_range / full_range
    }

    fn to_absolute(&self, relative: F) -> N {
        let full_range: F = (self.max.clone() - self.min.clone()).to_float();
        let partial: N = (relative * full_range).to_int();
        self.min.clone() + partial
    }
}

pub trait ToFloat<F> {
    fn to_float(self) -> F;
}

pub trait ToInt<I> {
    fn to_int(self) -> I;
}

pub trait FromFloat<F> {
    fn from_float(f: F) -> Self;
}

pub trait FromInt<I> {
    fn from_int(i: I) -> Self;
}

impl<F, I> ToFloat<F> for I
where
    F: FromInt<I>,
{
    fn to_float(self) -> F {
        F::from_int(self)
    }
}

impl FromFloat<f64> for f64 {
    fn from_float(f: f64) -> Self {
        f
    }
}

impl FromFloat<f32> for f32 {
    fn from_float(f: f32) -> Self {
        f
    }
}

impl FromFloat<f64> for f32 {
    fn from_float(f: f64) -> Self {
        f as f32
    }
}

impl FromFloat<f32> for f64 {
    fn from_float(f: f32) -> Self {
        f as f64
    }
}

impl ToFloat<f64> for f32 {
    fn to_float(self) -> f64 {
        self as f64
    }
}

impl ToFloat<f64> for f64 {
    fn to_float(self) -> f64 {
        self
    }
}

impl ToFloat<f32> for f64 {
    fn to_float(self) -> f32 {
        self as f32
    }
}

impl ToFloat<f32> for f32 {
    fn to_float(self) -> f32 {
        self
    }
}

impl<F, I> ToInt<I> for F
where
    I: FromFloat<F>,
{
    fn to_int(self) -> I {
        I::from_float(self)
    }
}

impl FromFloat<f64> for i128 {
    fn from_float(f: f64) -> Self {
        f as i128
    }
}
impl FromFloat<f32> for i128 {
    fn from_float(f: f32) -> Self {
        f as i128
    }
}

impl FromFloat<f64> for i64 {
    fn from_float(f: f64) -> Self {
        f as i64
    }
}
impl FromFloat<f32> for i64 {
    fn from_float(f: f32) -> Self {
        f as i64
    }
}

impl FromFloat<f64> for i32 {
    fn from_float(f: f64) -> Self {
        f as i32
    }
}
impl FromFloat<f32> for i32 {
    fn from_float(f: f32) -> Self {
        f as i32
    }
}

impl FromFloat<f64> for i16 {
    fn from_float(f: f64) -> Self {
        f as i16
    }
}
impl FromFloat<f32> for i16 {
    fn from_float(f: f32) -> Self {
        f as i16
    }
}

impl FromFloat<f64> for i8 {
    fn from_float(f: f64) -> Self {
        f as i8
    }
}
impl FromFloat<f32> for i8 {
    fn from_float(f: f32) -> Self {
        f as i8
    }
}

impl FromFloat<f64> for u128 {
    fn from_float(f: f64) -> Self {
        f as u128
    }
}
impl FromFloat<f32> for u128 {
    fn from_float(f: f32) -> Self {
        f as u128
    }
}

impl FromFloat<f64> for u64 {
    fn from_float(f: f64) -> Self {
        f as u64
    }
}
impl FromFloat<f32> for u64 {
    fn from_float(f: f32) -> Self {
        f as u64
    }
}

impl FromFloat<f64> for u32 {
    fn from_float(f: f64) -> Self {
        f as u32
    }
}
impl FromFloat<f32> for u32 {
    fn from_float(f: f32) -> Self {
        f as u32
    }
}

impl FromFloat<f64> for u16 {
    fn from_float(f: f64) -> Self {
        f as u16
    }
}
impl FromFloat<f32> for u16 {
    fn from_float(f: f32) -> Self {
        f as u16
    }
}

impl FromFloat<f64> for u8 {
    fn from_float(f: f64) -> Self {
        f as u8
    }
}
impl FromFloat<f32> for u8 {
    fn from_float(f: f32) -> Self {
        f as u8
    }
}

impl FromFloat<f64> for usize {
    fn from_float(f: f64) -> Self {
        f as usize
    }
}
impl FromFloat<f32> for usize {
    fn from_float(f: f32) -> Self {
        f as usize
    }
}

impl FromInt<i128> for f64 {
    fn from_int(i: i128) -> Self {
        i as f64
    }
}
impl FromInt<i128> for f32 {
    fn from_int(i: i128) -> Self {
        i as f32
    }
}

impl FromInt<i64> for f64 {
    fn from_int(i: i64) -> Self {
        i as f64
    }
}
impl FromInt<i64> for f32 {
    fn from_int(i: i64) -> Self {
        i as f32
    }
}

impl FromInt<i32> for f64 {
    fn from_int(i: i32) -> Self {
        i as f64
    }
}
impl FromInt<i32> for f32 {
    fn from_int(i: i32) -> Self {
        i as f32
    }
}

impl FromInt<i16> for f64 {
    fn from_int(i: i16) -> Self {
        i as f64
    }
}
impl FromInt<i16> for f32 {
    fn from_int(i: i16) -> Self {
        i as f32
    }
}

impl FromInt<i8> for f64 {
    fn from_int(i: i8) -> Self {
        i as f64
    }
}
impl FromInt<i8> for f32 {
    fn from_int(i: i8) -> Self {
        i as f32
    }
}

impl FromInt<u128> for f64 {
    fn from_int(i: u128) -> Self {
        i as f64
    }
}
impl FromInt<u128> for f32 {
    fn from_int(i: u128) -> Self {
        i as f32
    }
}

impl FromInt<u64> for f64 {
    fn from_int(i: u64) -> Self {
        i as f64
    }
}
impl FromInt<u64> for f32 {
    fn from_int(i: u64) -> Self {
        i as f32
    }
}

impl FromInt<u32> for f64 {
    fn from_int(i: u32) -> Self {
        i as f64
    }
}
impl FromInt<u32> for f32 {
    fn from_int(i: u32) -> Self {
        i as f32
    }
}

impl FromInt<u16> for f64 {
    fn from_int(i: u16) -> Self {
        i as f64
    }
}
impl FromInt<u16> for f32 {
    fn from_int(i: u16) -> Self {
        i as f32
    }
}

impl FromInt<u8> for f64 {
    fn from_int(i: u8) -> Self {
        i as f64
    }
}
impl FromInt<u8> for f32 {
    fn from_int(i: u8) -> Self {
        i as f32
    }
}

impl FromInt<usize> for f64 {
    fn from_int(i: usize) -> Self {
        i as f64
    }
}
impl FromInt<usize> for f32 {
    fn from_int(i: usize) -> Self {
        i as f32
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_linear_to_rel_f64() {
        let min: f64 = 0.0;
        let max: f64 = 100.0;
        let scale: LinearScale<f64, f64> = LinearScale::with_min_max(min, max);
        assert_eq!(scale.to_relative(0.0), 0.0);
        assert_eq!(scale.to_relative(100.0), 1.0);
        assert_eq!(scale.to_relative(10.0), 0.1);
        assert_eq!(scale.to_relative(50.0), 0.5);
        assert_eq!(scale.to_relative(90.0), 0.9);
    }

    #[test]
    fn test_linear_to_abs_f64() {
        let min: f64 = 0.0;
        let max: f64 = 100.0;
        let scale = LinearScale::with_min_max(min, max);
        assert_eq!(scale.to_absolute(0.0), 0.0);
        assert_eq!(scale.to_absolute(1.0), 100.0);
        assert_eq!(scale.to_absolute(0.1), 10.0);
        assert_eq!(scale.to_absolute(0.5), 50.0);
        assert_eq!(scale.to_absolute(0.9), 90.0);
    }

    #[test]
    fn test_linear_to_rel_f32() {
        let min: f32 = 0.0;
        let max: f32 = 100.0;
        let scale: LinearScale<f32, f32> = LinearScale::with_min_max(min, max);
        assert_eq!(scale.to_relative(0.0), 0.0);
        assert_eq!(scale.to_relative(100.0), 1.0);
        assert_eq!(scale.to_relative(10.0), 0.1);
        assert_eq!(scale.to_relative(50.0), 0.5);
        assert_eq!(scale.to_relative(90.0), 0.9);
    }

    #[test]
    fn test_linear_to_abs_f32() {
        let min: f32 = 0.0;
        let max: f32 = 100.0;
        let scale = LinearScale::with_min_max(min, max);
        assert_eq!(scale.to_absolute(0.0), 0.0);
        assert_eq!(scale.to_absolute(1.0), 100.0);
        assert_eq!(scale.to_absolute(0.1), 10.0);
        assert_eq!(scale.to_absolute(0.5), 50.0);
        assert_eq!(scale.to_absolute(0.9), 90.0);
    }

    #[test]
    fn test_linear_scale() {
        let scale_a: LinearScale<f64, f64> = LinearScale::with_min_max(0.0, 100.0);
        let scale_b: LinearScale<f64, f64> = LinearScale::with_min_max(-1.0, 1.0);

        assert_eq!(scale(25.0, &scale_a, &scale_b), -0.5);
        assert_eq!(scale(0.5, &scale_b, &scale_a), 75.0);
    }

    #[test]
    fn test_rasterizer() {
        let min: f32 = 0.0;
        let max: f32 = 1_000.0;
        let step = 10.0;
        let scale: LinearScale<f32, f32> =
            LinearScale::with_min_max_and_rasterizer(min, max, move |u| (u / step).round() * step);
        assert_eq!(scale.to_absolute(0.085), 90.0);
        assert_eq!(scale.to_relative(85.0), 0.09);
    }

    #[test]
    fn test_integral_linear_scale() {
        let scale_a: IntegralLinearScale<usize, f32> = IntegralLinearScale::with_min_max(0, 100);
        assert_eq!(scale_a.to_relative(20), 0.2);
        assert_eq!(scale_a.to_absolute(0.9), 90);

        let scale_b: LinearScale<f64, f32> = LinearScale::with_min_max(-10.0, 10.0);
        assert_eq!(scale_b.to_relative(5.0), 0.75);
        assert_eq!(scale_b.to_absolute(0.75), 5.0);

        assert_eq!(scale(75, &scale_a, &scale_b), 5.0);
    }
}
