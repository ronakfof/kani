// compile-flags: --edition 2021
#![allow(unused)]
#![feature(negative_impls)]
pub fn main() {
trait DerefMut { }
impl<T: ?Sized> !DerefMut for &T { }
}