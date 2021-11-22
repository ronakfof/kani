// compile-flags: --edition 2021
// rmc-flags: --cbmc-args --unwind 4
#![allow(unused)]
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);
}