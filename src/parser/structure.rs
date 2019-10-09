use std::collections::HashMap;
use std::sync::Arc;

use crate::context::Context;
use crate::declaration::{self, StructurePath};
use crate::error::Diagnostic;
use crate::lexer::{Lexer, Token};
use crate::node::{AscriptionPattern, Structure};
use crate::span::Spanned;

use super::ParserError;

pub fn structure(context: &Context, structure_path: &Spanned<Arc<StructurePath>>)
                 -> Result<Arc<Structure>, Diagnostic> {
	let StructurePath(declaration_path) = &*structure_path.node;
	declaration::load_modules(context, declaration_path.module_path.clone())
		.map_err(|error| Diagnostic::new(Spanned::new(error, structure_path.span)))?;

	let declarations_structure = context.declarations_structure.read();
	let declaration = declarations_structure.get(&structure_path.node).ok_or_else(||
		Diagnostic::new(structure_path.clone().map(|path| ParserError::UndefinedStructure(path))))?;
	let source = declaration.source.get(context);
	let lexer = &mut Lexer::new(source.read_string()
		.map_err(|error| Diagnostic::new(Spanned::new(error, structure_path.span)))?,
		*declaration.line_offset, declaration.source);

	super::expect(lexer, Token::Data)?;
	super::identifier(lexer)?;
	super::expect(lexer, Token::Separator)?;

	let fields = match lexer.peek().node {
		Token::BlockOpen => {
			lexer.next();
			fields(lexer, Some(Token::LineBreak), Token::BlockClose)
		}
		_ => fields(lexer, None, Token::LineBreak),
	}?;

	let structure = Arc::new(Structure::new(fields));
	context.node_structures.write().insert(structure_path.node.clone(), structure.clone());
	Ok(structure)
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

		if let Some(token) = &skip_token {
			super::skip(lexer, token.clone());
		}
	}

	super::expect(lexer, terminator)?;
	Ok(fields)
}

fn field(lexer: &mut Lexer, fields: &mut HashMap<Arc<str>, AscriptionPattern>) -> Result<(), Diagnostic> {
	let identifier = super::identifier(lexer)?;
	super::expect(lexer, Token::Separator)?;
	let ascription = super::pattern(lexer, &mut super::ascription)?.node;

	match fields.contains_key(&identifier.node) {
		true => Err(Diagnostic::new(identifier.map(|field|
			ParserError::DuplicateField(field)))),
		false => {
			fields.insert(identifier.node, ascription);
			Ok(())
		}
	}
}
