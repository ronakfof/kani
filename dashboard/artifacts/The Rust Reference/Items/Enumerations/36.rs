// compile-flags: --edition 2018
#![allow(unused)]
fn main() {
enum Animal {
    Dog,
    Cat,
}

let mut a: Animal = Animal::Dog;
a = Animal::Cat;
}