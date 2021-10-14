// compile-flags: --edition 2018
#![allow(unused)]
#[no_mangle]
pub extern "C" fn hello_rust() -> *const u8 {
    "Hello, world!\0".as_ptr()
}
pub fn main() {}