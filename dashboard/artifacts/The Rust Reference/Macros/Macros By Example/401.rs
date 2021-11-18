// compile-flags: --edition 2021
#![allow(unused)]
pub fn main() {
#[macro_export]
macro_rules! call_foo {
    () => { $crate::foo() };
}

fn foo() {}
}