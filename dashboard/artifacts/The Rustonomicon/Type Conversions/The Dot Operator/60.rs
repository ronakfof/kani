// compile-flags: --edition 2021
#![allow(unused)]
pub fn main() {
fn do_stuff<T: Clone>(value: &T) {
    let cloned = value.clone();
}
}