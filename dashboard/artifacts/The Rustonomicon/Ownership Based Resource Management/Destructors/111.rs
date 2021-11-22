// compile-flags: --edition 2021
#![allow(unused)]
fn main() {
enum Link {
    Next(Box<Link>),
    None,
}
}