// compile-flags: --edition 2018
#![allow(unused)]
fn main() {
struct Cookie;
let c = [Cookie, Cookie {}, Cookie, Cookie {}];
}