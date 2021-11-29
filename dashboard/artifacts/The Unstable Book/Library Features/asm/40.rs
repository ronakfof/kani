// compile-flags: --edition 2015
#![allow(unused)]
#![feature(asm)]
fn main() {
unsafe {
    asm!("nop");
}
}