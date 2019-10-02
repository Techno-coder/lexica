use crate::error::Diagnostic;
use crate::lexer::{Lexer, Token};
use crate::node::{Ascription, BindingVariable, Expression, ExpressionKey, FunctionContext,
	Mutability, Variable};
use crate::span::Spanned;

pub fn expression(context: &mut FunctionContext, lexer: &mut Lexer) -> Result<ExpressionKey, Diagnostic> {
	let token = lexer.peek();
	match &token.node {
		Token::Let => binding(context, lexer).map_err(|diagnostic|
			diagnostic.note("In parsing a binding")),
		_ => {
			let value = super::root_value(context, lexer)?;
			match &context[&value].node {
				Expression::Block(_) => return Ok(value),
				_ => super::expect(lexer, Token::LineBreak)?,
			};
			Ok(value)
		}
	}
}

fn binding(context: &mut FunctionContext, lexer: &mut Lexer) -> Result<ExpressionKey, Diagnostic> {
	let initial_span = super::expect(lexer, Token::Let)?;
	let pattern = super::pattern(lexer, &mut binding_variable)?;
	let mut binding_ascription = None;
	if lexer.peek().node == Token::Separator {
		lexer.next();
		binding_ascription = Some(ascription(lexer)?);
	}

	super::expect(lexer, Token::Equals)?;
	let value = super::root_value(context, lexer)?;
	let span = initial_span.merge(super::expect(lexer, Token::LineBreak)?);
	let binding = Expression::Binding(pattern.node, binding_ascription, value);
	Ok(context.register(Spanned::new(binding, span)))
}

fn variable(lexer: &mut Lexer) -> Result<Spanned<Variable>, Diagnostic> {
	Ok(super::identifier(lexer)
		.map_err(|diagnostic| diagnostic.note("In parsing a variable"))?
		.map(|identifier| Variable::new(identifier)))
}

pub fn binding_variable(lexer: &mut Lexer) -> Result<Spanned<BindingVariable>, Diagnostic> {
	let mut mutability = Mutability::Immutable;
	if lexer.peek().node == Token::Mutable {
		mutability = Mutability::Mutable;
		lexer.next();
	}

	let variable = variable(lexer).map_err(|diagnostic|
		diagnostic.note("In parsing a binding pattern"))?;
	Ok(variable.map(|variable| BindingVariable(variable, mutability)))
}

pub fn ascription(lexer: &mut Lexer) -> Result<Spanned<Ascription>, Diagnostic> {
	Ok(super::identifier(lexer)
		.map_err(|diagnostic| diagnostic.note("In parsing an ascription"))?
		.map(|identifier| Ascription(identifier)))
}
