// compile-flags: --edition 2015
#![allow(unused)]
#![feature(llvm_asm)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn main() {
let result: i32;
unsafe {
   llvm_asm!("mov eax, 2" : "={eax}"(result) : : : "intel")
}
println!("eax is currently {}", result);
}
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn main() {}