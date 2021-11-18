// compile-flags: --edition 2021
#![allow(unused)]
pub fn main() {
use std::cmp::Ordering;

unsafe trait UnsafeOrd {
    fn cmp(&self, other: &Self) -> Ordering;
}
}