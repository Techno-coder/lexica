use crate::intrinsics::IntrinsicStore;
use crate::source::Spanned;

use super::{CompilationUnit, CompileContext, CompileError, CompileMetadata, CompileResult, Direction,
            Function, Instruction, TranslationInstruction, TranslationUnit};
use super::parser::OperationStore;

pub const ENTRY_POINT: &str = "main";

pub fn compile<'a>(translation_unit: TranslationUnit<'a>,
                   operations: &OperationStore, intrinsics: &'a IntrinsicStore)
                   -> (CompilationUnit, CompileMetadata, Vec<Spanned<CompileError<'a>>>) {
	let mut unit = CompilationUnit::default();
	let mut context = CompileContext::new(translation_unit, intrinsics);

	for (identifier, function) in &context.unit.functions {
		let mut unit_function = Function::default();
		unit_function.locals = function.locals.clone();
		context.pending_function = Some(function);

		if identifier == ENTRY_POINT {
			let function_target = &context.metadata.function_targets[identifier];
			unit.main = Some(function_target.clone());
		}

		for instruction in &function.instructions {
			let result = compile_instruction(instruction, &context, operations);
			match result {
				Ok(instruction) => unit_function.instructions.push(instruction),
				Err(error) => context.errors.push(error),
			}
		}

		unit.functions.push(unit_function);
	}

	(unit, context.metadata, context.errors)
}

fn compile_instruction<'a, 'b>(instruction: &Spanned<TranslationInstruction<'a>>,
                               context: &CompileContext<'a, 'b>, operations: &OperationStore)
                               -> CompileResult<'a, Instruction> {
	let identifier = &instruction.operation.to_string();
	let (identifier, constructor) = match operations.get(identifier) {
		Some((identifier, constructor)) => (identifier, constructor),
		None => panic!("Invalid operation encountered during compilation"),
	};

	let operation = constructor(instruction.span, &instruction.operands, context)?;
	let is_reversible = operation.reversible().is_some();

	match (is_reversible, instruction.direction, instruction.polarization) {
		(false, Direction::Reverse, _) => return Err(instruction
			.map(|_| CompileError::IrreversibleOperation(identifier))),
		(false, _, None) => return Err(instruction
			.map(|_| CompileError::MissingPolarization(identifier))),
		_ => (),
	};

	Ok(Instruction {
		identifier,
		operation,
		direction: instruction.direction,
		polarization: instruction.polarization,
	})
}
