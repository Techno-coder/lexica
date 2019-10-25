use std::sync::Arc;

use crate::context::Context;
use crate::declaration::{InclusionTerminal, ModuleContext, ModulePath, StructurePath};
use crate::error::Diagnostic;
use crate::span::Spanned;

use super::{Ascription, Expression, FunctionContext, FunctionType, NodeError, Parameter, Pattern};

const INTRINSICS: &[&str] = &["u8", "u16", "u32", "u64", "i8", "i16", "i32,", "i64", "truth"];

pub fn resolve_function_type(context: &Context, module_context: &ModuleContext,
                             function_type: &mut FunctionType) -> Result<(), Diagnostic> {
	for parameter in &mut function_type.parameters {
		let Parameter(_, ascriptions) = &mut parameter.node;
		resolve_ascriptions(context, module_context, ascriptions)?;
	}

	resolve_ascriptions(context, module_context, &mut function_type.return_type.node)
}

pub fn resolve_function(context: &Context, module_context: &ModuleContext,
                        function: &mut FunctionContext) -> Result<(), Diagnostic> {
	for expression in function.expressions.iter_mut() {
		if let Expression::Binding(_, Some(ascriptions), _) = &mut expression.node {
			resolve_ascriptions(context, module_context, ascriptions)?;
		}
	}
	Ok(())
}

fn resolve_ascriptions(context: &Context, module_context: &ModuleContext,
                       pattern: &mut Pattern<Spanned<Ascription>>) -> Result<(), Diagnostic> {
	match pattern {
		Pattern::Wildcard => Ok(()),
		Pattern::Terminal(terminal) => resolve_ascription(context, module_context, terminal),
		Pattern::Tuple(patterns) => patterns.iter_mut().try_for_each(|pattern|
			resolve_ascriptions(context, module_context, pattern)),
	}
}

fn resolve_ascription(context: &Context, module_context: &ModuleContext,
                      ascription: &mut Spanned<Ascription>) -> Result<(), Diagnostic> {
	let Ascription(StructurePath(declaration_path)) = &mut ascription.node;
	assert!(declaration_path.module_path.is_unresolved());

	if INTRINSICS.contains(&declaration_path.identifier.as_ref()) {
		declaration_path.module_path = ModulePath::intrinsic();
		return Ok(());
	}

	let declaration_path = declaration_path.clone();
	for inclusion in &module_context.inclusions {
		match &inclusion.node.terminal {
			InclusionTerminal::Identifier(identifier) => {
				if &declaration_path.identifier == identifier {
					resolve(ascription, inclusion.node.module_path.clone())?;
				}
			}
			InclusionTerminal::Wildcard => {
				let mut candidate = declaration_path.clone();
				candidate.module_path = inclusion.node.module_path.clone();
				crate::declaration::load_modules(context, candidate.module_path.clone())
					.map_err(|error| Diagnostic::new(Spanned::new(error, inclusion.span)))?;

				let structures = &context.declarations_structure;
				if structures.contains_key(&StructurePath(candidate.clone())) {
					resolve(ascription, candidate.module_path)?;
				}
			}
		}
	}

	let Ascription(StructurePath(declaration_path)) = &mut ascription.node;
	match declaration_path.module_path.is_unresolved() {
		false => Ok(()),
		true => {
			let error = NodeError::UnresolvedResolution(declaration_path.identifier.clone());
			Err(Diagnostic::new(Spanned::new(error, ascription.span))
				.note(format!("Add an include with 'use module::{}'",
					declaration_path.identifier)))
		}
	}
}

fn resolve(ascription: &mut Spanned<Ascription>, candidate: Arc<ModulePath>) -> Result<(), Diagnostic> {
	let Ascription(StructurePath(declaration_path)) = &mut ascription.node;
	match declaration_path.module_path.is_unresolved() {
		true => Ok(declaration_path.module_path = candidate),
		false => {
			let error = NodeError::ResolutionConflict(declaration_path.identifier.clone());
			return Err(Diagnostic::new(Spanned::new(error, ascription.span)));
		}
	}
}
