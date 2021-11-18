// compile-flags: --edition 2021
#![allow(unused)]
// Source
pub fn main() {
async fn example(x: &str) -> usize {
    x.len()
}
}