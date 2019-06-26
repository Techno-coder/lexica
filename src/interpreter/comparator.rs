use std::fmt;

use crate::interpreter::Integer;

use super::{InterpreterError, InterpreterResult, Primitive};

use self::Comparator::*;

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
		use super::Primitive::*;
		match (left, right) {
			(Boolean(left), Boolean(right)) => match self {
				Equal => Ok(left == right),
				_ => Err(InterpreterError::UndefinedComparison)
			}
			(Integer(left), Integer(right)) => Ok(self.compare_integer(left, right)),
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

	pub fn compare_integer(&self, left: &Integer, right: &Integer) -> bool {
		match left.extend_unsigned() {
			Ok(left) => match right.extend_unsigned() {
				Ok(right) => self.compare_unsigned_integer(left, right),
				Err(right) => self.compare_mixed_integer(left, right),
			},
			Err(left) => match right.extend_unsigned() {
				Ok(right) => self.compare_mixed_integer(right, left),
				Err(right) => self.compare_unsigned_integer(left as u64, right as u64),
			}
		}
	}

	pub fn compare_mixed_integer(&self, unsigned: u64, signed: i64) -> bool {
		match signed >= 0 {
			true => self.compare_unsigned_integer(unsigned, signed as u64),
			false => false,
		}
	}

	pub fn compare_unsigned_integer(&self, left: u64, right: u64) -> bool {
		match self {
			Equal => left == right,
			LessThan => left < right,
			LessThanEqual => left <= right,
			GreaterThan => left > right,
			GreaterThanEqual => left >= right,
		}
	}
}

impl fmt::Display for Comparator {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Equal => write!(f, "="),
			LessThan => write!(f, "<"),
			LessThanEqual => write!(f, "<="),
			GreaterThan => write!(f, ">"),
			GreaterThanEqual => write!(f, ">="),
		}
	}
}
