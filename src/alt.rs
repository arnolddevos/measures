use core::{
    fmt::{Display, Formatter, Result},
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::scale::{engineering, precision};

#[derive(Copy, Clone, Default)]
pub struct Value<R, U>(R, U);

impl<R, U> Display for Value<R, U>
where
    R: Into<f64> + Clone,
    U: Display,
{
    fn fmt(&self, dst: &mut Formatter) -> Result {
        let repr: f64 = self.0.clone().into();
        let (prefix, scale) = engineering(repr);
        let value = repr / scale;
        let fract = precision(value);
        write!(dst, "{:.*}{}{}", fract, value, prefix, self.1)
    }
}

impl<R, U> PartialEq for Value<R, U>
where
    R: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<R, U> PartialOrd for Value<R, U>
where
    R: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<R, U> Neg for Value<R, U>
where
    R: Neg<Output = R>,
{
    type Output = Value<R, U>;
    fn neg(self) -> Self::Output {
        Value(-self.0, self.1)
    }
}

impl<R, U> Add<Value<R, U>> for Value<R, U>
where
    R: Add<R, Output = R>,
{
    type Output = Value<R, U>;

    fn add(self, rhs: Value<R, U>) -> Self::Output {
        Value(self.0 + rhs.0, self.1)
    }
}

impl<R, U> Sub<Value<R, U>> for Value<R, U>
where
    R: Sub<R, Output = R>,
{
    type Output = Value<R, U>;

    fn sub(self, rhs: Value<R, U>) -> Self::Output {
        Value(self.0 - rhs.0, self.1)
    }
}

impl<R, U> Mul<R> for Value<R, U>
where
    R: Mul<R, Output = R>,
{
    type Output = Value<R, U>;

    fn mul(self, rhs: R) -> Self::Output {
        Value(self.0 * rhs, self.1)
    }
}

/// Mul is commutative when representation is f64
impl<U> Mul<Value<f64, U>> for f64 {
    type Output = Value<f64, U>;

    fn mul(self, rhs: Value<f64, U>) -> Self::Output {
        rhs * self
    }
}

/// Mul is commutative when representation is f32
impl<U> Mul<Value<f32, U>> for f32 {
    type Output = Value<f32, U>;

    fn mul(self, rhs: Value<f32, U>) -> Self::Output {
        rhs * self
    }
}

impl<R, U> Div<R> for Value<R, U>
where
    R: Div<R, Output = R>,
{
    type Output = Value<R, U>;

    fn div(self, rhs: R) -> Self::Output {
        Value(self.0 / rhs, self.1)
    }
}

impl<R, U> Div<Value<R, U>> for Value<R, U>
where
    R: Div<R, Output = R>,
{
    type Output = R;

    fn div(self, rhs: Value<R, U>) -> Self::Output {
        self.0 / rhs.0
    }
}

/// Indicates the type for a Product of two units
pub trait Product<U> {
    type Output;
}

/// Ability to multiply values where Product is defined for their units
impl<R, U, V> Mul<Value<R, U>> for Value<R, V>
where
    V: Product<U>,
    V::Output: Default,
    R: Mul<R, Output = R>,
{
    type Output = Value<R, V::Output>;

    fn mul(self, rhs: Value<R, U>) -> Self::Output {
        Value(self.0 * rhs.0, Default::default())
    }
}

#[derive(Copy, Clone, Default)]
pub struct Volt;

impl Display for Volt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("V")
    }
}

impl Mul<Volt> for f64 {
    type Output = Value<f64, Volt>;

    fn mul(self, rhs: Volt) -> Self::Output {
        Value(self, rhs)
    }
}

#[derive(Copy, Clone, Default)]
pub struct Amp;

impl Display for Amp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("A")
    }
}

impl Mul<Amp> for f64 {
    type Output = Value<f64, Amp>;

    fn mul(self, rhs: Amp) -> Self::Output {
        Value(self, rhs)
    }
}

#[derive(Copy, Clone, Default)]
pub struct Watt;

impl Display for Watt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("W")
    }
}

impl Mul<Watt> for f64 {
    type Output = Value<f64, Watt>;

    fn mul(self, rhs: Watt) -> Self::Output {
        Value(self, rhs)
    }
}

impl Product<Amp> for Volt {
    type Output = Watt;
}

impl Product<Volt> for Amp {
    type Output = Watt;
}

// impl Measure for Value<f64, Volt> {
//     type Scalar = f64;
//     type Unit = Volt;
//     const UNIT: Volt = Volt;
// }

// pub trait Measure
// where
//     Self: Neg<Output = Self>
//         + Add<Output = Self>
//         + Sub<Output = Self>
//         + Mul<Self::Scalar, Output = Self>
//         + Div<Self::Scalar, Output = Self>
//         + Div<Output = Self::Scalar>
//         + PartialEq
//         + PartialOrd
//         + Display
//         + Copy
//         + Clone,
//     Self::Unit: Display,
//     Self::Scalar: Mul<Self, Output = Self> + Mul<Self::Unit, Output = Self>,
// {
//     type Scalar;
//     type Unit;
//     const UNIT: Self::Unit;
// }
