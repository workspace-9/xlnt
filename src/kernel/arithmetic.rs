use std::ops::*;

pub trait Floating: std::str::FromStr {
    fn sqrt(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn ln(self) -> Self;
    fn log(self, base: Self) -> Self;
    fn pow(self, exp: Self) -> Self;
    fn from_f64(number: f64) -> Self;
}

macro_rules! impl_floating_for {
    ($t:ty, $conv:expr) => {
        impl Floating for $t {
            fn sqrt(self) -> Self {
                Self::sqrt(self)
            }

            fn sin(self) -> Self {
                Self::sin(self)
            }

            fn cos(self) -> Self {
                Self::cos(self)
            }

            fn tan(self) -> Self {
                Self::tan(self)
            }

            fn ln(self) -> Self {
                Self::ln(self)
            }

            fn log(self, base: Self) -> Self {
                Self::log(self, base)
            }

            fn pow(self, exp: Self) -> Self {
                Self::powf(self, exp)
            }

            fn from_f64(number: f64) -> Self {
                $conv(number)
            }
        }
    };
}

impl_floating_for!(f32, |x| x as f32);
impl_floating_for!(f64, |x| x);

#[cfg(feature = "f128")]
impl_floating_for!(f128, |x| x.into());

pub trait Arithmetic:
    Add<Output=Self> +
    Sub<Output=Self> +
    Mul<Output=Self> +
    Div<Output=Self> +
    AddAssign +
    SubAssign +
    MulAssign +
    DivAssign +
    Floating +
    Copy +
    Sized
{}

impl Arithmetic for f32 {}
impl Arithmetic for f64 {}

#[cfg(feature = "f128")]
impl Arithmetic for f128 {}
