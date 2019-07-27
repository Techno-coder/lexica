use crate::node::{Binding, ExplicitDrop, Expression, Mutation, Statement, VariableTarget};
use crate::source::{Span, Spanned};

use super::{ParserError, ParserResult, PeekLexer, Token};

pub fn parse_statement<'a>(lexer: &mut PeekLexer<'a>, end_span: Span, terminator: Token<'a>)
                           -> ParserResult<'a, Spanned<Statement<'a>>> {
	let error = ParserError::ExpectedStatement;
	let next_token = lexer.peek().ok_or(Spanned::new(error.clone(), end_span))?;
	let span_start = next_token.span.byte_start;

	let statement = match next_token.node {
		Token::Identifier(_) => return parse_statement_identifier(lexer, end_span, terminator),
		Token::Binding => Statement::Binding(parse_binding(lexer, end_span)?),
		Token::Drop => Statement::ExplicitDrop(parse_explicit_drop(lexer, end_span)?),
		Token::Loop => {
			let conditional_loop = super::parse_conditional_loop(lexer, end_span)?;
			let conditional_loop_span = conditional_loop.span;
			let statement = Statement::ConditionalLoop(conditional_loop);
			return Ok(Spanned::new(statement, conditional_loop_span));
		}
		Token::When => {
			let when_conditional = super::parse_when_conditional(lexer, end_span)?;
			return match when_conditional.unit_evaluation() {
				false => Err(Spanned::new(error, when_conditional.span).into()),
				true => {
					let span = when_conditional.span;
					let expression = Expression::WhenConditional(when_conditional).into();
					let statement = Statement::Expression(Spanned::new(expression, span));
					Ok(Spanned::new(statement, span))
				}
			};
		}
		_ => return Err(Spanned::new(error, next_token.span).into()),
	};

	let span_end = expect_token!(lexer, end_span, terminator).byte_end;
	Ok(Spanned::new(statement, Span::new(span_start, span_end)))
}

pub fn parse_binding<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                         -> ParserResult<'a, Spanned<Binding<'a>>> {
	let span_start = expect!(lexer, end_span, Binding).byte_start;
	let variable = super::parse_variable(lexer, end_span)?;
	expect!(lexer, end_span, Assign);
	let expression = super::parse_expression_root(lexer, end_span)?;
	let span = Span::new(span_start, expression.span.byte_end);
	Ok(Spanned::new(Binding { variable, expression }, span))
}

pub fn parse_statement_identifier<'a>(lexer: &mut PeekLexer<'a>, end_span: Span, terminator: Token<'a>)
                                      -> ParserResult<'a, Spanned<Statement<'a>>> {
	let lexer_recovery = lexer.clone();
	match parse_mutation(lexer, end_span) {
		Ok(mutation) => {
			let span_end = expect_token!(lexer, end_span, terminator).byte_end;
			let span = Span::new(mutation.span.byte_start, span_end);
			let mutation = Statement::Mutation(mutation);
			Ok(Spanned::new(mutation, span))
		}
		Err(statement_error) => {
			*lexer = lexer_recovery;
			let identifier = identifier!(lexer, end_span);
			match super::parse_function_call(lexer, identifier, end_span) {
				Ok(function_call) => {
					let span_end = expect_token!(lexer, end_span, terminator).byte_end;
					let span = Span::new(function_call.span.byte_start, span_end);
					let function_call = Statement::Expression(function_call);
					Ok(Spanned::new(function_call, span))
				}
				Err(_) => Err(statement_error)
			}
		}
	}
}

pub fn parse_mutation<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                          -> ParserResult<'a, Spanned<Mutation<'a>>> {
	let target: Spanned<VariableTarget> = identifier!(lexer, end_span).into();
	let error = Spanned::new(ParserError::ExpectedMutator, end_span);
	let mutator = lexer.next().ok_or(error)?;

	let span_start = target.span.byte_start;
	Ok(match mutator.node {
		Token::Swap => {
			let other = identifier!(lexer, end_span);
			let span = Span::new(span_start, other.span.byte_end);
			Spanned::new(Mutation::Swap(target, other.into()), span)
		}
		Token::AddAssign => {
			let expression = super::parse_expression_root(lexer, end_span)?;
			let span = Span::new(span_start, expression.span.byte_end);
			Spanned::new(Mutation::AddAssign(target, expression), span)
		}
		Token::MinusAssign => {
			let expression = super::parse_expression_root(lexer, end_span)?;
			let span = Span::new(span_start, expression.span.byte_end);
			Spanned::new(Mutation::MinusAssign(target, expression), span)
		}
		Token::MultiplyAssign => {
			let expression = super::parse_expression_root(lexer, end_span)?;
			let span = Span::new(span_start, expression.span.byte_end);
			Spanned::new(Mutation::MultiplyAssign(target, expression), span)
		}
		_ => return Err(Spanned::new(ParserError::ExpectedMutator, mutator.span).into()),
	})
}

pub fn parse_explicit_drop<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                               -> ParserResult<'a, Spanned<ExplicitDrop<'a>>> {
	let span_start = expect!(lexer, end_span, Drop).byte_start;
	let target = identifier!(lexer, end_span).into();
	expect!(lexer, end_span, Assign);
	let expression = super::parse_expression_root(lexer, end_span)?;
	let span = Span::new(span_start, expression.span.byte_end);
	Ok(Spanned::new(ExplicitDrop { target, expression }, span))
}
