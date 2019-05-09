use crate::source::Spanned;

use super::{CompilationUnit, CompileError, CompileMetadata, CompileContext, Function,
            GenericOperation, Instruction, OperationalStore, TranslationUnit};

pub fn compile(translation_unit: TranslationUnit, operations: &OperationalStore)
               -> (CompilationUnit, CompileMetadata, Vec<Spanned<CompileError>>) {
	let mut unit = CompilationUnit::default();
	let mut context = CompileContext::new(translation_unit);
	let mut metadata = CompileMetadata::default();

	for (identifier, function) in &context.unit.functions {
		metadata.function_indexes.insert(identifier.clone(), unit.functions.len());
		context.pending_function = Some(function);

		let mut unit_function = Function::default();
		unit_function.locals = function.locals.clone();

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
				},
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

	(unit, metadata, context.errors)
}
