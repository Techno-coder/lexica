use hashbrown::HashMap;

use crate::node::{DataType, Field, Structure};
use crate::source::{Span, Spanned};

use super::{ParserResult, PeekLexer, Token};

pub fn parse_structure<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                           -> ParserResult<'a, Spanned<Structure<'a>>> {
	let byte_start = expect!(lexer, end_span, Data).byte_start;
	let identifier = identifier!(lexer, end_span);
	expect!(lexer, end_span, BlockOpen);

	let mut fields = HashMap::new();
	while let Some(token) = lexer.peek() {
		match token.node {
			Token::BlockClose => break,
			_ => {
				let field = parse_field(lexer, end_span)?;
				fields.insert(field.identifier.node.clone(), field);
				match lexer.peek() {
					Some(token) if token.node == Token::ListSeparator => lexer.next(),
					_ => break,
				};
			}
		}
	}

	let byte_end = expect!(lexer, end_span, BlockClose).byte_end;
	Ok(Spanned::new(Structure { identifier, fields }, Span::new(byte_start, byte_end)))
}

pub fn parse_field<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                       -> ParserResult<'a, Spanned<Field<'a>>> {
	let identifier = identifier!(lexer, end_span);
	expect!(lexer, end_span, VariableSeparator);
	let data_type = identifier!(lexer, end_span);

	let data_type = Spanned::new(DataType::new(data_type.node), data_type.span);
	let span = Span::new(identifier.span.byte_start, data_type.span.byte_end);
	Ok(Spanned::new(Field { identifier, data_type }, span))
}
