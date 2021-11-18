// compile-flags: --edition 2021
#![allow(unused)]
#![feature(asm)]
pub fn main() {
unsafe {
    asm!("nop");
}
}