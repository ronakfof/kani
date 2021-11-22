// compile-flags: --edition 2021
#![allow(unused)]
fn main() {
fn foo() -> ! {
    panic!("This call never returns.");
}
}