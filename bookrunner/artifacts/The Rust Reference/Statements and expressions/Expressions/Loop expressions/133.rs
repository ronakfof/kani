// compile-flags: --edition 2021
#![allow(unused)]
fn main() {
let v = &["apples", "cake", "coffee"];

for text in v {
    println!("I like {}.", text);
}
}