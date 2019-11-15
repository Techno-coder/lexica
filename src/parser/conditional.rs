use crate::error::Diagnostic;
use crate::lexer::{Lexer, Token};
use crate::node::{ConditionEnd, ConditionStart, Expression, ExpressionKey, FunctionContext};
use crate::span::Spanned;

pub fn termination_loop(context: &mut FunctionContext, lexer: &mut Lexer) -> Result<ExpressionKey, Diagnostic> {
	let initial_span = super::expect(lexer, Token::Loop)?;
	let condition_start = match lexer.peek().node {
		Token::Implies => None,
		_ => Some(super::root_value(context, lexer).map_err(|diagnostic|
			diagnostic.note("In parsing loop start condition"))?),
	};

	super::expect(lexer, Token::Implies)?;
	let condition_end = super::root_value(context, lexer)
		.map_err(|diagnostic| diagnostic.note("In parsing loop end condition"))?;
	let span = initial_span.merge(super::expect(lexer, Token::Separator)?);
	let expression = super::root_value(context, lexer)?;

	let expression = Expression::TerminationLoop(condition_start, condition_end, expression);
	Ok(context.register(Spanned::new(expression, span)))
}

pub fn conditional(context: &mut FunctionContext, lexer: &mut Lexer) -> Result<ExpressionKey, Diagnostic> {
	let initial_span = super::expect(lexer, Token::If)?;
	let (branches, span) = match lexer.peek().node {
		Token::Separator => {
			let mut branches = Vec::new();
			super::expect(lexer.consume(), Token::BlockOpen)?;
			while lexer.peek().node != Token::BlockClose {
				let (condition_start, condition_end) = branch(context, lexer)?;
				branches.push((condition_start, condition_end,
					super::expression(context, lexer)?));
				super::skip(lexer, &Token::LineBreak);
			}
			(branches, super::expect(lexer, Token::BlockClose)?)
		}
		_ => {
			let (condition_start, condition_end) = branch(context, lexer)?;
			let expression = super::expression(context, lexer)?;
			(vec![(condition_start, condition_end, expression)],
				context[&expression].span)
		}
	};

	let conditional = Expression::Conditional(branches);
	Ok(context.register(Spanned::new(conditional, initial_span.merge(span))))
}

pub fn branch(context: &mut FunctionContext, lexer: &mut Lexer)
              -> Result<(ConditionStart, Option<ConditionEnd>), Diagnostic> {
	let condition_start = super::root_value(context, lexer).map_err(|diagnostic|
		diagnostic.note("In parsing conditional branch condition"))?;

	let mut condition_end = None;
	if lexer.peek().node == Token::Implies {
		condition_end = Some(super::root_value(context, lexer.consume()).map_err(|diagnostic|
			diagnostic.note("In parsing conditional branch end condition"))?);
	}

	super::expect(lexer, Token::Separator).map_err(|diagnostic|
		diagnostic.note("In parsing a conditional branch condition"))?;
	Ok((condition_start, condition_end))
}
