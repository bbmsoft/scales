/// Something that can be converted to a floating point number.
pub trait ToFloat<F> {
    /// Convert self into a floating point number.
    fn to_float(self) -> F;
}

/// Something that can be converted to an integral number.
pub trait ToInt<I> {
    /// Convert self into an integral number.
    fn to_int(self) -> I;
}

/// Something a floating point number can be converted into.
pub trait FromFloat<F> {
    /// Convert the provided floating point number into an instance of the implementing type.
    fn from_float(f: F) -> Self;
}

/// Something an integral number can be converted into.
pub trait FromInt<I> {
    /// Convert the provided integral number into an instance of the implementing type.
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
