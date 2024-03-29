use std::collections::HashMap;
use std::sync::Arc;

use crate::context::Context;
use crate::declaration::{Declaration, ModulePath, StructurePath};
use crate::error::Diagnostic;
use crate::lexer::{Lexer, Token};
use crate::node::{AscriptionPattern, Definition, Expression, ExpressionKey,
	FunctionContext, Structure, Variable};
use crate::span::{Span, Spanned};

use super::ParserError;

pub fn structure(context: &Context, structure_path: &Spanned<Arc<StructurePath>>)
                 -> Result<Structure, Diagnostic> {
	let declaration = context.declarations_structure.get(&structure_path.node).ok_or_else(||
		Diagnostic::new(structure_path.clone().map(|path| ParserError::UndefinedStructure(path))))?;
	let source = declaration.source.get(context);

	let lexer = &mut Lexer::declaration(&source, &declaration)?;
	super::expect(lexer, Token::Data)?;
	super::identifier(lexer)?;

	let templates = templates(lexer)?;
	let fields = match lexer.peek().node {
		Token::BlockOpen => {
			let fields = fields(lexer.consume(), Some(Token::LineBreak), Token::BlockClose)?;
			super::expect(lexer, Token::BlockClose)?;
			fields
		}
		_ => fields(lexer, None, Token::LineBreak)?,
	};

	Ok(Structure { templates, fields })
}

pub fn definition(context: &Context, declaration: Arc<ModulePath>,
                  definition: &Declaration) -> Result<Definition, Diagnostic> {
	let source = definition.source.get(context);
	let lexer = &mut Lexer::declaration(&source, definition)?;

	super::expect(lexer, Token::Define)?;
	let structure = super::expression::path(lexer)?
		.map(|path| StructurePath(path));
	let templates = templates(lexer)?;
	Ok(Definition { declaration, structure, templates })
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
		_ => literal_fields(context, lexer, None, Token::LineBreak),
	}?;

	let span = structure_path.span;
	let expression = Expression::Structure(structure_path, fields);
	Ok(context.register(Spanned::new(expression, span)))
}

pub fn templates(lexer: &mut Lexer) -> Result<Vec<Spanned<Arc<str>>>, Diagnostic> {
	let mut templates = Vec::new();
	let token = lexer.next();
	match token.node {
		Token::Separator => (),
		Token::AngleLeft => {
			super::list(lexer, Token::AngleRight, Token::ListSeparator, &mut |lexer| {
				super::expect(lexer, Token::Template)?;
				templates.push(super::identifier(lexer)?);
				Ok(())
			})?;
			lexer.next();
		}
		other => {
			let error = ParserError::ExpectedStructureTerminator(other);
			return Err(Diagnostic::new(Spanned::new(error, token.span)));
		}
	}
	Ok(templates)
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
