// compile-flags: --edition 2021
#![allow(unused)]
fn main() {
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
}