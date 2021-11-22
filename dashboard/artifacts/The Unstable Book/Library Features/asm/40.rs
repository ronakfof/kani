// compile-flags: --edition 2021
#![allow(unused)]
#![feature(asm)]
fn main() {
unsafe {
    asm!("nop");
}
}