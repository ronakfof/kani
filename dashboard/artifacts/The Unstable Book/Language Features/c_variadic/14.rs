// compile-flags: --edition 2018
#![allow(unused)]
#![feature(c_variadic)]

pub fn main() {
pub unsafe extern "C" fn add(n: usize, mut args: ...) -> usize {
    let mut sum = 0;
    for _ in 0..n {
        sum += args.arg::<usize>();
    }
    sum
}
}