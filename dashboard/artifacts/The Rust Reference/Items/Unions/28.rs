// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
union MyUnion { f1: u32, f2: f32 }

let u = MyUnion { f1: 1 };
}