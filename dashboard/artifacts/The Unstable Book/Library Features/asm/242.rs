// compile-flags: --edition 2021
#![allow(unused)]
#![feature(asm)]
fn main() {
let ebx: u32;
let ecx: u32;

unsafe {
    asm!(
        "cpuid",
        // EAX 4 selects the "Deterministic Cache Parameters" CPUID leaf
        inout("eax") 4 => _,
        // ECX 0 selects the L0 cache information.
        inout("ecx") 0 => ecx,
        lateout("ebx") ebx,
        lateout("edx") _,
    );
}

println!(
    "L0 Cache: {}",
    ((ebx >> 22) + 1) * (((ebx >> 12) & 0x3ff) + 1) * ((ebx & 0xfff) + 1) * (ecx + 1)
);
}