// compile-flags: --edition 2018
#![allow(unused)]
fn main() {
fn fn_call() {}
let _: () = {
    fn_call();
};

let five: i32 = {
    fn_call();
    5
};

assert_eq!(5, five);
}