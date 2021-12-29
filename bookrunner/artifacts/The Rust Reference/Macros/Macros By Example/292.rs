// compile-flags: --edition 2018
#![allow(unused)]
fn main() {
#[macro_use]
mod inner {
    macro_rules! m {
        () => {};
    }
}

m!();
}