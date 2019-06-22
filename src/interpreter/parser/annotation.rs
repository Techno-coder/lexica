use std::convert::TryFrom;

use crate::source::Spanned;

use super::{Float, Integer, ParserError, Primitive, Token};

/// An annotation instance parsed from byte code.
#[derive(Debug, Clone)]
pub struct Annotation<'a> {
	/// The identifier of the annotation type.
	pub identifier: &'a str,
	/// The arguments to be provided to the annotation type.
	pub arguments: Vec<Spanned<Argument<'a>>>,
}

/// An annotation argument parsed from byte code.
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
				Token::Boolean(boolean) => Primitive::Boolean(boolean),
				other => return Err(ParserError::InvalidArgument(other))
			})
		})
	}
}
