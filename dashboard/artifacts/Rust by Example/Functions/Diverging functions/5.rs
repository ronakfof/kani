// compile-flags: --edition 2021
#![allow(unused)]
pub fn main() {
fn foo() -> ! {
    panic!("This call never returns.");
}
}