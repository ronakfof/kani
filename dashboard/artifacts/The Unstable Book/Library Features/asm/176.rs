// compile-flags: --edition 2015
#![allow(unused)]
#![feature(asm)]
fn main() {
let mut a: u64 = 4;
let b: u64 = 4;
unsafe {
    asm!("add {0}, {1}", inlateout(reg) a, in(reg) b);
}
assert_eq!(a, 8);
}