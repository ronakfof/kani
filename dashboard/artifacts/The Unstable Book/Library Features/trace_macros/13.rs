// compile-flags: --edition 2021
#![allow(unused)]
#![feature(trace_macros)]

fn main() {
    trace_macros!(true);
    println!("Hello, Rust!");
    trace_macros!(false);
}