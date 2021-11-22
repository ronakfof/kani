// rmc-check-fail
// compile-flags: --edition 2021
#![allow(unused)]
fn main() {
struct Iter<'a, T: 'a> {
    ptr: *const T,
    end: *const T,
}
}