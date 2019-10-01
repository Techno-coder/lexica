use std::sync::Arc;

use crate::context::Context;
use crate::declaration::{self, FunctionPath};
use crate::error::Diagnostic;
use crate::lexer::{Lexer, Token};
use crate::node::{AscriptionPattern, BindingPattern, FunctionContext};
use crate::span::Spanned;

use super::ParserError;

pub fn function(context: &Context, function_path: Spanned<Arc<FunctionPath>>) -> Result<(), Diagnostic> {
	let FunctionPath(declaration_path) = &*function_path.node;
	declaration::load_modules(context, declaration_path.module_path.clone())
		.map_err(|error| Diagnostic::new(Spanned::new(error, function_path.span)))?;

	let declarations_function = context.declarations_function.read();
	let declaration = declarations_function.get(&function_path.node).ok_or_else(||
		Diagnostic::new(function_path.clone().map(|path| ParserError::UndefinedFunction(path))))?;

	let source = declaration.source.get(context);
	let lexer = &mut Lexer::new(source.read_string()
		.map_err(|error| Diagnostic::new(Spanned::new(error, function_path.span)))?,
		*declaration.line_offset, declaration.source);

	super::expect(lexer, Token::Function)?;
	let identifier = super::identifier(lexer)?;
	let parameters = parameters(lexer).map_err(|diagnostic|
		diagnostic.note("In parsing function parameters".to_owned()))?;
	// TODO Parse rest of function

	let mut context = FunctionContext::new(function_path.node.clone());
	let expression = super::expression(&mut context, lexer)?;
	Ok(())
}

fn parameters(lexer: &mut Lexer) -> Result<Vec<(BindingPattern, AscriptionPattern)>, Diagnostic> {
	let mut parameters = Vec::new();
	super::expect(lexer, Token::ParenthesisOpen)?;
	while lexer.peek().node != Token::ParenthesisClose {
		let pattern = super::pattern(lexer, &super::binding_variable)?;
		super::expect(lexer, Token::Separator)?;
		let ascription = super::pattern(lexer, &super::ascription)?;
		parameters.push((pattern, ascription));

		match lexer.peek().node {
			Token::ListSeparator => lexer.next(),
			_ => break,
		};
	}

	super::expect(lexer, Token::ParenthesisClose)?;
	Ok(parameters)
}
