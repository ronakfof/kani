// compile-flags: --edition 2021
#![allow(unused)]
#![feature(no_sanitize)]

fn main() {
#[no_sanitize(address)]
fn foo() {
  // ...
}
}