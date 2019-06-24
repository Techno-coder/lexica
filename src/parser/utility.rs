macro_rules! expect {
    ($lexer: expr, $end_span: expr, $token: ident) => {{
		let error = crate::parser::ParserError::ExpectedToken(crate::parser::Token::$token);
		match $lexer.next() {
			Some(token) => match token.node {
				crate::parser::Token::$token => (),
				_ => return Err(crate::source::Spanned::new(error, token.span)),
			},
			None => return Err(crate::source::Spanned::new(error, $end_span)),
		}
	}};
}

macro_rules! identifier {
    ($lexer: expr, $end_span: expr) => {{
		let error = crate::parser::ParserError::ExpectedIdentifier;
		match $lexer.next() {
			Some(token) => match token.node {
				crate::parser::Token::Identifier(identifier) => {
					let identifier = crate::node::Identifier(identifier);
					Spanned::new(identifier, token.span)
				},
				_ => return Err(crate::source::Spanned::new(error, token.span)),
			},
			None => return Err(crate::source::Spanned::new(error, $end_span)),
		}
	}};
}
