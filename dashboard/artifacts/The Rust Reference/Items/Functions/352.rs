// compile-flags: --edition 2018
#![allow(unused)]
fn main() {
fn len(
    #[cfg(windows)] slice: &[u16],
    #[cfg(not(windows))] slice: &[u8],
) -> usize {
    slice.len()
}
}