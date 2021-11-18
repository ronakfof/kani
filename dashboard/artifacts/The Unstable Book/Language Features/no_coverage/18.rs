// compile-flags: --edition 2021
#![allow(unused)]
#![feature(no_coverage)]

// `foo()` will get coverage instrumentation (by default)
pub fn main() {
fn foo() {
  // ...
}

#[no_coverage]
fn bar() {
  // ...
}
}