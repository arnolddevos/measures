use core::ops::{Mul, Add, Sub, Div, Neg, BitOr};
use core::fmt;

pub use crate::scale::*;
use crate::macros::*;

measure!(Ohm, "Ω");
measure!(Siemen, "S");
measure!(Second, "s");
measure!(Farad, "F");
measure!(Hertz, "Hz");
measure!(Volt, "V");
measure!(Amp, "A");
measure!(Watt, "W");

product!(Ohm, Farad, Second);
product!(Amp, Ohm, Volt); // Ohm's law.
product!(Volt, Amp, Watt);

inverse!(Second, Hertz);
inverse!(Ohm, Siemen);

inverse_sum_inverse!(Siemen);
inverse_sum_inverse!(Ohm);
inverse_sum_inverse!(Farad);
