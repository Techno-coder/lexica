mod compiler;
mod interpreter;
mod node;
mod display;

//static PROGRAM: &'static str = r"
//fn fibonacci(n: u32) -> u32 {
//  let ~first = 1;
//  let ~second = 1;
//
//  let ~counter = 1;
//  loop counter == 1 => counter == n {
//    let summation = first + second;
//    first <=> second;
//    second <=> summation;
//
//    // `summation` contains the original `first`
//    drop summation = second - first;
//    counter += 1;
//  }
//
//  # Implicit drop of `first` and `counter`
//  drop n = counter;
//  second
//}
//";

// TODO: Execution in the normal direction is desired when reversing sometimes

fn main() {
	use crate::interpreter::*;
	use crate::node::*;

	let function = construct();
	let mut context = Context::default();
	context.register_binding(Identifier("n"), 10);

	let mut runtime = RuntimeInterpreter::new(context);
	runtime.stack_dependency(Dependency::advance(&function), Direction::Advance);

	for _ in 0..258 {
		runtime.step(Direction::Advance);
	}

	println!("REVERSING");

	runtime.run(Direction::Reverse);
	println!("{:#?}", runtime);

//	let value = runtime.run(Direction::Advance);
//	println!("{:#?}", value);

//	runtime.queue_node(Dependency::reverse(&function));
//	while let Some(_) = runtime.step(Direction::Reverse) {
//		println!("{:#?}", runtime);
//	}

//	println!("Fibonacci #10: {}", runtime.interpret(&function, &mut context));
//	println!("Beginning time reversal ...");
//	runtime.reverse(&function, &mut context);
//	println!("Context: {:#?}", context);
}

fn pause() {
	use std::io;
	use std::io::prelude::*;

	let mut stdin = io::stdin();
	let mut stdout = io::stdout();

	// We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
	write!(stdout, "Press any key to continue...").unwrap();
	stdout.flush().unwrap();

	// Read a single byte and discard
	let _ = stdin.read(&mut [0u8]).unwrap();
}
