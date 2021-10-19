use core::fmt;
use core::ops::{Add, BitOr, Neg};

use crate::units::*;

#[derive(Debug, Copy, Clone)]
pub enum Cct {
    Thevenin(Volt, Ohm),
    Norton(Amp, Siemen),
}
use self::Cct::*;

impl Cct {
    pub fn i_short(self) -> Amp {
        match self {
            Thevenin(v, r) => v / r,
            Norton(i, _) => i,
        }
    }
    pub fn v_open(self) -> Volt {
        match self {
            Thevenin(v, _) => v,
            Norton(i, g) => i * (1.0 / g),
        }
    }
    pub fn r_equiv(self) -> Ohm {
        match self {
            Thevenin(_, r) => r,
            Norton(_, g) => 1.0 / g,
        }
    }
    pub fn g_equiv(self) -> Siemen {
        match self {
            Thevenin(_, r) => 1.0 / r,
            Norton(_, g) => g,
        }
    }
}

fn norton_wins(lhs: Cct, rhs: Cct) -> bool {
    match (lhs, rhs) {
        (Thevenin(_, _), Thevenin(_, _)) => false,
        (Norton(_, _), Norton(_, _)) => true,
        (Norton(_, g), _) if g.0 < n => true,
        (_, Norton(_, g)) if g.0 < n => true,
        _ => false,
    }
}

impl BitOr<Cct> for Cct {
    type Output = Cct;
    fn bitor(self, rhs: Cct) -> Cct {
        if norton_wins(self, rhs) {
            let ge = self.g_equiv() + rhs.g_equiv();
            let ie = self.i_short() + rhs.i_short();
            Norton(ie, ge)
        } else {
            let re = self.r_equiv() | rhs.r_equiv();
            let rs = self.r_equiv() + rhs.r_equiv();
            let n1 = rhs.r_equiv() / rs;
            let n2 = self.r_equiv() / rs;
            let ve = self.v_open() * n1 + rhs.v_open() * n2;
            Thevenin(ve, re)
        }
    }
}

impl Add<Cct> for Cct {
    type Output = Cct;
    fn add(self, rhs: Cct) -> Cct {
        if norton_wins(self, rhs) {
            let ge = self.g_equiv() | rhs.g_equiv();
            let gp = self.g_equiv() + rhs.g_equiv();
            let n1 = rhs.g_equiv() / gp;
            let n2 = self.g_equiv() / gp;
            let ie = self.i_short() * n1 + rhs.i_short() * n2;
            Norton(ie, ge)
        } else {
            let re = self.r_equiv() + rhs.r_equiv();
            let ve = self.v_open() + rhs.v_open();
            Thevenin(ve, re)
        }
    }
}

impl Neg for Cct {
    type Output = Cct;
    fn neg(self) -> Cct {
        match self {
            Thevenin(v, r) => Thevenin(-v, r),
            Norton(i, g) => Norton(-i, g),
        }
    }
}

impl fmt::Display for Cct {
    fn fmt(&self, dst: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Thevenin(v, r) => write!(dst, "{} + {}", v, r),
            Norton(i, g) => write!(dst, "{} | {}", i, g),
        }
    }
}

// Conversions from elec units to Cct

impl Add<Ohm> for Volt {
    type Output = Cct;
    fn add(self, rhs: Ohm) -> Cct {
        Thevenin(self, rhs)
    }
}

impl BitOr<Siemen> for Amp {
    type Output = Cct;
    fn bitor(self, rhs: Siemen) -> Cct {
        Norton(self, rhs)
    }
}

impl BitOr<Ohm> for Cct {
    type Output = Cct;
    fn bitor(self, rhs: Ohm) -> Cct {
        self | Thevenin(Volt(0.0), rhs)
    }
}

impl Add<Ohm> for Cct {
    type Output = Cct;
    fn add(self, rhs: Ohm) -> Cct {
        self + Thevenin(Volt(0.0), rhs)
    }
}

impl BitOr<Siemen> for Cct {
    type Output = Cct;
    fn bitor(self, rhs: Siemen) -> Cct {
        self | Norton(Amp(0.0), rhs)
    }
}

impl Add<Siemen> for Cct {
    type Output = Cct;
    fn add(self, rhs: Siemen) -> Cct {
        self + Norton(Amp(0.0), rhs)
    }
}

#[test]
fn open_evse_cp_example() {
    let r7 = 200.0 * Ohm(k);
    let r6 = 100.0 * Ohm(k);
    let r5 = 56.0 * Ohm(k);

    let vcc = Volt(5.0);
    let v_cp_hi = Volt(12.0);
    let gnd = Volt(0.0);

    let cct = v_cp_hi + r7 | gnd + r6 | vcc + r5;

    fn divider3(v1: Volt, r1: Ohm, v2: Volt, r2: Ohm, v3: Volt, r3: Ohm) -> Volt {
        (v1 / r1 + v2 / r2 + v3 / r3) * (r1 | r2 | r3)
    }

    let v1 = cct.v_open();
    let v2 = divider3(v_cp_hi, r7, gnd, r6, vcc, r5);

    assert_eq!(v1, v2)
}

#[test]
fn doc_example() {
    let vcc = Volt(5.0);
    let r1 = Ohm(10.0 * k);
    let r2 = Ohm(5.0 * k);

    let circuit = vcc + r1    // A voltage source vcc in series with resistor r1 forms a Cct,
        | r2; // which is extended by resistor r2 in parallel

    let v1 = circuit.v_open(); // v1 is the voltage produced
    assert_eq!(v1, r2 / (r1 + r2) * vcc)
}
