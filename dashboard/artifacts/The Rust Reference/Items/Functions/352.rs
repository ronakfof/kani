// compile-flags: --edition 2021
#![allow(unused)]
pub fn main() {
fn len(
    #[cfg(windows)] slice: &[u16],
    #[cfg(not(windows))] slice: &[u8],
) -> usize {
    slice.len()
}
}