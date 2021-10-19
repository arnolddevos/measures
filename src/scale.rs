pub const M: f64 = 1e+6;
pub const k: f64 = 1e+3;
pub const m: f64 = 1e-3;
pub const u: f64 = 1e-6;
pub const n: f64 = 1e-9;
pub const p: f64 = 1e-12;

fn abs(x: f64) -> f64 {
    if x < 0.0 {
        -x
    } else {
        x
    }
}

pub fn engineering(value: f64) -> (&'static str, f64) {
    match abs(value) {
        v if v >= M => ("M", M),
        v if v >= k => ("k", k),
        v if v >= 1.0 => ("", 1.0),
        v if v >= m => ("m", m),
        v if v >= u => ("μ", u),
        v if v >= n => ("n", n),
        v if v == 0.0 => ("", 1.0),
        _ => ("p", p),
    }
}

pub fn precision(value: f64) -> usize {
    match abs(value) {
        v if v >= 100.0 => 0,
        v if v >= 10.0 => 1,
        v if v >= 1.0 => 2,
        v if v == 0.0 => 0,
        _ => 10,
    }
}
