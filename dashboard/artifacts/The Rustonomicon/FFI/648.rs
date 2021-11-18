// rmc-check-fail
// compile-flags: --edition 2021
#![allow(unused)]
// This will not compile

pub fn main() {
fn foo(x: i32, ...) {}
}