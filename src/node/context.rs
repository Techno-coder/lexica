use std::sync::Arc;

use crate::context::Context;
use crate::declaration::FunctionPath;
use crate::error::Diagnostic;
use crate::node::{Function, FunctionType};
use crate::span::Spanned;

pub fn function_type(context: &Context, function_path: &Spanned<Arc<FunctionPath>>)
                     -> Result<Arc<FunctionType>, Diagnostic> {
	if let Some(function_type) = context.function_types.read().get(&function_path.node) {
		return Ok(function_type.clone());
	}

	crate::parser::function_type(context, function_path)
}


pub fn function(context: &Context, function_path: &Spanned<Arc<FunctionPath>>)
                -> Result<Arc<Function>, Diagnostic> {
	if let Some(function) = context.node_functions.read().get(&function_path.node) {
		return Ok(function.clone());
	}

	crate::parser::function(context, function_path)
}
