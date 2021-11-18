// compile-flags: --edition 2021
#![allow(unused)]
#![feature(no_sanitize)]

pub fn main() {
#[no_sanitize(address)]
fn foo() {
  // ...
}
}