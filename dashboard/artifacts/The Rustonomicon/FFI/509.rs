// compile-flags: --edition 2021
#![allow(unused)]
pub fn main() {
unsafe fn kaboom(ptr: *const i32) -> i32 { *ptr }
}