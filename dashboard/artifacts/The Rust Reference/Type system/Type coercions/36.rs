// compile-flags: --edition 2018
#![allow(unused)]
fn bar(_: &i8) { }

fn main() {
    bar(&mut 42);
}