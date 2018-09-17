# cash_float
Wrapper around bigdecimal for safe conversions to and from f64

# Goal
The goal of this crate is to take a f64 and pass it to a financial decimal crate.

Financial decimal crates are used to perform operations on floats without introduciing erros.
See [this](https://floating-point-gui.de/) guide for an explination of float erros.

These crates usually that takes a string form of float eg.
`"3.141592"`
However some use cases involve recieving an float in primitive float form.
Simply casting to a string can indroduce errors.

# Usage
```rust
let my_float: f64 = 3.141592;
let decimal = cash_float::new(my_float);
// Do operations
let my_float = cash_float::to_f64(decimal);
```
