// compile-flags: --edition 2021
#![allow(unused)]
pub fn main() {
use std::fmt::Debug;
fn foo<T>(x: T) where T: Debug {
}
}