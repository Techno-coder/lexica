use std::sync::Arc;

use crate::context::Context;
use crate::declaration::{FunctionPath, StructurePath};
use crate::error::Diagnostic;
use crate::span::Spanned;

use super::{FunctionType, NodeFunction, Structure};

pub fn function_type(context: &Context, function_path: &Spanned<Arc<FunctionPath>>)
                     -> Result<Arc<FunctionType>, Diagnostic> {
	if let Some(function_type) = context.function_types.get(&function_path.node) {
		return Ok(function_type.clone());
	}

	let FunctionPath(declaration_path) = function_path.node.as_ref();
	let mut function_type = crate::parser::function_type(context, function_path)?;
	super::shadow::shadow_function_type(&mut function_type)?;

	super::resolution::resolve_function_type(context, &context.module_contexts
		.get(&declaration_path.module_path).unwrap(), &mut function_type)?;

	let function_type = Arc::new(function_type);
	context.function_types.insert(function_path.node.clone(), function_type.clone());
	Ok(function_type)
}

pub fn function(context: &Context, function_path: &Spanned<Arc<FunctionPath>>)
                -> Result<Arc<NodeFunction>, Diagnostic> {
	if let Some(function) = context.node_functions.get(&function_path.node) {
		return Ok(function.clone());
	}

	let FunctionPath(declaration_path) = function_path.node.as_ref();
	let mut function = crate::parser::function(context, function_path)?;
	super::shadow::shadow_function(&mut function)?;

	super::resolution::resolve_function(context, &context.module_contexts
		.get(&declaration_path.module_path).unwrap(), &mut function.context)?;

	let function = Arc::new(function);
	context.node_functions.insert(function_path.node.clone(), function.clone());
	Ok(function)
}

pub fn structure(context: &Context, structure_path: &Spanned<Arc<StructurePath>>)
                 -> Result<Arc<Structure>, Diagnostic> {
	if let Some(structure) = context.node_structures.get(&structure_path.node) {
		return Ok(structure.clone());
	}

	let StructurePath(declaration_path) = structure_path.node.as_ref();
	let mut structure = crate::parser::structure(context, structure_path)?;
	super::resolution::resolve_structure(context, &context.module_contexts
		.get(&declaration_path.module_path).unwrap(), &mut structure)?;

	let structure = Arc::new(structure);
	context.node_structures.insert(structure_path.node.clone(), structure.clone());
	Ok(structure)
}
