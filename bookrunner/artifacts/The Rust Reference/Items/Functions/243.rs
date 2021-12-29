// compile-flags: --edition 2018
#![allow(unused)]
// Source
fn main() {
async fn example(x: &str) -> usize {
    x.len()
}
}