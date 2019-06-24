use crate::node::{Binding, ConditionalLoop, ExplicitDrop, Mutation, Statement};
use crate::source::{Span, Spanned};

use super::{ParserError, ParserResult, PeekLexer, Token};

pub fn parse_statement<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                           -> ParserResult<'a, Statement<'a>> {
	let error = ParserError::ExpectedStatement;
	let next_token = lexer.peek().ok_or(Spanned::new(error.clone(), end_span.clone()))?;

	let span_clone = end_span.clone();
	let statement = match next_token.node {
		Token::Binding => Statement::Binding(parse_binding(lexer, span_clone)?),
		Token::Identifier(_) => Statement::Mutation(parse_mutation(lexer, span_clone)?),
		Token::Drop => Statement::ExplicitDrop(parse_explicit_drop(lexer, span_clone)?),
		Token::While => {
			let conditional_loop = parse_conditional_loop(lexer, span_clone)?;
			return Ok(Statement::ConditionalLoop(conditional_loop));
		}
		_ => return Err(Spanned::new(error, next_token.span.clone())),
	};

	expect!(lexer, end_span, Terminator);
	Ok(statement)
}

pub fn parse_binding<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                         -> ParserResult<'a, Binding<'a>> {
	expect!(lexer, end_span, Binding);
	let variable = super::parse_variable(lexer, end_span.clone())?;
	expect!(lexer, end_span, Assign);
	let expression = super::parse_expression_root(lexer, end_span)?;
	Ok(Binding { variable, expression })
}

pub fn parse_mutation<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                          -> ParserResult<'a, Mutation<'a>> {
	let identifier = identifier!(lexer, end_span).node;
	let error = Spanned::new(ParserError::ExpectedMutator, end_span.clone());
	let mutator = lexer.next().ok_or(error)?;
	Ok(match mutator.node {
		Token::Swap => {
			let other = identifier!(lexer, end_span).node;
			Mutation::Swap(identifier, other)
		}
		Token::AddAssign => {
			let expression = super::parse_expression_root(lexer, end_span)?;
			Mutation::AddAssign(identifier, expression)
		}
		_ => return Err(Spanned::new(ParserError::ExpectedMutator, mutator.span)),
	})
}

pub fn parse_explicit_drop<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                               -> ParserResult<'a, ExplicitDrop<'a>> {
	expect!(lexer, end_span, Drop);
	let identifier = identifier!(lexer, end_span).node;
	expect!(lexer, end_span, Assign);
	let expression = super::parse_expression_root(lexer, end_span)?;
	Ok(ExplicitDrop { identifier, expression })
}

pub fn parse_conditional_loop<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                                  -> ParserResult<'a, ConditionalLoop<'a>> {
	expect!(lexer, end_span, While);
	let mut end_condition = super::parse_expression_root(lexer, end_span.clone())?;
	let mut start_condition = None;

	let error = ParserError::ExpectedToken(Token::BlockOpen);
	let next_token = lexer.next().ok_or(Spanned::new(error.clone(), end_span.clone()))?;
	match next_token.node {
		Token::Implies => {
			start_condition = Some(end_condition);
			end_condition = super::parse_expression_root(lexer, end_span.clone())?;
		}
		Token::BlockOpen => (),
		_ => return Err(Spanned::new(error, next_token.span)),
	}

	let mut statements = Vec::new();
	expect!(lexer, end_span, BlockOpen);
	let error = Spanned::new(ParserError::ExpectedToken(Token::BlockClose), end_span.clone());
	while lexer.peek().ok_or(error.clone())?.node != Token::BlockClose {
		statements.push(parse_statement(lexer, end_span.clone())?);
	}

	expect!(lexer, end_span, BlockClose);
	Ok(ConditionalLoop { start_condition, end_condition, statements })
}
