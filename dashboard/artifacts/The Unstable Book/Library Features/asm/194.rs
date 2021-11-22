// compile-flags: --edition 2021
#![allow(unused)]
#![feature(asm)]
fn main() {
let cmd = 0xd1;
unsafe {
    asm!("out 0x64, eax", in("eax") cmd);
}
}