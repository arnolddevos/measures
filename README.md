# Measures

This project includes a naescent unit-of-measure library I will call _measures_.  
This enables you to write type-checked expressions in terms of SI units.

There is also a representation of Thevenin and Norton equivalent circuits for
doing electrical calculations.

## Using Measures

If the types agree then the expression is dimensionally correct:

```rust
  let v1 = Amp(0.1) * Ohm(10.0);             // v1 has type Volt (Ohm's law)
  let x  = v1 + Amp(10.0);                   // error: can't add volts to amps.
  let x: Volt = v1 / Ohm(10.0);              // error: result has type Amp, not Volt
  let r1 = Second(450.0*u) / Farad(100.0*n); // OK: compute the resistor value for an RC network
```

(There is an emphasis on electrical units.) 

The types provided, called _measures_, represent unscaled SI units.  
They include: `Volt`, `Amp`, `Ohm`, `Farad` and `Second`.

Standard scaling constants are also provided: `M`, `k`, `m`, `u`, `n`, `p`.  
So 10kΩ can be written `Ohm(10.0*k)`, `10.0*Ohm(k)`, `Ohm(10.0)*k` and so on.  
Not quite the natural notation but simple and predictable.  

## Formatting

Measures can be formatted in a natural notation, that is, engineering notation. 
The formatted value is scaled to the appropriate range and the SI unit symbol is appended.
For example:

```rust
println!("R = {}", Second(450.0*u) / Farad(100.0*n));
// R = 4.50kΩ
```

## Declaring Measures

Measures are declared with a macro:

```rust
measure!(Candela, "cd"); // the measure of luminous intensity
```

This declares a type, `Candela` and some traits for the type:

- multiply operators between `Candela` and dimensionless `f64` values.
- divide operator on `Candela` by `f64`.
- add, subtract and divide operators between `Candela` values. 
- comparison operators between `Candela` values.
- formatting for `Candela` using the given unit name `"cd"`.

## Expressions

The library knows something about how different measures can be combined.  Two macros are
used to express relationships between measures.  

A product rule establishes a three-measure relationship, such as Ohm's law.  
An inverse rule establishes a two-measure relationship such as time and frequency.

```rust
product!(Amp, Ohm, Volt); // Ohm's law.
inverse!(Second, Hertz);  // Time and frequency.
```

These rules define further operators between the types:

- multiply between `Amp` and `Ohm`
- divide `Volt` by `Ohm` or `Amp`
- multiply between `Second` and `Hertz`
- divide dimensionless `f64` by `Second` or `Hertz`


##  Equivalent Circuits.

A `Cct` type is provided to help with simple DC circuit calculations.  It represents a 
Thevenin or Norton equivalent circuit. Ref:  https://en.wikipedia.org/wiki/Thévenin%27s_theorem

A Thevenin `Cct` is created by combining a `Volt` and an `Ohm` measure: `v + r`.  Here `+` is
the series operator.  

A Norton `Cct` is created by combining an `Amp` and a `Siemen` measure: `i | g`.  Here `|` is
the parallel operator.  

A `Cct` of either form can be combined with another element in series parallel using `+` or `|`. 
The added element can be an `Ohm` or a `Siemen` measure or another `Cct`. 

In this way larger networks can be built up.   The result `Cct` will be represented internally
in Thevenin or Norton form depending on its antecedents and its equivalent conductance. 
Norton is favoured for very low conductance and Thevenin otherwise.

An example voltage divider circuit:

```rust
    let vcc = Volt(5.0);
    let r1 = Ohm(10.0*k);
    let r2 = Ohm(5.0*k);
    
    let circuit =
        vcc + r1    // A voltage source vcc in series with resistor r1 forms a Cct,
        | r2;       // which is extended by resistor r2 in parallel
    
    let v1 = circuit.v_open(); // v1 = 1.67V is the voltage produced
```

A `Cct` can be queried with methods `.v_open()`, `.i_short()`, `.r_equiv()` or `g_equiv()` 
for its open circuit voltage, short circuit current, equivalent resistance or conductance.  

Care must be taken with very low resistance or conductance:

```rust
let vs = Volt(x) + Ohm(0.0);    // OK: zero resistance voltage source
let is = Amp(x) + Siemen(0.0);  // OK: zero conductance current source
let i = vs.i_short();           // error: infinite current in short circuit
let v = is.v_open();            // error: infinite voltage in open circuit
let p = vs | vs;                // error: can't parallel pure voltage sources
let s = is + is;                // error: can't place pure current sources in series
```

Finally, it is possible to reverse the direction of a `Cct` with `-`, the negate operator.  
The result has the same resistance (conductance) but the voltage (current) source polarity
is reversed. 

The following brash one-liner uses the `circuit` defined above and 
seeks to represent a wheatstone bridge:

```rust
let w =  - circuit + circuit;  
```

# TODO

- More measures.
- Complex impedances. AC sources.
- More tests.
