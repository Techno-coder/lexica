use std::collections::HashMap;
use std::sync::Arc;

use crate::context::Context;
use crate::declaration::{self, StructurePath};
use crate::error::Diagnostic;
use crate::lexer::{Lexer, Token};
use crate::node::{AscriptionPattern, Expression, ExpressionKey,
	FunctionContext, Structure, Variable};
use crate::span::{Span, Spanned};

use super::ParserError;

pub fn structure(context: &Context, structure_path: &Spanned<Arc<StructurePath>>)
                 -> Result<Structure, Diagnostic> {
	let StructurePath(declaration_path) = &*structure_path.node;
	declaration::load_modules(context, declaration_path.module_path.clone())
		.map_err(|error| Diagnostic::new(Spanned::new(error, structure_path.span)))?;

	let declaration = context.declarations_structure.get(&structure_path.node).ok_or_else(||
		Diagnostic::new(structure_path.clone().map(|path| ParserError::UndefinedStructure(path))))?;
	let source = declaration.source.get(context);
	let lexer = &mut Lexer::new(source.read_string()
		.map_err(|error| Diagnostic::new(Spanned::new(error, structure_path.span)))?,
		*declaration.line_offset, declaration.source);

	super::expect(lexer, Token::Data)?;
	super::identifier(lexer)?;
	super::expect(lexer, Token::Separator)?;

	let fields = match lexer.peek().node {
		Token::BlockOpen => fields(lexer.consume(), Some(Token::LineBreak), Token::BlockClose),
		_ => fields(lexer, None, Token::LineBreak),
	}?;

	Ok(Structure { fields })
}

pub fn literal(context: &mut FunctionContext, lexer: &mut Lexer,
               structure_path: Spanned<StructurePath>) -> Result<ExpressionKey, Diagnostic> {
	super::expect(lexer, Token::Separator)?;
	let fields = match lexer.peek().node {
		Token::BlockOpen => {
			let fields = literal_fields(context, lexer.consume(),
				Some(Token::LineBreak), Token::BlockClose);
			super::expect(lexer, Token::BlockClose)?;
			fields
		}
		_ => {
			let fields = literal_fields(context, lexer, None, Token::LineBreak);
			super::expect_peek(lexer, Token::LineBreak)?;
			fields
		}
	}?;

	let span = structure_path.span;
	let expression = Expression::Structure(structure_path, fields);
	Ok(context.register(Spanned::new(expression, span)))
}

fn fields(lexer: &mut Lexer, skip_token: Option<Token>, terminator: Token)
          -> Result<HashMap<Arc<str>, AscriptionPattern>, Diagnostic> {
	let mut fields = HashMap::new();
	while lexer.peek().node != terminator {
		field(lexer, &mut fields).map_err(|diagnostic|
			diagnostic.note("In parsing structure field"))?;
		match lexer.peek().node {
			Token::ListSeparator => lexer.next(),
			_ => break,
		};

		skip_token.as_ref().map(|token| super::skip(lexer, token));
	}

	skip_token.as_ref().map(|token| super::skip(lexer, token));
	Ok(fields)
}

fn field(lexer: &mut Lexer, fields: &mut HashMap<Arc<str>, AscriptionPattern>)
         -> Result<(), Diagnostic> {
	let identifier = super::identifier(lexer)?;
	super::expect(lexer, Token::Separator)?;
	let ascription = super::pattern(lexer, &mut super::ascription)?.node;

	match fields.contains_key(&identifier.node) {
		true => Err(Diagnostic::new(identifier.map(|field| ParserError::DuplicateField(field)))),
		false => Ok(fields.insert(identifier.node, ascription).unwrap_none()),
	}
}

fn literal_fields(context: &mut FunctionContext, lexer: &mut Lexer, skip_token: Option<Token>,
                  terminator: Token) -> Result<HashMap<Arc<str>, (Span, ExpressionKey)>, Diagnostic> {
	let mut fields = HashMap::new();
	while lexer.peek().node != terminator {
		let expression = literal_field(context, lexer, &mut fields)
			.map_err(|diagnostic| diagnostic.note("In parsing structure field"))?;
		match &context[&expression].node {
			Expression::Structure(_, _) => (),
			_ => match lexer.peek().node {
				Token::ListSeparator => { lexer.next(); }
				_ => break,
			},
		}

		skip_token.as_ref().map(|token| super::skip(lexer, token));
	}

	skip_token.as_ref().map(|token| super::skip(lexer, token));
	Ok(fields)
}

fn literal_field(context: &mut FunctionContext, lexer: &mut Lexer,
                 fields: &mut HashMap<Arc<str>, (Span, ExpressionKey)>)
                 -> Result<ExpressionKey, Diagnostic> {
	let identifier = super::identifier(lexer)?;
	let expression = match lexer.peek().node {
		Token::Separator => super::root_value(context, lexer.consume())?,
		_ => {
			let variable = Variable::new(identifier.node.clone());
			context.register(Spanned::new(Expression::Variable(variable), identifier.span))
		}
	};

	match fields.contains_key(&identifier.node) {
		true => Err(Diagnostic::new(identifier.map(|field| ParserError::DuplicateField(field)))),
		false => {
			fields.insert(identifier.node, (identifier.span, expression)).unwrap_none();
			Ok(expression)
		}
	}
}
