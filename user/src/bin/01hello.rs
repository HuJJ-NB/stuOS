#![no_std]
#![no_main]

#[macro_use]
extern crate base_lib;

use base_lib::get_taskinfo;

#[no_mangle]
fn main() -> i32 {
	println!("Hello, world! at {}", get_taskinfo());
	0
}
