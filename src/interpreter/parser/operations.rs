//use crate::source::{Span, Spanned};
//
//use super::{LocalTarget, Operation, ParserError, ParserResult, Primitive, Token};
//
//type Lexer<'a> = &'a mut Iterator<Item=Spanned<Token<'a>>>;
//
//pub fn parse_operation<'a>(operation: Spanned<&str>, lexer: Lexer<'a>)
//                           -> ParserResult<'a, Operation> {
//	Ok(match operation.node {
//		"pass" => Operation::Pass,
//		"swap" => {
//			let left = local_target(&operation.span, lexer)?;
//			let right = local_target(&operation.span, lexer)?;
//		}
//		_ => unimplemented!(),
//	})
//}
//
//fn local_target<'a>(operation_span: &Span, lexer: Lexer<'a>) -> ParserResult<'a, LocalTarget> {
//	let target = lexer.next().ok_or(Spanned::new(ParserError::EndOfInput, operation_span.clone()))?;
//	match target.node {
//		Token::UnsignedInteger(integer) => Ok(LocalTarget(integer as usize)),
//		_ => Err(target.map(|node| ParserError::UnexpectedOperand(node.clone()))),
//	}
//}