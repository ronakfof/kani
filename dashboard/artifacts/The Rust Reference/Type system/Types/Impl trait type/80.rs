// compile-flags: --edition 2018
#![allow(unused)]
fn main() {
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}
}