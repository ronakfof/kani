// compile-flags: --edition 2021
#![allow(unused)]
pub fn main() {
fn is_unix_platform() -> bool {
    #[cfg(unix)] { true }
    #[cfg(not(unix))] { false }
}
}