// compile-flags: --edition 2018
#![allow(unused)]
fn main() {
use std::future::Future;
// Desugared
fn example<'a>(x: &'a str) -> impl Future<Output = usize> + 'a {
    async move { x.len() }
}
}