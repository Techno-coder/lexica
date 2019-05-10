#![feature(map_get_key_value)]
#![feature(never_type)]

//mod compiler;
mod interpreter;
//mod node;
mod display;
mod source;

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

static LEXER_TEST: &'static str = r"
@local u32 0   # 0: n
@local u32 1   # 1: first
@local u32 1   # 2: second
@local u32 1   # 3: counter
@local u32 0   # 4: summation
~fibonacci {
  -return'
  restore 0       *
0:
  *
  -jump 1
  +branch = 3 0 1
  *
  add 4 1         *
  add 4 2         *
  swap 1 2        *
  swap 2 4        *
  *
  +reset 4 0
  -minus 4 1
  -add 4 2
  *
  add.i 3 1       *
  *
  +jump 0
  -branch.i = 3 1 0
  *
1:
  -clone 0 3
  drop 1
  drop 3
  drop 2
  +return
}

@local u32 0    # 0 : fibonacci result
~main {
  exit' *
  drop.i u32 35 *
  call fibonacci
  recall fibonacci
  *
  -reset 0 0
  restore 0
  *
  exit
}
";

fn main() {
	use crate::interpreter::*;
	use crate::source::TextMap;
	use colored::Colorize;

	let text_map = TextMap::new(LEXER_TEST.to_owned());
	let operations = OperationStore::new();

	let mut annotations = AnnotationStore::default();
	annotations.register("local".to_owned(), Box::new(crate::interpreter::annotations::LocalAnnotation));

	let mut error_occurred = false;

	let (elements, errors) = ElementParser::new(text_map.text(), &annotations, &operations)
		.partition::<Vec<_>, _>(|result| result.is_ok());
	let elements = elements.into_iter().map(|element| element.unwrap()).collect();
	let errors: Vec<_> = errors.into_iter().map(|error| error.unwrap_err()).collect();
	for error in errors {
		crate::source::emit(&text_map, error);
		error_occurred = true;
		println!();
	}

	let (unit, errors) = parse(elements, &annotations);
	for error in errors {
		crate::source::emit(&text_map, error);
		error_occurred = true;
		println!();
	}

	let (unit, _metadata, errors) = compile(unit, &operations);
	for error in errors {
		crate::source::emit(&text_map, error);
		error_occurred = true;
		println!();
	}

	if error_occurred {
		return;
	}

	let mut runtime = Runtime::new(unit)
		.expect("Failed to create runtime");
	loop {
		println!("{}", format!("[ {} ]", runtime.current_instruction().unwrap()).blue().bold());
		match runtime.force_step(Direction::Advance) {
			Ok(RuntimeStep::Halted) => {
				println!("{}: {:#?}", "Halt".green().bold(), runtime.context());
				break;
			}
			Err(error) => {
				println!("{} {}", "[Error]".red().bold(), format!("{}", error).red());
				return;
			}
			_ => (),
		}
	}

	println!("{}", "REVERSING".red().bold());
	loop {
		println!("{}", format!("[ {} ]", runtime.current_instruction().unwrap()).blue().bold());
		match runtime.force_step(Direction::Reverse) {
			Ok(RuntimeStep::Halted) => {
				println!("{}: {:#?}", "Halt".green().bold(), runtime.context());
				break;
			}
			Err(error) => {
				println!("{} {}", "[Error]".red().bold(), format!("{}", error).red());
				return;
			}
			_ => (),
		}
	}
}
