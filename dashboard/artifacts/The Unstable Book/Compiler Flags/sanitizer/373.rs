// compile-flags: --edition 2021
#![allow(unused)]
pub fn main() {
    let xs = vec![0, 1, 2, 3];
    let _y = unsafe { *xs.as_ptr().offset(4) };
}