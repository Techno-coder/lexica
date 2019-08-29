use crate::node::{Block, ConditionalLoop, Expression, ExpressionBlock, ExpressionNode, WhenBranch,
                  WhenConditional};
use crate::source::{Span, Spanned};

use super::{ParserError, ParserResult, PeekLexer, Token};

pub type WhenCondition<'a> = (Spanned<ExpressionNode<'a>>, Option<Spanned<ExpressionNode<'a>>>);

pub fn parse_conditional_loop<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                                  -> ParserResult<'a, Spanned<ConditionalLoop<'a>>> {
	let span_start = expect!(lexer, end_span, Loop).byte_start;
	let mut end_condition = super::parse_expression_root(lexer, end_span)?;
	let mut start_condition = None;

	let error = ParserError::ExpectedToken(Token::BlockOpen);
	let next_token = lexer.peek().ok_or(Spanned::new(error.clone(), end_span))?;
	match next_token.node {
		Token::Implies => {
			lexer.next();
			start_condition = Some(end_condition);
			end_condition = super::parse_expression_root(lexer, end_span)?;
		}
		Token::BlockOpen => (),
		_ => return Err(Spanned::new(error, next_token.span).into()),
	}

	let block = super::parse_block(lexer, end_span)?;
	let span = Span::new(span_start, block.span.byte_end);
	let conditional_loop = ConditionalLoop { start_condition, end_condition, block };
	Ok(Spanned::new(conditional_loop, span))
}

pub fn parse_when_conditional<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                                  -> ParserResult<'a, Spanned<WhenConditional<'a>>> {
	let span_start = expect!(lexer, end_span, When).byte_start;
	match lexer.peek() {
		Some(token) if token.node == Token::BlockOpen => {
			let mut branches = Vec::new();
			lexer.next();

			while let Some(token) = lexer.peek() {
				if token.node == Token::BlockClose {
					break;
				}

				let (condition, end_condition) = parse_when_condition(lexer, end_span)?;
				expect!(lexer, end_span, BlockSeparator);

				let expression_block = parse_branch_block(lexer, end_span)?;
				branches.push(WhenBranch { condition, end_condition, expression_block });
			}

			let span_end = expect!(lexer, end_span, BlockClose).byte_end;
			let span = Span::new(span_start, span_end);
			Ok(Spanned::new(WhenConditional { branches }, span))
		}
		_ => {
			let (condition, end_condition) = parse_when_condition(lexer, end_span)?;
			let expression_block = super::parse_expression_block(lexer, end_span)?;
			let span = Span::new(span_start, expression_block.span.byte_end);

			let branches = vec![WhenBranch { condition, end_condition, expression_block }];
			Ok(Spanned::new(WhenConditional { branches }, span))
		}
	}
}

pub fn parse_when_condition<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                                -> ParserResult<'a, WhenCondition<'a>> {
	let condition = super::parse_expression_root(lexer, end_span)?;
	Ok(match lexer.peek() {
		Some(token) if token.node == Token::Implies => {
			lexer.next();
			let end_condition = super::parse_expression_root(lexer, end_span)?;
			(condition, Some(end_condition))
		}
		_ => (condition, None),
	})
}

pub fn parse_branch_block<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                              -> ParserResult<'a, Spanned<ExpressionBlock<'a>>> {
	Ok(match lexer.peek() {
		Some(token) if token.node == Token::BlockOpen => {
			let expression_block = super::parse_expression_block(lexer, end_span)?;
			expect!(lexer, end_span, ListSeparator);
			expression_block
		}
		_ => {
			let lexer_recovery = lexer.clone();
			match super::parse_statement(lexer, end_span, Token::ListSeparator) {
				Ok(statement) => {
					let span = statement.span;
					let block = Spanned::new(Block { statements: vec![statement] }, span);
					let expression = Spanned::new(Expression::Unit.into(), span);
					Spanned::new(ExpressionBlock { block, expression }, span)
				}
				Err(statement_error) => {
					*lexer = lexer_recovery;
					match super::parse_expression_root(lexer, end_span) {
						Err(_) => return Err(statement_error),
						Ok(expression) => {
							let span = expression.span;
							expect!(lexer, end_span, ListSeparator);
							let block = Spanned::new(Block { statements: Vec::new() }, span);
							Spanned::new(ExpressionBlock { block, expression }, span)
						}
					}
				}
			}
		}
	})
}
