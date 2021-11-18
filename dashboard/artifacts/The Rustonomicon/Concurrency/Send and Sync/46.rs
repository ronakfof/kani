// compile-flags: --edition 2021
#![allow(unused)]
pub fn main() {
struct MyBox(*mut u8);

unsafe impl Send for MyBox {}
unsafe impl Sync for MyBox {}
}