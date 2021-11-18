// compile-flags: --edition 2021
#![allow(unused)]
pub fn main() {
#[macro_use]
mod inner {
    macro_rules! m {
        () => {};
    }
}

m!();
}