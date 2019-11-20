use std::sync::Arc;

use crate::context::Context;
use crate::declaration::FunctionPath;
use crate::error::Diagnostic;
use crate::lexer::{Lexer, Token};
use crate::node::*;
use crate::span::Spanned;

use super::ParserError;

pub fn function_type(context: &Context, function_path: &Spanned<Arc<FunctionPath>>)
                     -> Result<FunctionType, Diagnostic> {
	let declaration = context.declarations_function.get(&function_path.node).ok_or_else(||
		Diagnostic::new(function_path.clone().map(|path| ParserError::UndefinedFunction(path))))?;
	let source = declaration.source.get(context);

	let lexer = &mut Lexer::declaration(&source, &declaration)?;
	super::expect(lexer, Token::Function)?;
	super::identifier(lexer)?;

	let definition = context.node_definitions.get(&function_path.node);
	let definition = definition.as_ref().map(|definition| definition.as_ref());
	let parameters = parameters(lexer, definition).map_err(|diagnostic|
		diagnostic.note("In parsing function parameters"))?;
	let return_type = return_type(lexer)?;

	let function_offset = super::expect(lexer, Token::Separator)?.byte_end;
	Ok(FunctionType::new(parameters, return_type, function_offset))
}

pub fn function(context: &Context, function_path: &Spanned<Arc<FunctionPath>>)
                -> Result<NodeFunction, Diagnostic> {
	let function_type = crate::node::function_type(context, function_path)?;
	let source_key = context.declarations_function.get(&function_path.node).unwrap().source;
	let source = source_key.get(context);

	let offset = function_type.function_byte_offset;
	let lexer = &mut Lexer::new(source.read_string().unwrap(), offset, source_key);

	let mut function_context = FunctionContext::new(function_path.node.clone());
	let expression = super::expression(&mut function_context, lexer)?;
	Ok(NodeFunction::new(function_context, expression, function_type))
}

fn parameters(lexer: &mut Lexer, definition: Option<&Definition>)
              -> Result<Vec<Spanned<Parameter>>, Diagnostic> {
	let mut parameters = Vec::new();
	super::expect(lexer, Token::ParenthesisOpen)?;
	super::list(lexer, Token::ParenthesisClose, Token::ListSeparator, &mut |lexer| {
		match lexer.peek().node {
			Token::Reference | Token::Unique => self_variable(definition, &mut parameters, lexer),
			Token::SelfVariable | Token::Mutable => self_variable(definition, &mut parameters, lexer),
			_ => {
				let pattern = super::pattern(lexer, &mut super::binding_variable)?;
				super::expect(lexer, Token::Separator)?;

				let ascription = super::pattern(lexer, &mut super::ascription)?;
				Ok(parameters.push(Spanned::new(Parameter(pattern.node, ascription.node),
					pattern.span.merge(ascription.span))))
			}
		}
	})?;

	super::expect(lexer, Token::ParenthesisClose)?;
	Ok(parameters)
}

fn self_variable(definition: Option<&Definition>, parameters: &mut Vec<Spanned<Parameter>>,
                 lexer: &mut Lexer) -> Result<(), Diagnostic> {
	use super::expression;
	let (permission, lifetime, mutability) = match lexer.peek().node {
		Token::Reference => (Some(Permission::Shared),
			expression::lifetime(lexer.consume())?, Mutability::Immutable),
		Token::Unique => (Some(Permission::Unique),
			expression::lifetime(lexer.consume())?, Mutability::Immutable),
		Token::SelfVariable => (None, None, Mutability::Immutable),
		Token::Mutable => {
			lexer.next();
			(None, None, Mutability::Mutable)
		}
		_ => unreachable!(),
	};

	let span = super::expect(lexer, Token::SelfVariable)?;
	let definition = definition.ok_or({
		let error = ParserError::FunctionSelfVariable;
		Diagnostic::new(Spanned::new(error, span))
	})?;

	if !parameters.is_empty() {
		let error = ParserError::SelfVariablePosition;
		return Err(Diagnostic::new(Spanned::new(error, span)));
	}

	let templates = definition.templates.iter().cloned().map(|template|
		Pattern::Terminal(Spanned::new(Ascription::Template(template.node), span))).collect();
	let ascription = Ascription::Structure(definition.structure.node.clone(), templates);
	let mut ascription = Pattern::Terminal(Spanned::new(ascription, span));
	if let Some(permission) = permission {
		let reference = Ascription::Reference(permission, lifetime, Box::new(ascription));
		ascription = Pattern::Terminal(Spanned::new(reference, span));
	}

	let variable = BindingVariable(Variable::new("self".into()), mutability);
	let parameter = Parameter(Pattern::Terminal(Spanned::new(variable, span)), ascription);
	Ok(parameters.push(Spanned::new(parameter, span)))
}

fn return_type(lexer: &mut Lexer) -> Result<Spanned<AscriptionPattern>, Diagnostic> {
	let token = lexer.peek();
	match token.node {
		Token::ReturnSeparator => {
			super::pattern(lexer.consume(), &mut super::ascription).map_err(|diagnostic|
				diagnostic.note("In parsing function return type"))
		}
		_ => {
			let intrinsic = crate::intrinsic::Intrinsic::Unit.structure();
			let ascription = Ascription::Structure(intrinsic, Vec::new());
			Ok(Spanned::new(Pattern::Terminal(Spanned::new(ascription, token.span)), token.span))
		}
	}
}
