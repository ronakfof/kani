// compile-flags: --edition 2018
#![allow(unused)]
// same meanings:
fn main() {
let a = &&  10;
let a = & & 10;

// same meanings:
let a = &&&&  mut 10;
let a = && && mut 10;
let a = & & & & mut 10;
}