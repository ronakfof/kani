// compile-flags: --edition 2018
#![allow(unused)]
fn main() {
use std::fmt::Display;
fn foo(x: &u32) -> &dyn Display {
    x
}
}