use crate::basic::{Compound, Item};
use crate::node::{Arithmetic, BinaryOperator, UnaryOperator, Variable};

use super::{EvaluationError, FrameContext};

pub fn binding(frame: &mut FrameContext, variable: &Variable,
               compound: &Compound) -> Result<(), EvaluationError> {
	let object = object(frame, compound)?;
	frame.insert(variable.clone(), object);
	Ok(())
}

macro_rules! binary_operator {
    ($operator:expr, $left:expr, $right:expr, $identifier:ident) => {
        if let (Item::$identifier(left), Item::$identifier(right)) = ($left, $right) {
		   return Ok(match $operator {
				BinaryOperator::GreaterThan => Item::Truth(left > right),
				BinaryOperator::LessThan => Item::Truth(left < right),
				BinaryOperator::Equality => Item::Truth(left == right),
				BinaryOperator::Arithmetic(operator) => Item::$identifier(match operator {
					Arithmetic::Add => left.wrapping_add(*right),
					Arithmetic::Minus => left.wrapping_sub(*right),
					Arithmetic::Multiply => left.checked_mul(*right)
						.ok_or(EvaluationError::ArithmeticOverflow)?,
				}),
		   })
        }
    };
}

fn object(frame: &mut FrameContext, compound: &Compound) -> Result<Item, EvaluationError> {
	Ok(match compound {
		Compound::Value(value) => frame.value(value).clone(),
		Compound::Unary(operator, value) => {
			let item = frame.value(value);
			match item {
				Item::Signed64(value) => match operator {
					UnaryOperator::Negate => Item::Signed64(-*value),
				}
				_ => panic!("Cannot perform unary operation on invalid value")
			}
		}
		Compound::Binary(operator, left, right) => {
			let (left, right) = (frame.value(left), frame.value(right));
			binary_operator!(operator, left, right, Unsigned64);
			binary_operator!(operator, left, right, Signed64);

			match (left, right) {
				(Item::Truth(left), Item::Truth(right)) => match operator {
					BinaryOperator::Equality => Item::Truth(left == right),
					_ => panic!("Cannot perform operation: {:?}, on truth values", operator),
				}
				_ => panic!("Cannot perform binary operation on values: {:?}, and: {:?}", left, right)
			}
		}
	})
}
