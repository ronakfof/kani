// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
#[must_use]
fn five() -> i32 { 5i32 }

// Violates the unused_must_use lint.
five();
}