use crate::source::Spanned;

use super::{CompilationUnit, CompileContext, CompileError, CompileMetadata, Function,
            GenericOperation, Instruction, OperationalStore, TranslationUnit};

const ENTRY_POINT: &'static str = "main";

pub fn compile<'a>(translation_unit: TranslationUnit<'a>, operations: &OperationalStore)
                   -> (CompilationUnit, CompileMetadata, Vec<Spanned<CompileError<'a>>>) {
	let mut unit = CompilationUnit::default();
	let mut context = CompileContext::new(translation_unit);

	for (identifier, function) in &context.unit.functions {
		let mut unit_function = Function::default();
		unit_function.locals = function.locals.clone();
		context.pending_function = Some(function);

		if identifier == ENTRY_POINT {
			let function_target = &context.metadata.function_targets[identifier];
			unit.main = Some(function_target.clone());
		}

		for instruction in &function.instructions {
			let identifier = &format!("{}", instruction.operation);
			let (identifier, constructor) = match operations.get(identifier) {
				Some((identifier, constructor)) => (identifier, constructor),
				None => panic!("Invalid operation encountered during compilation"),
			};

			let operation = constructor(&instruction.span, &instruction.operands, &context);
			let operation: GenericOperation = match operation {
				Ok(operation) => operation,
				Err(error) => {
					context.errors.push(error);
					continue;
				}
			};

			let instruction = Instruction {
				identifier,
				operation,
				direction: instruction.direction,
				polarization: instruction.polarization,
			};
			unit_function.instructions.push(instruction);
		}

		unit.functions.push(unit_function);
	}

	(unit, context.metadata, context.errors)
}
