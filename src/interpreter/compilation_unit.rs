use super::{FunctionLabel, Instruction};

#[derive(Debug)]
pub struct CompilationUnit {
	instructions: Vec<Instruction>,
	functions: Vec<FunctionLabel>,
}
