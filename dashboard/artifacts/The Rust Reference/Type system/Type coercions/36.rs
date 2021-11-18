// compile-flags: --edition 2021
#![allow(unused)]
fn bar(_: &i8) { }

pub fn main() {
    bar(&mut 42);
}