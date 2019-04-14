use super::{CompilationUnit, Context};

#[derive(Debug)]
pub struct Runtime {
	compilation_unit: CompilationUnit,
	context: Context,
}

impl Runtime {
	pub fn new(compilation_unit: CompilationUnit, program_counter: usize) -> Runtime {
		Runtime {
			compilation_unit,
			context: Context::new(program_counter),
		}
	}
}
