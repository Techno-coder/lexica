#![feature(map_get_key_value)]
#![feature(const_vec_new)]
#![feature(never_type)]

mod driver;
mod compiler;
mod interpreter;
mod intrinsics;
mod node;
mod display;
mod parser;
mod source;

fn main() {
	better_panic::install();
	crate::driver::execute();
}
