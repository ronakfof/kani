// compile-flags: --edition 2021
#![allow(unused)]
pub fn main() {
enum Link {
    Next(Box<Link>),
    None,
}
}