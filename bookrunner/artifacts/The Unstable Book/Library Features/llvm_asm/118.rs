// compile-flags: --edition 2015
#![allow(unused)]
#![feature(llvm_asm)]
fn main() {
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn read_byte_in(port: u16) -> u8 {
let result: u8;
llvm_asm!("in %dx, %al" : "={al}"(result) : "{dx}"(port));
result
}
}