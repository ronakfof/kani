// rmc-check-fail
// compile-flags: --edition 2021
#![allow(unused)]
pub fn main() {
struct Struct;
trait HasAssocType { type Ty; }
impl<'a> HasAssocType for Struct {
    type Ty = &'a Struct;
}
}