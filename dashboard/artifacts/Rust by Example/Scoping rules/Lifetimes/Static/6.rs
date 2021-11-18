// compile-flags: --edition 2021
#![allow(unused)]
// A reference with 'static lifetime:
pub fn main() {
let s: &'static str = "hello world";

// 'static as part of a trait bound:
fn generic<T>(x: T) where T: 'static {}
}