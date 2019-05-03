use std::clone::Clone;

use crate::interpreter::instruction::InstructionTarget;
use crate::interpreter::operations::*;
use crate::source::{Span, Spanned};

use super::{Comparator, Float, Integer, InterpreterResult, LocalTarget, RefactorOperation, OperationIdentifier,
            ParserContext, ParserError, ParserResult, Primitive, Size, Token, TranslationUnit};

type Operand<'a> = Spanned<Token<'a>>;

/// Parses an operation given the operation identifier and operands.
pub fn match_operation<'a>(span: &Span, operation: &OperationIdentifier, operands: &Vec<Operand<'a>>,
                           context: &ParserContext, unit: &TranslationUnit) -> ParserResult<'a, RefactorOperation> {
	let function = context
		.last_function_label
		.and_then(|label| unit.functions.get(label))
		.ok_or(Spanned::new(ParserError::FunctionMissingContext, span.clone()));
	let table = function.clone().map(|function| &function.locals);

	Ok(match operation {
		OperationIdentifier::ReversalHint => RefactorOperation::ReversalHint,
		OperationIdentifier::Pass => RefactorOperation::Pass,
		OperationIdentifier::Swap => {
			let (left, right) = (local(&operands[0])?, local(&operands[1])?);
			RefactorOperation::Swap(error(Swap::new(table?, left, right), span)?)
		}
		OperationIdentifier::Add => {
			let (left, right) = (local(&operands[0])?, local(&operands[1])?);
			RefactorOperation::Add(error(Add::new(table?, left, right), span)?)
		}
		OperationIdentifier::AddImmediate => {
			let (local, primitive) = (local(&operands[0])?, primitive(&operands[1])?);
			RefactorOperation::AddImmediate(error(AddImmediate::new(table?, local, primitive), span)?)
		}
		OperationIdentifier::Minus => {
			let (left, right) = (local(&operands[0])?, local(&operands[1])?);
			RefactorOperation::Minus(error(Minus::new(table?, left, right), span)?)
		}
		OperationIdentifier::MinusImmediate => {
			let (local, primitive) = (local(&operands[0])?, primitive(&operands[1])?);
			RefactorOperation::MinusImmediate(error(MinusImmediate::new(table?, local, primitive), span)?)
		}
		OperationIdentifier::Drop => {
			let local = local(&operands[0])?;
			RefactorOperation::Drop(error(Drop::new(table?, local), span)?)
		}
		OperationIdentifier::DropImmediate => {
			let (size, primitive) = (size(&operands[0])?, primitive(&operands[1])?);
			RefactorOperation::DropImmediate(error(DropImmediate::new(size, primitive), span)?)
		}
		OperationIdentifier::Restore => {
			let local = local(&operands[0])?;
			RefactorOperation::Restore(error(Restore::new(table?, local), span)?)
		}
		OperationIdentifier::Discard => {
			let size = size(&operands[0])?;
			RefactorOperation::Discard(Discard::new(size))
		}
		OperationIdentifier::Reset => {
			let (local, primitive) = (local(&operands[0])?, primitive(&operands[1])?);
			RefactorOperation::Reset(error(Reset::new(table?, local, primitive), span)?)
		}
		OperationIdentifier::Clone => {
			let (left, right) = (local(&operands[0])?, local(&operands[1])?);
			RefactorOperation::Clone(error(CloneLocal::new(table?, left, right), span)?)
		}
		OperationIdentifier::Call => {
			let target = target(&operands[0])?;
			let (target, reverse_target) = unit
				.functions.get(&target)
				.map(|function| (function.target.clone(), function.reverse_target.clone()))
				.ok_or(operands[0].map(|_| ParserError::UndefinedFunction(target)))?;
			RefactorOperation::Call(Call::new(target, reverse_target))
		}
		OperationIdentifier::Recall => {
			let target = target(&operands[0])?;
			let function = unit.functions.get(&target)
				.ok_or(operands[0].map(|_| ParserError::UndefinedFunction(target)))?;
			let reverse_target = function.reverse_target.clone()
				.ok_or(operands[0].map(|_| ParserError::IrreversibleCall))?;
			RefactorOperation::Recall(Recall::new(function.target.clone(), reverse_target))
		}
		OperationIdentifier::Return => RefactorOperation::Return(Return),
		OperationIdentifier::Exit => RefactorOperation::Exit,
		OperationIdentifier::Jump => {
			let target = target_label(span, &operands[0], unit, context)?;
			RefactorOperation::Jump(Jump::new(target))
		}
		OperationIdentifier::Branch => {
			let comparator = comparator(&operands[0])?;
			let (left, right) = (local(&operands[1])?, local(&operands[2])?);
			let target = target_label(span, &operands[3], unit, context)?;
			RefactorOperation::Branch(error(Branch::new(table?, comparator, left, right, target), span)?)
		}
		OperationIdentifier::BranchImmediate => {
			let comparator = comparator(&operands[0])?;
			let (left, right) = (local(&operands[1])?, primitive(&operands[2])?);
			let target = target_label(span, &operands[3], unit, context)?;
			RefactorOperation::BranchImmediate(error(BranchImmediate::new(table?, comparator, left, right, target), span)?)
		}
	})
}

/// Transforms an `InterpreterResult` into a `ParserResult`.
fn error<'a, 'b, T>(result: InterpreterResult<T>, span: &'a Span) -> ParserResult<'b, T> {
	result.map_err(|error| Spanned::new(ParserError::Interpreter(error), span.clone()))
}

/// Transforms the operand into a `LocalTarget`.
fn local<'a>(local: &Operand<'a>) -> ParserResult<'a, LocalTarget> {
	match local.node {
		Token::UnsignedInteger(integer) => Ok(LocalTarget(integer as usize)),
		_ => Err(local.map(|token| ParserError::UnexpectedOperand(token.clone())))
	}
}

/// Transforms the operand into a `Primitive`.
fn primitive<'a>(primitive: &Operand<'a>) -> ParserResult<'a, Primitive> {
	Ok(match primitive.node {
		Token::UnsignedInteger(integer) => Primitive::Integer(Integer::Unsigned64(integer)),
		Token::SignedInteger(integer) => Primitive::Integer(Integer::Signed64(integer)),
		Token::Float(float) => Primitive::Float(Float::Float64(float)),
		_ => return Err(primitive.map(|token| ParserError::UnexpectedOperand(token.clone())))
	})
}

/// Transforms the operand into a `Size`.
fn size<'a>(size: &Operand<'a>) -> ParserResult<'a, Size> {
	match size.node {
		Token::Identifier(identifier) => {
			Size::parse(identifier).map_err(|error| Spanned::new(error, size.span.clone()))
		}
		_ => Err(size.map(|token| ParserError::UnexpectedOperand(token.clone())))
	}
}

/// Transforms the operand into a label target string.
fn target<'a>(target: &Operand<'a>) -> ParserResult<'a, String> {
	Ok(match target.node {
		Token::Identifier(identifier) => identifier.to_owned(),
		Token::UnsignedInteger(integer) => integer.to_string(),
		Token::SignedInteger(integer) => integer.to_string(),
		_ => return Err(target.map(|token| ParserError::UnexpectedOperand(token.clone())))
	})
}

/// Transforms the operand into an `InstructionTarget`.
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

/// Transforms the operand into a `Comparator`.
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
