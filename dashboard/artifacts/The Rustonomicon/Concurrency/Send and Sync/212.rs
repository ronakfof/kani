// compile-flags: --edition 2021
#![allow(unused)]
fn main() {
struct Carton<T>(std::ptr::NonNull<T>);
unsafe impl<T> Send for Carton<T> where Box<T>: Send {}
}