// compile-flags: --edition 2021
#![allow(unused)]
pub fn main() {
struct List<T> {
  data: T,
  next: Option<Box<List<T>>>,
}
}