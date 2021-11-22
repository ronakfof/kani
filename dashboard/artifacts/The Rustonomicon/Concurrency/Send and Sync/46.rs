// compile-flags: --edition 2021
#![allow(unused)]
fn main() {
struct MyBox(*mut u8);

unsafe impl Send for MyBox {}
unsafe impl Sync for MyBox {}
}