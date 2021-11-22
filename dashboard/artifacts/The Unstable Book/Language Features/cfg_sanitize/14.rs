// compile-flags: --edition 2021
#![allow(unused)]
#![feature(cfg_sanitize)]

fn main() {
#[cfg(sanitize = "thread")]
fn a() {
    // ...
}

#[cfg(not(sanitize = "thread"))]
fn a() {
    // ...
}

fn b() {
    if cfg!(sanitize = "leak") {
        // ...
    } else {
        // ...
    }
}
}