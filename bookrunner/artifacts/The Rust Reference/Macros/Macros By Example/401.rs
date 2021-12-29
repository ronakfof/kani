// compile-flags: --edition 2018
#![allow(unused)]
fn main() {
#[macro_export]
macro_rules! call_foo {
    () => { $crate::foo() };
}

fn foo() {}
}