// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
fn main() {
    assert!(true);
    assert!(true || false);
    assert!(!false);

    let a = true;
    let b = false;
    let c = a || b;
    let d = c && a;
    assert!(d && true);
    assert!(!b && d);

    let e: bool = kani::any();
    assert!(e || !e);
}
