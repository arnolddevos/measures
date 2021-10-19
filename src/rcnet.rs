use crate::elec::*;
use crate::units::*;
use core::fmt::Display;

pub struct Net<Drive: Display + Copy + Default> {
    pub title: &'static str,
    pub drive: [Drive; 3],
    pub cap: Farad,
    pub cct: fn(Drive) -> Cct,
}

impl<Drive: Display + Copy + Default> Net<Drive> {
    pub fn describe_ac(&self) {
        let x: Drive = Default::default();
        let r: Ohm = (self.cct)(x).r_equiv();
        println!("{} 3db f = {}", self.title, f3db(r * self.cap))
    }

    pub fn describe_dc(&self) {
        println!("{} in -> out", self.title);
        for &x in &self.drive {
            let c = (self.cct)(x);
            println!("{} \t -> {}, {}", x, c.v_open(), c.i_short())
        }
    }

    pub fn describe(&self) {
        println!();
        self.describe_dc();
        self.describe_ac();
        println!();
    }
}

/// Design a voltage divider to convert +/-vin to 0/+vout
/// The assumed circuit is:
///
/// ```
/// vout + r1 | r2 | vin + r_input
/// ```
///
/// Provide atten = vout/vin
/// and r_input
///
/// The fn returns (r1, r2)
///
pub fn double_to_single_ended(atten: f64, r_input: Ohm) -> (Ohm, Ohm) {
    let r1 = atten * r_input;
    let r2 = r1 / (1.0 - atten);
    (r1, r2)
}

use core::f64::consts::PI;

pub fn f3db(t: Second) -> Hertz {
    1.0 / (2.0 * PI * t)
}
