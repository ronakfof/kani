// rmc-check-fail
// compile-flags: --edition 2021
#![allow(unused)]
pub fn main() {
enum ZeroVariants {}
let x: ZeroVariants = panic!();
let y: u32 = x; // mismatched type error
}