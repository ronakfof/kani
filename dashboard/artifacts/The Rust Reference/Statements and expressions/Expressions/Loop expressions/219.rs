// compile-flags: --edition 2018
#![allow(unused)]
fn main() {
'outer: loop {
    while true {
        break 'outer;
    }
}
}