macro_rules! expect {
    ($lexer: expr, $end_span: expr, $token: ident) => {
        expect_token!($lexer, $end_span, crate::parser::Token::$token)
    };
}

macro_rules! expect_token {
    ($lexer: expr, $end_span: expr, $token: expr) => {{
		let error = crate::parser::ParserError::ExpectedToken($token.clone());
		match $lexer.next() {
			Some(token) => match token.node == $token {
				false => return Err(crate::source::Spanned::new(error, token.span).into()),
				true => token.span,
			},
			None => return Err(crate::source::Spanned::new(error, $end_span).into()),
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
				_ => return Err(crate::source::Spanned::new(error, token.span).into()),
			},
			None => return Err(crate::source::Spanned::new(error, $end_span).into()),
		}
	}};
}
