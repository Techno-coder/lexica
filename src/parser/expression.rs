use crate::declaration::{DeclarationPath, ModulePath, StructurePath};
use crate::error::Diagnostic;
use crate::lexer::{Lexer, Token};
use crate::node::{Arithmetic, Ascription, BindingVariable, Expression, ExpressionKey,
	FunctionContext, Mutability, MutationKind, Variable};
use crate::parser::ParserError;
use crate::span::{Span, Spanned};

use super::parser::expect;

pub fn expression(context: &mut FunctionContext, lexer: &mut Lexer) -> Result<ExpressionKey, Diagnostic> {
	let token = lexer.peek();
	match &token.node {
		Token::Let => binding(context, lexer).map_err(|diagnostic|
			diagnostic.note("In parsing a binding")),
		Token::Loop => termination_loop(context, lexer).map_err(|diagnostic|
			diagnostic.note("In parsing a termination loop")),
		Token::Drop => explicit_drop(context, lexer).map_err(|diagnostic|
			diagnostic.note("In parsing an explicit drop")),
		_ => expression_terminator(context, lexer),
	}
}

fn binding(context: &mut FunctionContext, lexer: &mut Lexer) -> Result<ExpressionKey, Diagnostic> {
	let initial_span = super::expect(lexer, Token::Let)?;
	let pattern = super::pattern(lexer, &mut binding_variable)?;
	let mut binding_ascription = None;
	if lexer.peek().node == Token::Separator {
		lexer.next();
		let pattern = super::pattern(lexer, &mut ascription)?;
		binding_ascription = Some(pattern.node);
	}

	super::expect(lexer, Token::Assign)?;
	let value = super::root_value(context, lexer)?;
	let span = initial_span.merge(expect_terminator(lexer, &context[&value])?);
	let expression = Expression::Binding(pattern.node, binding_ascription, value);
	Ok(context.register(Spanned::new(expression, span)))
}

fn termination_loop(context: &mut FunctionContext, lexer: &mut Lexer) -> Result<ExpressionKey, Diagnostic> {
	let initial_span = super::expect(lexer, Token::Loop)?;
	let condition_start = match lexer.peek().node {
		Token::Implies => None,
		_ => Some(super::root_value(context, lexer).map_err(|diagnostic|
			diagnostic.note("In parsing loop start condition"))?),
	};

	expect(lexer, Token::Implies)?;
	let condition_end = super::root_value(context, lexer).map_err(|diagnostic|
		diagnostic.note("In parsing loop end condition"))?;
	let span = initial_span.merge(expect(lexer, Token::Separator)?);
	let expression = super::root_value(context, lexer)?;

	let expression = Expression::TerminationLoop(condition_start, condition_end, expression);
	Ok(context.register(Spanned::new(expression, span)))
}

fn explicit_drop(context: &mut FunctionContext, lexer: &mut Lexer) -> Result<ExpressionKey, Diagnostic> {
	let initial_span = super::expect(lexer, Token::Drop)?;
	let pattern = super::pattern(lexer, &mut variable)?;

	super::expect(lexer, Token::Assign)?;
	let value = super::root_value(context, lexer)?;
	let span = initial_span.merge(expect_terminator(lexer, &context[&value])?);
	let expression = Expression::ExplicitDrop(pattern.node, value);
	Ok(context.register(Spanned::new(expression, span)))
}

fn expression_terminator(context: &mut FunctionContext, lexer: &mut Lexer) -> Result<ExpressionKey, Diagnostic> {
	let value = super::root_value(context, lexer)?;
	if let Expression::Block(_) = &context[&value].node {
		return Ok(value);
	}

	let token = lexer.next();
	let mutation_kind = Spanned::new(match token.node {
		Token::Swap => MutationKind::Swap,
		Token::AddAssign => MutationKind::Arithmetic(Arithmetic::Add),
		Token::MinusAssign => MutationKind::Arithmetic(Arithmetic::Minus),
		Token::MultiplyAssign => MutationKind::Arithmetic(Arithmetic::Multiply),
		Token::LineBreak => return Ok(value),
		_ => return Err(Diagnostic::new(token.map(|token|
			ParserError::ExpectedExpressionTerminator(token)))),
	}, token.span);

	let expression = expression(context, lexer)?;
	let span = context[&value].span.merge(context[&expression].span);
	let expression = Expression::Mutation(mutation_kind, value, expression);
	Ok(context.register(Spanned::new(expression, span)))
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
		.map(|identifier| Ascription(StructurePath(DeclarationPath {
			module_path: ModulePath::intrinsic(),
			identifier,
		}))))
}

fn expect_terminator(lexer: &mut Lexer, expression: &Spanned<Expression>) -> Result<Span, Diagnostic> {
	match expression.node {
		Expression::Block(_) => Ok(expression.span),
		_ => super::expect(lexer, Token::LineBreak),
	}
}
