// compile-flags: --edition 2021
#![allow(unused)]
#![feature(negative_impls)]
fn main() {
trait DerefMut { }
impl<T: ?Sized> !DerefMut for &T { }
}