// rmc-check-fail
// compile-flags: --edition 2021
#![allow(unused)]
fn main() {
let mut x = [1, 2, 3];
let a = &mut x[0];
let b = &mut x[1];
println!("{} {}", a, b);
}