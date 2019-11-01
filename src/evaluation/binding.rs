use crate::basic::{Compound, Item};
use crate::node::{Arithmetic, BinaryOperator, Variable};

use super::{EvaluationError, FrameContext};

pub fn binding(frame: &mut FrameContext, variable: &Variable,
               compound: &Compound) -> Result<(), EvaluationError> {
	let object = object(frame, compound)?;
	frame.insert(variable.clone(), object);
	Ok(())
}

fn object(frame: &mut FrameContext, compound: &Compound) -> Result<Item, EvaluationError> {
	Ok(match compound {
		Compound::Value(value) => frame.value(value).clone(),
		Compound::Binary(operator, left, right) => {
			let (left, right) = (frame.value(left), frame.value(right));
			match (left, right) {
				(Item::Unsigned64(left), Item::Unsigned64(right)) => match operator {
					BinaryOperator::Equality => Item::Truth(left == right),
					BinaryOperator::Arithmetic(operator) => Item::Unsigned64(match operator {
						Arithmetic::Add => left.wrapping_add(*right),
						Arithmetic::Minus => left.wrapping_sub(*right),
						Arithmetic::Multiply => left.checked_mul(*right)
							.ok_or(EvaluationError::ArithmeticOverflow)?,
					}),
				}
				(Item::Truth(left), Item::Truth(right)) => match operator {
					BinaryOperator::Equality => Item::Truth(left == right),
					BinaryOperator::Arithmetic(_) =>
						panic!("Cannot perform binary arithmetic on truth values"),
				}
				_ => panic!("Cannot perform binary operation on invalid values")
			}
		}
	})
}
