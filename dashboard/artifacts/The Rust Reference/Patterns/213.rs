// compile-flags: --edition 2021
#![allow(unused)]
pub fn main() {
let a = Some(10);
match a {
    None => (),
    Some(value) => (),
}

match a {
    None => (),
    Some(ref value) => (),
}
}