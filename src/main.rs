#![feature(map_get_key_value)]
#![feature(const_vec_new)]
#![feature(never_type)]

#[macro_use]
mod utility;
mod driver;
mod compiler;
mod interpreter;
mod intrinsics;
mod basic;
mod node;
mod parser;
mod source;

fn main() {
	better_panic::install();
	crate::driver::execute();
}
