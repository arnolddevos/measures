#![allow(non_upper_case_globals)]
// #![no_std] almost - using println!() from std

mod units;
mod macros;
mod scale;
mod elec;
mod rcnet;

use units::*;
use rcnet::*;

const root2: f64 = 1.4142;

fn ct_net() -> Net<Amp> {
    Net {

        title: "CT Circuit",

        cct: |i: Amp| {
            let vdd = Volt(3.0);
            let r1 = Ohm(330.0*k);
            let r2 = Ohm(330.0*k);
            let r3 = Ohm(33.0);    // CT burden
            let ctr = 1000.0;
        
            vdd + r1 |  (i/ctr | 1.0/r3) + r2
        },
    
        drive: {
            let i = 50.0*root2; 
            [Amp(-i), Amp(0.0), Amp(i)] 
        },

        cap: Farad(1.5*n),
    
    }
}

fn line_net() -> Net<Volt> {

    Net {

        title: "Line Circuit",

        cct: |v: Volt| {
            let vdd = Volt(3.0);
            let r1 = Ohm(33.0*k);
            let r2 = Ohm(33.0*k);
            let r3 = Ohm(5.1*M);

            vdd + r1 | r2 | v + r3
        },

        drive: { 
            let v = 240.0 * root2;
            [Volt(-v), Volt(0.0), Volt(v)] 
        },

        cap: Farad(15.0*n),
    }
}

fn cp_net() -> Net<Volt> {
    Net {

        title: "CP Circuit",

        cct: |v: Volt| {
            let vdd = Volt(3.0);
            let r1 = Ohm(75.0*k);
            let r2 = Ohm(100.0*k);
            let r3 = Ohm(300.0*k);

            vdd + r1 | r2 | v + r3
        },

        drive: { 
            let v = 12.0;
            [Volt(-v), Volt(0.0), Volt(v)] 
        },

        cap: Farad(1.5*n),
    }   
}

fn pp_net() -> Net<Volt> {
    Net {

        title: "PP Circuit",

        cct: |v: Volt| {
            let r1 = Ohm(300.0*k);
            let r2 = Ohm(200.0*k);

            v + r2 | r1
        },

        drive: { 
            [Volt(0.0), Volt(2.0), Volt(5.0)] 
        },

        cap: Farad(100.0*n),
    }   
}

fn open_evse_cp_net() -> Net<Volt> {
    Net {

        title: "Open EVSE CP Circuit",

        cct: |v: Volt| {
            let r7 = 200.0*Ohm(k);
            let r6 = 100.0*Ohm(k);
            let r5 = 56.0*Ohm(k);
        
            let vcc = Volt(5.0);
            let gnd = Volt(0.0);
        
            v + r7 |  gnd + r6 | vcc + r5
        },

        drive: { 
            let v = 12.0;
            [Volt(-v), Volt(0.0), Volt(v)] 
        },

        cap: Farad(15.0*p)  // no explicit cap in cct
    }
}

fn cp_cct_design() {
    let vcp = Volt(12.0);
    let vdd = Volt(3.0);
    let r3  = Ohm(300.0*k);
    let margin = 0.1;
    let atten = vdd/vcp * (1.0 - margin);

    let (r1, r2) = double_to_single_ended(atten, r3);
    println!("CP divider: r1 = {}, r2 = {}, r3 = {}", r1, r2, r3);
}

fn main() {
    cp_cct_design();
    ct_net().describe();
    line_net().describe();
    cp_net().describe();
    pp_net().describe();
    open_evse_cp_net().describe();
}
