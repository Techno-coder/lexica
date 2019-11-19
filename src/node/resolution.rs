use std::sync::Arc;

use crate::context::Context;
use crate::declaration::{DeclarationPath, FunctionPath, InclusionTerminal,
	ModuleContext, ModulePath, StructurePath};
use crate::error::Diagnostic;
use crate::intrinsic::Intrinsic;
use crate::node::Structure;
use crate::span::{Span, Spanned};

use super::{Ascription, Expression, FunctionContext, FunctionType, NodeError, Parameter, Pattern};

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
		match &mut expression.node {
			Expression::Binding(_, Some(ascriptions), _) =>
				resolve_ascriptions(context, module_context, ascriptions)?,
			Expression::FunctionCall(function_path, _, _) =>
				resolve_function_path(context, module_context, function_path)?,
			Expression::Structure(structure_path, _) =>
				resolve_structure_path(context, module_context, structure_path)?,
			_ => (),
		}
	}
	Ok(())
}

pub fn resolve_structure(context: &Context, module_context: &ModuleContext,
                         structure: &mut Structure) -> Result<(), Diagnostic> {
	structure.fields.values_mut().try_for_each(|ascriptions|
		resolve_ascriptions(context, module_context, ascriptions))
}

pub fn resolve_structure_path(context: &Context, module_context: &ModuleContext,
                              structure_path: &mut Spanned<StructurePath>) -> Result<(), Diagnostic> {
	let StructurePath(declaration_path) = &mut structure_path.node;
	if !declaration_path.module_path.any_unresolved() { return Ok(()); }

	let structures = &context.declarations_structure;
	resolve_declaration(module_context, declaration_path, structure_path.span,
		&mut |candidate| structures.contains_key(&StructurePath(candidate)))
}

fn resolve_ascriptions(context: &Context, module_context: &ModuleContext,
                       pattern: &mut Pattern<Spanned<Ascription>>) -> Result<(), Diagnostic> {
	pattern.apply(&mut |terminal| resolve_ascription(context, module_context, terminal))
}

fn resolve_ascription(context: &Context, module_context: &ModuleContext,
                      ascription: &mut Spanned<Ascription>) -> Result<(), Diagnostic> {
	let declaration_path = match &mut ascription.node {
		Ascription::Structure(StructurePath(declaration_path), templates) => {
			templates.iter_mut().try_for_each(|template|
				resolve_ascriptions(context, module_context, template))?;
			declaration_path
		}
		Ascription::Reference(_, _, ascription) =>
			return resolve_ascriptions(context, module_context, ascription),
		_ => return Ok(()),
	};

	if !declaration_path.module_path.any_unresolved() { return Ok(()); }
	let intrinsic = Intrinsic::parse(&declaration_path.identifier.as_ref());
	if declaration_path.module_path.is_unresolved() && intrinsic.is_some() {
		declaration_path.module_path = ModulePath::intrinsic();
		return Ok(());
	}

	let structures = &context.declarations_structure;
	resolve_declaration(module_context, declaration_path, ascription.span,
		&mut |candidate| structures.contains_key(&StructurePath(candidate)))
}

fn resolve_function_path(context: &Context, module_context: &ModuleContext,
                         function_path: &mut Spanned<FunctionPath>) -> Result<(), Diagnostic> {
	let FunctionPath(declaration_path) = &mut function_path.node;
	if !declaration_path.module_path.any_unresolved() { return Ok(()); }

	let functions = &context.declarations_function;
	resolve_declaration(module_context, declaration_path, function_path.span,
		&mut |candidate| functions.contains_key(&FunctionPath(candidate)))
}

fn resolve_declaration<F>(module_context: &ModuleContext, declaration_path: &mut DeclarationPath,
                          span: Span, predicate: &mut F) -> Result<(), Diagnostic>
	where F: FnMut(DeclarationPath) -> bool {
	if declaration_path.module_path.head().map(|head| head.is_root()).unwrap_or(false) {
		declaration_path.module_path = declaration_path.module_path.clone().tail();
	}

	let comparison_path = declaration_path.clone();
	for inclusion in &module_context.inclusions {
		match &inclusion.node.terminal {
			InclusionTerminal::Identifier(identifier) => {
				if &comparison_path.head() == identifier {
					resolve(declaration_path, inclusion.node.module_path.clone(), span)?;
				}
			}
			InclusionTerminal::Wildcard => {
				let mut candidate = comparison_path.clone();
				candidate.module_path = inclusion.node.module_path.clone()
					.append(&candidate.module_path);
				if predicate(candidate.clone()) {
					resolve(declaration_path, inclusion.node.module_path.clone(), span)?;
				}
			}
		}
	}

	match declaration_path.module_path.any_unresolved() {
		false => Ok(()),
		true => {
			let error = NodeError::UnresolvedResolution(declaration_path.clone());
			let note = format!("Add an include with 'use module::{}'", declaration_path.head());
			Err(Diagnostic::new(Spanned::new(error, span)).note(note))
		}
	}
}

fn resolve(declaration_path: &mut DeclarationPath, candidate: Arc<ModulePath>,
           span: Span) -> Result<(), Diagnostic> {
	match declaration_path.module_path.any_unresolved() {
		true => {
			let module = candidate.append(&declaration_path.module_path);
			Ok(declaration_path.module_path = module)
		}
		false => {
			let error = NodeError::ResolutionConflict(declaration_path.clone());
			return Err(Diagnostic::new(Spanned::new(error, span)));
		}
	}
}
