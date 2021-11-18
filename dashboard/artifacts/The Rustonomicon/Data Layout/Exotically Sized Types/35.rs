// compile-flags: --edition 2021
#![allow(unused)]
// Can't be stored on the stack directly
pub fn main() {
struct MySuperSlice {
    info: u32,
    data: [u8],
}
}