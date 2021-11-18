// compile-flags: --edition 2021
#![allow(unused)]
pub fn main() {
use std::marker;

struct Iter<'a, T: 'a> {
    ptr: *const T,
    end: *const T,
    _marker: marker::PhantomData<&'a T>,
}
}