use std::convert::TryFrom;

use crate::source::Spanned;

use super::{Float, Integer, ParserError, Primitive, Token};

#[derive(Debug, Clone)]
pub struct Annotation<'a> {
	pub identifier: &'a str,
	pub arguments: Vec<Spanned<Argument<'a>>>,
}

#[derive(Debug, Clone)]
pub enum Argument<'a> {
	String(&'a str),
	Primitive(Primitive),
}

impl<'a> TryFrom<Token<'a>> for Argument<'a> {
	type Error = ParserError<'a>;

	fn try_from(value: Token<'a>) -> Result<Self, Self::Error> {
		Ok(match value {
			Token::Identifier(string) => Argument::String(string),
			other => Argument::Primitive(match other {
				Token::UnsignedInteger(integer) => Primitive::Integer(Integer::Unsigned64(integer)),
				Token::SignedInteger(integer) => Primitive::Integer(Integer::Signed64(integer)),
				Token::Float(float) => Primitive::Float(Float::Float64(float)),
				other => return Err(ParserError::InvalidArgument(other))
			})
		})
	}
}
