// compile-flags: --edition 2021
#![allow(unused)]
pub fn main() {
let mut sum = 0;
for n in 1..11 {
    sum += n;
}
assert_eq!(sum, 55);
}