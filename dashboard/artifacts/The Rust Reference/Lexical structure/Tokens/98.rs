// compile-flags: --edition 2018
#![allow(unused)]
fn main() {
macro_rules! blackhole { ($tt:tt) => () }

blackhole!("string"suffix); // OK
}