use super::{InterpreterError, InterpreterResult, Primitive};

#[derive(Debug)]
pub enum Comparator {
	Equal,
	LessThan,
	LessThanEqual,
	GreaterThan,
	GreaterThanEqual,
}

impl Comparator {
	pub fn compare(&self, left: &Primitive, right: &Primitive) -> InterpreterResult<bool> {
		use self::Comparator::*;
		use super::Primitive::*;
		match (left, right) {
			(Boolean(left), Boolean(right)) => match self {
				Equal => Ok(left == right),
				_ => Err(InterpreterError::UndefinedComparison)
			}
			(Integer(left), Integer(right)) => {
				let left = left.extend_unsigned();
				let right = right.extend_unsigned();
				Ok(match self {
					Equal => left == right,
					LessThan => left < right,
					LessThanEqual => left <= right,
					GreaterThan => left > right,
					GreaterThanEqual => left >= right,
				})
			}
			(Float(left), Float(right)) => {
				let left = left.extend();
				let right = right.extend();
				Ok(match self {
					Equal => left == right,
					LessThan => left < right,
					LessThanEqual => left <= right,
					GreaterThan => left > right,
					GreaterThanEqual => left >= right,
				})
			}
			_ => Err(InterpreterError::TypesIncompatible)
		}
	}
}
