#![cfg(target_arch = "wasm32")]
#![no_std]

use profiling::prelude;
use prelude::intrinsics;
use prelude::proc_macros::main;

#[main]
fn main() {
    intrinsics::log("0");
    panic!("oops");
    intrinsics::log("1");
}
