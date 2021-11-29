// compile-flags: --edition 2015
#![allow(unused)]
#![feature(asm)]
// Multiply x by 6 using shifts and adds
fn main() {
let mut x: u64 = 4;
unsafe {
    asm!(
        "mov {tmp}, {x}",
        "shl {tmp}, 1",
        "shl {x}, 2",
        "add {x}, {tmp}",
        x = inout(reg) x,
        tmp = out(reg) _,
    );
}
assert_eq!(x, 4 * 6);
}