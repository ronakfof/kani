// compile-flags: --edition 2015
#![allow(unused)]
#![feature(llvm_asm)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn main() { unsafe {
llvm_asm!("xor %eax, %eax"
    :
    :
    : "eax"
   );
} }
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn main() {}