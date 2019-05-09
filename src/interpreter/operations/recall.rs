use std::fmt;

use crate::source::Span;

use super::{CallFrame, CompilationUnit, Context, Direction, GenericOperation, InstructionTarget,
            InterpreterResult, Operand, Operation, Operational, ParserContext, ParserError,
            ParserResult, Reversible, TranslationUnit};

#[derive(Debug)]
pub struct Recall {
	target: InstructionTarget,
	reverse_target: InstructionTarget,
}

impl Recall {
	pub fn new(target: InstructionTarget, reverse_target: InstructionTarget) -> Recall {
		Recall { target, reverse_target }
	}
}

impl Operational for Recall {
	fn compile<'a>(_: &Span, operands: &Vec<Operand<'a>>, _: &ParserContext,
	               unit: &TranslationUnit) -> ParserResult<'a, GenericOperation> {
		use super::unit_parsers::*;
		let target = target(&operands[0])?;
		let function = unit.functions.get(&target)
			.ok_or(operands[0].map(|_| ParserError::UndefinedFunction(target)))?;
		let reverse_target = function.reverse_target.clone()
			.ok_or(operands[0].map(|_| ParserError::IrreversibleCall))?;
		Ok(Box::new(Recall::new(function.target.clone(), reverse_target)))
	}
}

impl Operation for Recall {
	fn execute(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()> {
		let label = unit.reverse_targets.get(&self.reverse_target)
			.expect("Reverse function label does not exist");
		let function = unit.function_targets.get(label).unwrap();
		context.push_frame(CallFrame::construct(&function, Direction::Reverse, context.program_counter()));
		context.set_next_instruction(|| Ok(self.reverse_target.clone()));
		Ok(())
	}

	fn reversible(&self) -> Option<&Reversible> {
		Some(self)
	}
}

impl Reversible for Recall {
	fn reverse(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()> {
		let function = unit.function_targets.get(&self.target)
			.expect("Function label does not exist");
		context.push_frame(CallFrame::construct(&function, Direction::Reverse, context.program_counter()));
		context.set_next_instruction(|| Ok(self.target.clone()));
		Ok(())
	}
}

impl fmt::Display for Recall {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self.target)
	}
}
