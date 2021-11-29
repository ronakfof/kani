// compile-flags: --edition 2018
#![allow(unused)]
fn main() {
fn first((value, _): (i32, i32)) -> i32 { value }
}