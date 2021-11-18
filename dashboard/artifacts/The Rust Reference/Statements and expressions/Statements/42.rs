// compile-flags: --edition 2021
#![allow(unused)]
pub fn main() {
fn outer() {
  let outer_var = true;

  fn inner() { /* outer_var is not in scope here */ }

  inner();
}
}