use std::sync::Arc;

use crate::basic::Reversibility;
use crate::context::Context;
use crate::declaration::FunctionPath;
use crate::error::Diagnostic;
use crate::extension::StringExtension;
use crate::span::{Span, Spanned};

use super::Command;

#[derive(Debug)]
pub struct CommandBasic;

impl Command for CommandBasic {
	fn execute(&self, context: &Context, string: &str) -> Result<String, Diagnostic> {
		let arguments: Vec<_> = string.split_whitespace().collect();
		match arguments.len() {
			0 => Ok("Expected reversibility argument".to_owned()),
			1 => Ok("Expected function path".to_owned()),
			_ => {
				let path = function_path(arguments[1])?;
				Ok(crate::basic::function(context, &path, match arguments[0] {
					"reversible" => Reversibility::Reversible,
					"entropic" => Reversibility::Entropic,
					_ => return Ok("Expected argument of 'reversible' or 'entropic'".to_owned()),
				})?.to_string())
			}
		}
	}

	fn symbols(&self, context: &Context, string: &str) -> Vec<String> {
		let arguments: Vec<_> = string.split_whitespace().collect();
		match arguments.len() {
			0 | 1 => vec!["reversible".to_owned(), "entropic".to_owned()],
			_ => function_candidates(context, arguments[1]),
		}
	}
}

#[derive(Debug)]
pub struct CommandEvaluate;

impl Command for CommandEvaluate {
	fn execute(&self, context: &Context, string: &str) -> Result<String, Diagnostic> {
		let path = function_path(string)?;
		let function_type = crate::node::function_type(context, &path)?;
		match function_type.parameters.is_empty() {
			false => Ok("Evaluated functions must have zero arity".to_owned()),
			true => crate::evaluation::function(context, &path, Vec::new())
				.and_then(|item| Ok(item.collapse().map_err(|error|
					Diagnostic::new(Spanned::new(error, Span::INTERNAL)))?.to_string()))
		}
	}

	fn symbols(&self, context: &Context, string: &str) -> Vec<String> {
		function_candidates(context, string)
	}
}

fn function_path(string: &str) -> Result<Spanned<Arc<FunctionPath>>, Diagnostic> {
	let lexer = &mut crate::lexer::Lexer::new(string,
		0, crate::source::SourceKey::INTERNAL);
	Ok(crate::parser::path(lexer)?.map(|mut path| {
		path.module_path = path.module_path.tail();
		Arc::new(FunctionPath(path))
	}))
}

fn function_candidates(context: &Context, path: &str) -> Vec<String> {
	context.declarations_function.clone().into_iter()
		.map(|(function_path, _)| function_path.to_string())
		.filter(|candidate| candidate.as_str().prefix_equal(path))
		.collect()
}
