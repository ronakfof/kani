// compile-flags: --edition 2018
#![allow(unused)]
fn main() {
#[derive(PartialEq, Clone)]
struct Foo<T> {
    a: i32,
    b: T,
}
}