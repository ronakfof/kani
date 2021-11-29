// compile-flags: --edition 2018
#![allow(unused)]
fn main() {
#[export_name = "exported_symbol_name"]
pub fn name_in_rust() { }
}