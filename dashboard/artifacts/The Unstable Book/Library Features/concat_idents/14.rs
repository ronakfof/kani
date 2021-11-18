// compile-flags: --edition 2021
#![allow(unused)]
#![feature(concat_idents)]

pub fn main() {
    fn foobar() -> u32 { 23 }
    let f = concat_idents!(foo, bar);
    assert_eq!(f(), 23);
}