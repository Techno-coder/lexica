use crate::node::{Binding, ConditionalLoop, ExplicitDrop, Mutation, Statement};
use crate::source::{Span, Spanned};

use super::{ParserError, ParserResult, PeekLexer, Token};

pub fn parse_statement<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                           -> ParserResult<'a, Spanned<Statement<'a>>> {
	let error = ParserError::ExpectedStatement;
	let next_token = lexer.peek().ok_or(Spanned::new(error.clone(), end_span))?;
	let span_start = next_token.span.byte_start;

	let statement = match next_token.node {
		Token::Binding => Statement::Binding(parse_binding(lexer, end_span)?),
		Token::Identifier(_) => Statement::Mutation(parse_mutation(lexer, end_span)?),
		Token::Drop => Statement::ExplicitDrop(parse_explicit_drop(lexer, end_span)?),
		Token::Loop => {
			let conditional_loop = parse_conditional_loop(lexer, end_span)?;
			let conditional_loop_span = conditional_loop.span;
			let statement = Statement::ConditionalLoop(conditional_loop);
			return Ok(Spanned::new(statement, conditional_loop_span));
		}
		_ => return Err(Spanned::new(error, next_token.span).into()),
	};

	let span_end = expect!(lexer, end_span, Terminator).byte_end;
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

pub fn parse_mutation<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                          -> ParserResult<'a, Spanned<Mutation<'a>>> {
	let identifier = identifier!(lexer, end_span);
	let error = Spanned::new(ParserError::ExpectedMutator, end_span);
	let mutator = lexer.next().ok_or(error)?;

	let span_start = identifier.span.byte_start;
	Ok(match mutator.node {
		Token::Swap => {
			let other = identifier!(lexer, end_span);
			let span = Span::new(span_start, other.span.byte_end);
			Spanned::new(Mutation::Swap(identifier, other), span)
		}
		Token::AddAssign => {
			let expression = super::parse_expression_root(lexer, end_span)?;
			let span = Span::new(span_start, expression.span.byte_end);
			Spanned::new(Mutation::AddAssign(identifier, expression), span)
		}
		Token::MultiplyAssign => {
			let expression = super::parse_expression_root(lexer, end_span)?;
			let span = Span::new(span_start, expression.span.byte_end);
			Spanned::new(Mutation::MultiplyAssign(identifier, expression), span)
		}
		_ => return Err(Spanned::new(ParserError::ExpectedMutator, mutator.span).into()),
	})
}

pub fn parse_explicit_drop<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                               -> ParserResult<'a, Spanned<ExplicitDrop<'a>>> {
	let span_start = expect!(lexer, end_span, Drop).byte_start;
	let identifier = identifier!(lexer, end_span);
	expect!(lexer, end_span, Assign);
	let expression = super::parse_expression_root(lexer, end_span)?;
	let span = Span::new(span_start, expression.span.byte_end);
	Ok(Spanned::new(ExplicitDrop { identifier, expression }, span))
}

pub fn parse_conditional_loop<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                                  -> ParserResult<'a, Spanned<ConditionalLoop<'a>>> {
	let span_start = expect!(lexer, end_span, Loop).byte_start;
	let mut end_condition = super::parse_expression_root(lexer, end_span)?;
	let mut start_condition = None;

	let error = ParserError::ExpectedToken(Token::BlockOpen);
	let next_token = lexer.next().ok_or(Spanned::new(error.clone(), end_span))?;
	match next_token.node {
		Token::Implies => {
			start_condition = Some(end_condition);
			end_condition = super::parse_expression_root(lexer, end_span)?;
		}
		Token::BlockOpen => (),
		_ => return Err(Spanned::new(error, next_token.span).into()),
	}

	let spanned_statements = super::parse_block(lexer, end_span)?;
	let (statements, span_end) = (spanned_statements.node, spanned_statements.span.byte_end);
	let conditional_loop = ConditionalLoop { start_condition, end_condition, statements };
	Ok(Spanned::new(conditional_loop, Span::new(span_start, span_end)))
}