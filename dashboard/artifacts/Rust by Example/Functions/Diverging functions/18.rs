// compile-flags: --edition 2021
#![allow(unused)]
fn some_fn() {
    ()
}

fn main() {
    let a: () = some_fn();
    println!("This function returns and you can see this line.")
}