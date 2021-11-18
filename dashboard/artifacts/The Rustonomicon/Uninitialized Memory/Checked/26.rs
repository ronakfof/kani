// compile-flags: --edition 2021
#![allow(unused)]
pub fn main() {
    let x: i32;

    if true {
        x = 1;
    } else {
        x = 2;
    }

    println!("{}", x);
}