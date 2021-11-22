// compile-flags: --edition 2021
#![allow(unused)]
// `F` must be generic.
fn main() {
fn apply<F>(f: F) where
    F: FnOnce() {
    f();
}
}