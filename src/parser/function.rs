use std::sync::Arc;

use crate::context::Context;
use crate::declaration::{self, FunctionPath};
use crate::error::Diagnostic;
use crate::lexer::{Lexer, Token};
use crate::node::{AscriptionPattern, BindingPattern, Function, FunctionContext};
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
	super::identifier(lexer)?;

	let parameters = parameters(lexer).map_err(|diagnostic|
		diagnostic.note("In parsing function parameters"))?;
	super::expect(lexer, Token::ReturnSeparator)?;
	let return_type = super::ascription(lexer).map_err(|diagnostic|
		diagnostic.note("In parsing function return type"))?;
	super::expect(lexer, Token::Separator)?;

	let mut function_context = FunctionContext::new(function_path.node.clone());
	let expression = super::expression(&mut function_context, lexer)?;
	let function = Function::new(function_context, parameters, return_type, expression);
	context.node_functions.write().insert(function_path.node, function);
	Ok(())
}

fn parameters(lexer: &mut Lexer) -> Result<Vec<(BindingPattern, AscriptionPattern)>, Diagnostic> {
	let mut parameters = Vec::new();
	super::expect(lexer, Token::ParenthesisOpen)?;
	while lexer.peek().node != Token::ParenthesisClose {
		let pattern = super::pattern(lexer, &mut super::binding_variable)?;
		super::expect(lexer, Token::Separator)?;
		let ascription = super::pattern(lexer, &mut super::ascription)?;
		parameters.push((pattern.node, ascription.node));

		match lexer.peek().node {
			Token::ListSeparator => lexer.next(),
			_ => break,
		};
	}

	super::expect(lexer, Token::ParenthesisClose)?;
	Ok(parameters)
}
