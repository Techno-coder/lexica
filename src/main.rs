#![feature(map_get_key_value)]
#![feature(const_vec_new)]
#![feature(never_type)]

mod driver;
mod compiler;
mod interpreter;
mod node;
mod display;
mod parser;
mod source;

fn main() {
	let _ = crate::driver::execute();
}
