// compile-flags: --edition 2021
#![allow(unused)]
fn main() {
fn do_stuff<T: Clone>(value: &T) {
    let cloned = value.clone();
}
}