use std::fmt;

use crate::source::Span;

use super::{CallFrame, CompilationUnit, Context, Direction, GenericOperation, InstructionTarget,
            InterpreterError, InterpreterResult, Operand, Operation, Operational, ParserContext,
            ParserError, ParserResult, TranslationUnit};

#[derive(Debug)]
pub struct Call {
	target: InstructionTarget,
	reverse_target: Option<InstructionTarget>,
}

impl Call {
	pub fn new(target: InstructionTarget, reverse_target: Option<InstructionTarget>) -> Call {
		Call { target, reverse_target }
	}

	pub fn reversible(&self) -> bool {
		self.reverse_target.is_some()
	}
}

impl Operational for Call {
	fn parse<'a>(span: &Span, operands: &Vec<Operand<'a>>, context: &ParserContext,
	             unit: &TranslationUnit) -> ParserResult<'a, GenericOperation> {
		use super::unit_parsers::*;
		let function = base_function(context, unit, span);
		let target = target(&operands[0])?;
		let (target, reverse_target) = unit
			.functions.get(&target)
			.map(|function| (function.target.clone(), function.reverse_target.clone()))
			.ok_or(operands[0].map(|_| ParserError::UndefinedFunction(target)))?;
		Ok(Box::new(Call::new(target, reverse_target)))
	}
}

impl Operation for Call {
	fn execute(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()> {
		let function = unit.function_labels.get(&self.target)
			.expect("Function label does not exist");
		context.push_frame(CallFrame::construct(&function, Direction::Advance, context.program_counter()));
		context.set_next_instruction(|| Ok(self.target.clone()));
		Ok(())
	}

	fn reverse(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()> {
		let reverse_target = self.reverse_target.as_ref()
			.ok_or(InterpreterError::Irreversible)?;
		let label = unit.reverse_labels.get(reverse_target)
			.expect("Reverse function label does not exist");
		let function = unit.function_labels.get(label).unwrap();
		context.push_frame(CallFrame::construct(&function, Direction::Advance, context.program_counter()));
		context.set_next_instruction(|| Ok(reverse_target.clone()));
		Ok(())
	}
}

impl fmt::Display for Call {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self.target)
	}
}
