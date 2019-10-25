use std::sync::Arc;

use crate::context::Context;
use crate::declaration::{self, FunctionPath};
use crate::error::Diagnostic;
use crate::lexer::{Lexer, Token};
use crate::node::{Function, FunctionContext, FunctionType, Parameter};
use crate::span::Spanned;

use super::ParserError;

pub fn function_type(context: &Context, function_path: &Spanned<Arc<FunctionPath>>)
                     -> Result<FunctionType, Diagnostic> {
	let FunctionPath(declaration_path) = &*function_path.node;
	declaration::load_modules(context, declaration_path.module_path.clone())
		.map_err(|error| Diagnostic::new(Spanned::new(error, function_path.span)))?;

	let declaration = context.declarations_function.get(&function_path.node).ok_or_else(||
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
	let return_type = super::pattern(lexer, &mut super::ascription)
		.map_err(|diagnostic| diagnostic.note("In parsing function return type"))?;
	let function_offset = super::expect(lexer, Token::Separator)?.byte_end;
	Ok(FunctionType::new(parameters, return_type, function_offset))
}

pub fn function(context: &Context, function_path: &Spanned<Arc<FunctionPath>>)
                -> Result<Function, Diagnostic> {
	let offset = function_type(context, function_path)?.function_byte_offset;
	let source_key = context.declarations_function.get(&function_path.node).unwrap().source;
	let source = source_key.get(context);
	let lexer = &mut Lexer::new(source.read_string().unwrap(), offset, source_key);

	let mut function_context = FunctionContext::new(function_path.node.clone());
	let expression = super::expression(&mut function_context, lexer)?;
	Ok(Function::new(function_context, expression))
}

fn parameters(lexer: &mut Lexer) -> Result<Vec<Spanned<Parameter>>, Diagnostic> {
	let mut parameters = Vec::new();
	super::expect(lexer, Token::ParenthesisOpen)?;
	while lexer.peek().node != Token::ParenthesisClose {
		let pattern = super::pattern(lexer, &mut super::binding_variable)?;
		super::expect(lexer, Token::Separator)?;
		let ascription = super::pattern(lexer, &mut super::ascription)?;

		let span = pattern.span.merge(ascription.span);
		parameters.push(Spanned::new(Parameter(pattern.node, ascription.node), span));
		match lexer.peek().node {
			Token::ListSeparator => lexer.next(),
			_ => break,
		};
	}

	super::expect(lexer, Token::ParenthesisClose)?;
	Ok(parameters)
}
