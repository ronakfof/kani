// compile-flags: --edition 2021
#![allow(unused)]
pub fn main() {
#[repr(C)]
union MyUnion {
    f1: u32,
    f2: f32,
}
}