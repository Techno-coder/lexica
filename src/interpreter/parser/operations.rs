use std::clone::Clone;

use crate::interpreter::instruction::InstructionTarget;
use crate::interpreter::operations::*;
use crate::source::{Span, Spanned};

use super::{Comparator, Float, Integer, InterpreterResult, LocalTable, LocalTarget,
            Operation, OperationIdentifier, ParserContext, ParserError, ParserResult, Primitive,
            Size, Token, TranslationUnit};

type Operand<'a> = Spanned<Token<'a>>;

pub fn match_operation<'a>(span: &Span, operation: &OperationIdentifier, operands: &Vec<Operand<'a>>,
                           context: &ParserContext, unit: &TranslationUnit) -> ParserResult<'a, Operation> {
	let function = context
		.last_function_label
		.and_then(|label| unit.functions.get(label))
		.ok_or(Spanned::new(ParserError::FunctionMissingContext, span.clone()));
	let table = function.clone().map(|function| &function.locals);

	Ok(match operation {
		OperationIdentifier::ReversalHint => Operation::ReversalHint,
		OperationIdentifier::Pass => Operation::Pass,
		OperationIdentifier::Swap => {
			let (left, right) = (local(&operands[0])?, local(&operands[1])?);
			Operation::Swap(error(Swap::new(table?, left, right), span)?)
		}
		OperationIdentifier::Add => {
			let (left, right) = (local(&operands[0])?, local(&operands[1])?);
			Operation::Add(error(Add::new(table?, left, right), span)?)
		}
		OperationIdentifier::AddImmediate => {
			let (local, primitive) = (local(&operands[0])?, primitive(&operands[1])?);
			Operation::AddImmediate(error(AddImmediate::new(table?, local, primitive), span)?)
		}
		OperationIdentifier::Minus => {
			let (left, right) = (local(&operands[0])?, local(&operands[1])?);
			Operation::Minus(error(Minus::new(table?, left, right), span)?)
		}
		OperationIdentifier::MinusImmediate => {
			let (local, primitive) = (local(&operands[0])?, primitive(&operands[1])?);
			Operation::MinusImmediate(error(MinusImmediate::new(table?, local, primitive), span)?)
		}
		OperationIdentifier::Drop => {
			let local = local(&operands[0])?;
			Operation::Drop(error(Drop::new(table?, local), span)?)
		}
		OperationIdentifier::DropImmediate => {
			let (size, primitive) = (size(&operands[0])?, primitive(&operands[1])?);
			Operation::DropImmediate(error(DropImmediate::new(size, primitive), span)?)
		}
		OperationIdentifier::Restore => {
			let local = local(&operands[0])?;
			Operation::Restore(error(Restore::new(table?, local), span)?)
		}
		OperationIdentifier::Discard => {
			let size = size(&operands[0])?;
			Operation::Discard(Discard::new(size))
		}
		OperationIdentifier::Reset => {
			let (local, primitive) = (local(&operands[0])?, primitive(&operands[1])?);
			Operation::Reset(error(Reset::new(table?, local, primitive), span)?)
		}
		OperationIdentifier::Clone => {
			let (left, right) = (local(&operands[0])?, local(&operands[1])?);
			Operation::Clone(error(CloneLocal::new(table?, left, right), span)?)
		}
		OperationIdentifier::Call => {
			let target = target(&operands[0])?;
			let target = unit.functions.get(&target)
			                 .map(|function| function.target.clone())
			                 .ok_or(Spanned::new(ParserError::UndefinedFunction(target),
			                                     operands[0].span.clone()))?;
			Operation::Call(Call::new(target))
		}
		OperationIdentifier::Return => {
			// TODO
			Operation::Return
		}
		OperationIdentifier::Jump => {
			let target = target_label(span, &operands[0], unit, context)?;
			Operation::Jump(Jump::new(target))
		}
		OperationIdentifier::Branch => {
			let comparator = comparator(&operands[0])?;
			let (left, right) = (local(&operands[1])?, local(&operands[2])?);
			let target = target_label(span, &operands[3], unit, context)?;
			Operation::Branch(error(Branch::new(table?, comparator, left, right, target), span)?)
		}
		OperationIdentifier::BranchImmediate => {
			let comparator = comparator(&operands[0])?;
			let (left, right) = (local(&operands[1])?, primitive(&operands[2])?);
			let target = target_label(span, &operands[3], unit, context)?;
			Operation::BranchImmediate(error(BranchImmediate::new(table?, comparator, left, right, target), span)?)
		}
	})
}

fn error<'a, 'b, T>(result: InterpreterResult<T>, span: &'a Span) -> ParserResult<'b, T> {
	result.map_err(|error| Spanned::new(ParserError::Interpreter(error), span.clone()))
}

fn local<'a>(local: &Operand<'a>) -> ParserResult<'a, LocalTarget> {
	match local.node {
		Token::UnsignedInteger(integer) => Ok(LocalTarget(integer as usize)),
		_ => Err(local.map(|token| ParserError::UnexpectedOperand(token.clone())))
	}
}

fn primitive<'a>(primitive: &Operand<'a>) -> ParserResult<'a, Primitive> {
	Ok(match primitive.node {
		Token::UnsignedInteger(integer) => Primitive::Integer(Integer::Unsigned64(integer)),
		Token::SignedInteger(integer) => Primitive::Integer(Integer::Signed64(integer)),
		Token::Float(float) => Primitive::Float(Float::Float64(float)),
		_ => return Err(primitive.map(|token| ParserError::UnexpectedOperand(token.clone())))
	})
}

fn size<'a>(size: &Operand<'a>) -> ParserResult<'a, Size> {
	match size.node {
		Token::Identifier(identifier) => {
			Size::parse(identifier).map_err(|error| Spanned::new(error, size.span.clone()))
		}
		_ => Err(size.map(|token| ParserError::UnexpectedOperand(token.clone())))
	}
}

fn target<'a>(target: &Operand<'a>) -> ParserResult<'a, String> {
	Ok(match target.node {
		Token::Identifier(identifier) => identifier.to_owned(),
		Token::UnsignedInteger(integer) => integer.to_string(),
		Token::SignedInteger(integer) => integer.to_string(),
		_ => return Err(target.map(|token| ParserError::UnexpectedOperand(token.clone())))
	})
}

fn target_label<'a>(span: &Span, target_label: &Operand<'a>, unit: &TranslationUnit,
                    context: &ParserContext) -> ParserResult<'a, InstructionTarget> {
	let target = target(target_label)?;
	unit.labels.get(&target).map(|(label, _)| label.clone())
	    .or_else(|| {
		    let label = context.last_function_label?;
		    let (_, local_labels) = unit.labels.get(label)?;
		    local_labels.get(&target).cloned()
	    })
	    .ok_or(Spanned::new(ParserError::UndefinedLabel(target), span.clone()))
}

fn comparator<'a>(comparator: &Operand<'a>) -> ParserResult<'a, Comparator> {
	Ok(match comparator.node {
		Token::Equal => Comparator::Equal,
		Token::LessThan => Comparator::LessThan,
		Token::LessThanEqual => Comparator::LessThanEqual,
		Token::GreaterThan => Comparator::GreaterThan,
		Token::GreaterThanEqual => Comparator::GreaterThanEqual,
		_ => return Err(comparator.map(|token| ParserError::UnexpectedOperand(token.clone())))
	})
}
