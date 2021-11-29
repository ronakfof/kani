// compile-flags: --edition 2018
#![allow(unused)]
struct Foo<'a> { x: &'a i8 }

fn main() {
    Foo { x: &mut 42 };
}