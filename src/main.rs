#![feature(map_get_key_value)]
#![feature(never_type)]

mod compiler;
mod interpreter;
mod node;
mod display;
mod parser;
mod source;

static PROGRAM: &'static str = r"
fn fibonacci(n: u32) //-> u32 {
{
  let ~first = 1;
  let ~second = 1;

  let ~counter = 1;
  while counter == 1 => counter == n {
    let summation = first + second;
    first <=> second;
    second <=> summation;

    // `summation` contains the original `first`
    drop summation = second - first;
    counter += 1;
  }

  // Implicit drop of `first` and `counter`
  drop n = counter;
  second
}
";

//static LEXER_TEST: &'static str = r"
//@local u32 0   # 0: n
//@local u32 1   # 1: first
//@local u32 1   # 2: second
//@local u32 1   # 3: counter
//@local u32 0   # 4: summation
//~fibonacci {
//  -return'
//  restore 0       *
//0:
//  *
//  -jump 1
//  +branch = 3 0 1
//  *
//  add 4 1         *
//  add 4 2         *
//  swap 1 2        *
//  swap 2 4        *
//  *
//  +reset 4 0
//  -minus 4 1
//  -add 4 2
//  *
//  add.i 3 1       *
//  *
//  -branch.i = 3 1 0
//  +jump 0
//  *
//1:
//  -clone 0 3
//  drop 1
//  drop 3
//  drop 2
//  +return
//}
//
//@local u32 0    # 0 : fibonacci result
//~main {
//  exit' *
//  drop.i u32 35 *
//  call fibonacci
//  recall fibonacci
//  *
//  -reset 0 0
//  restore 0
//  *
//  exit
//}
//";

fn main() {
	let source_map = source::TextMap::new(PROGRAM.to_owned());
	let function = crate::parser::parse(source_map.text());
	let mut function = match function {
		Ok(mut function) => function.remove(0),
		Err(errors) => {
			for error in errors {
				crate::source::emit(&source_map, &error);
			}
			return;
		}
	};

	println!("{:#?}", function);

	use crate::interpreter::*;
	use crate::source::TextMap;
	use colored::Colorize;
	use crate::node::NodeConstruct;

	let mut visitor = crate::compiler::Translator::default();
	let elements = function.accept(&mut visitor);
	let translation_map = crate::compiler::TranslationMap::new(elements);

	let mut code_string = translation_map.text().to_owned();
	code_string += r#"
@local u64 0    # 0 : fibonacci result
~main {
  exit' *
  drop.i u64 35 *
  call fibonacci
#  recall fibonacci
  *
  -reset 0 0
  restore 0
  *
  exit
}
	"#;

	let text_map = TextMap::new(code_string);
	let operations = OperationStore::new();
	let annotations = AnnotationStore::new();

	let mut error_occurred = false;

	let (elements, errors) = ElementParser::new(text_map.text(), &annotations, &operations)
		.partition::<Vec<_>, _>(std::result::Result::is_ok);
	let elements = elements.into_iter().map(std::result::Result::unwrap).collect();
	let errors: Vec<_> = errors.into_iter().map(std::result::Result::unwrap_err).collect();
	for mut error in errors {
		crate::source::emit(&text_map, &error);
		println!("{}", format!("--> Error emitted from source: ").red().bold());
		translation_map.translate(&mut error);
		crate::source::emit(&source_map, &error);
		error_occurred = true;
		println!();
	}

	let (unit, errors) = crate::interpreter::parse(elements, &annotations);
	for mut error in errors {
		crate::source::emit(&text_map, &error);
		println!("{}", format!("--> Error emitted from source: ").red().bold());
		translation_map.translate(&mut error);
		crate::source::emit(&source_map, &error);
		error_occurred = true;
		println!();
	}

	let (unit, _metadata, errors) = compile(unit, &operations);
	for mut error in errors {
		crate::source::emit(&text_map, &error);
		println!("{}", format!("--> Error emitted from source: ").red().bold());
		translation_map.translate(&mut error);
		crate::source::emit(&source_map, &error);
		error_occurred = true;
		println!();
	}

	if error_occurred {
		return;
	}

	let mut runtime = Runtime::new(unit)
		.expect("Failed to create runtime");
//	for _ in 0..200 {
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
//	for _ in 0..200 {
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
