// compile-flags: --edition 2021
#![allow(unused)]
fn main() {
struct Vec<T> {
    data: *const T, // *const for variance!
    len: usize,
    cap: usize,
}
}