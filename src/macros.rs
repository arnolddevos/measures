macro_rules! measure {
    ($id:ident, $symbol:expr) => {
        #[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Default)]
        pub struct $id(pub f64);

        impl fmt::Display for $id {
            fn fmt(&self, dst: &mut fmt::Formatter) -> fmt::Result {
                let (prefix, scale) = engineering(self.0);
                let value = self.0 / scale;
                let fract = precision(value);
                write!(dst, "{:.*}{}{}", fract, value, prefix, $symbol)
            }
        }

        impl Neg for $id {
            type Output = $id;
            fn neg(self) -> $id {
                $id(-self.0)
            }
        }

        impl Add<$id> for $id {
            type Output = $id;
            fn add(self, rhs: $id) -> $id {
                $id(self.0 + rhs.0)
            }
        }

        impl Sub<$id> for $id {
            type Output = $id;
            fn sub(self, rhs: $id) -> $id {
                $id(self.0 - rhs.0)
            }
        }

        impl Div<$id> for $id {
            type Output = f64;
            fn div(self, rhs: $id) -> f64 {
                self.0 / rhs.0
            }
        }

        impl Div<f64> for $id {
            type Output = $id;
            fn div(self, rhs: f64) -> $id {
                $id(self.0 / rhs)
            }
        }

        impl Mul<f64> for $id {
            type Output = $id;
            fn mul(self, rhs: f64) -> $id {
                $id(self.0 * rhs)
            }
        }

        impl Mul<$id> for f64 {
            type Output = $id;
            fn mul(self, rhs: $id) -> $id {
                $id(self * rhs.0)
            }
        }
    };
}

pub(crate) use measure;

macro_rules! product {
    ($a:ident, $b:ident, $c:ident) => {
        impl Mul<$b> for $a {
            type Output = $c;
            fn mul(self, rhs: $b) -> $c {
                $c(self.0 * rhs.0)
            }
        }
        impl Mul<$a> for $b {
            type Output = $c;
            fn mul(self, rhs: $a) -> $c {
                $c(self.0 * rhs.0)
            }
        }
        impl Div<$a> for $c {
            type Output = $b;
            fn div(self, rhs: $a) -> $b {
                $b(self.0 / rhs.0)
            }
        }
        impl Div<$b> for $c {
            type Output = $a;
            fn div(self, rhs: $b) -> $a {
                $a(self.0 / rhs.0)
            }
        }
    };
}

pub(crate) use product;

macro_rules! inverse_sum_inverse {
    ($id:ident) => {
        impl BitOr<$id> for $id {
            type Output = $id;
            fn bitor(self, rhs: $id) -> $id {
                if self.0 == 0.0 && rhs.0 == 0.0 {
                    $id(0.0)
                } else {
                    $id(self.0 * rhs.0 / (self.0 + rhs.0))
                }
            }
        }
    };
}

pub(crate) use inverse_sum_inverse;

macro_rules! inverse {
    ($a:ident, $b:ident) => {
        impl Mul<$b> for $a {
            type Output = f64;
            fn mul(self, rhs: $b) -> f64 {
                self.0 * rhs.0
            }
        }
        impl Mul<$a> for $b {
            type Output = f64;
            fn mul(self, rhs: $a) -> f64 {
                self.0 * rhs.0
            }
        }
        impl Div<$a> for f64 {
            type Output = $b;
            fn div(self, rhs: $a) -> $b {
                $b(self / rhs.0)
            }
        }
        impl Div<$b> for f64 {
            type Output = $a;
            fn div(self, rhs: $b) -> $a {
                $a(self / rhs.0)
            }
        }
    };
}

pub(crate) use inverse;
