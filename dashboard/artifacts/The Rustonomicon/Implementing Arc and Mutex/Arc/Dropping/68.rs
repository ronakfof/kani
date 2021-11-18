// compile-flags: --edition 2021
#![allow(unused)]
pub fn main() {
use std::sync::atomic::Ordering;
use std::sync::atomic;
atomic::fence(Ordering::Acquire);
}