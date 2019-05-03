#![feature(try_from)]

mod compiler;
mod interpreter;
mod node;
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
+fibonacci:
  -return'
  restore 0       *
.0:
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
.1:
  -clone 0 3
  drop 1
  drop 3
  drop 2
  +return
-fibonacci^

@local u32 0    # 0 : fibonacci result
+main:
  exit' *
  drop.i u32 10 *
  call fibonacci
  recall fibonacci
  *
  -reset 0 0
  restore 0
  *
  exit
-main^
";

fn main() {
	use crate::interpreter::{AnnotationMap, parse, Runtime, RuntimeStep};
	use crate::source::TextMap;
	use colored::Colorize;
	use crate::interpreter::Direction;

	let _function = compiler::construct();
	let text_map = TextMap::new(LEXER_TEST.to_owned());

	let mut annotation_map = AnnotationMap::default();
	annotation_map.register("local".to_owned(), Box::new(crate::interpreter::annotations::LocalAnnotation));

	let result = match parse(text_map.text(), &annotation_map) {
		Ok(result) => result.compile(),
		Err(errors) => {
			for error in errors {
				crate::source::emit(&text_map, error);
				println!();
			}
			return;
		}
	};

	let mut runtime = Runtime::new(result)
		.expect("Failed to create runtime");
	loop {
		println!("{}", format!("[ {} ]", runtime.current_instruction()).blue().bold());
		match runtime.force_step(Direction::Advance) {
			Ok(RuntimeStep::Halted) => {
				println!("{}: {:#?}", "Exit".green().bold(), runtime.context());
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
		println!("{}", format!("[ {} ]", runtime.current_instruction()).blue().bold());
		match runtime.force_step(Direction::Reverse) {
			Ok(RuntimeStep::Halted) => {
				println!("{}: {:#?}", "Exit".green().bold(), runtime.context());
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
