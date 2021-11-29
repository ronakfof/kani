// compile-flags: --edition 2015
// rmc-flags: --cbmc-args --unwind 4 --object-bits 9
#![allow(unused)]
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Result<Vec<_>, _> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);
}