// compile-flags: --edition 2021
#![allow(unused)]
fn main() {
struct List<T> {
  data: T,
  next: Option<Box<List<T>>>,
}
}