// rmc-check-fail
// compile-flags: --edition 2021
#![allow(unused)]
// This will not compile

fn main() {
fn foo(x: i32, ...) {}
}