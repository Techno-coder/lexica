use crate::basic::{Compound, Item, Value};
use crate::node::{Arithmetic, BinaryOperator, UnaryOperator, Variable};

use super::{EvaluationError, EvaluationItem, ValueContext};

macro_rules! binary_integer {
    ($operator:expr, $left:expr, $right:expr, $identifier:ident) => {
        if let (Item::$identifier(left), Item::$identifier(right)) = ($left, $right) {
		   return Ok(EvaluationItem::Item(match $operator {
				BinaryOperator::GreaterThan => Item::Truth(left > right),
				BinaryOperator::GreaterEqual => Item::Truth(left >= right),
				BinaryOperator::LessThan => Item::Truth(left < right),
				BinaryOperator::LessEqual => Item::Truth(left <= right),
				BinaryOperator::Equality => Item::Truth(left == right),
				BinaryOperator::Arithmetic(operator) => Item::$identifier(match operator {
					Arithmetic::Add => left.wrapping_add(*right),
					Arithmetic::Minus => left.wrapping_sub(*right),
					Arithmetic::Multiply => left.checked_mul(*right)
						.ok_or(EvaluationError::ArithmeticOverflow)?,
				}),
		   }));
        }
    };
}

pub fn binding(context: &mut ValueContext, variable: &Variable,
               compound: &Compound) -> Result<(), EvaluationError> {
	let item = item(context, compound)?;
	context.frame().items.insert(variable.clone(), item);
	Ok(())
}

fn item(context: &mut ValueContext, compound: &Compound) -> Result<EvaluationItem, EvaluationError> {
	Ok(match compound {
		Compound::Value(value) => context.value(value),
		Compound::Unary(operator, value) => match operator {
			UnaryOperator::Reference(_) => match value {
				Value::Location(location) =>
					EvaluationItem::Reference(context.frame_index(), location.clone()),
				Value::Item(_) => panic!("Cannot reference item immediate"),
			}
			UnaryOperator::Negate => EvaluationItem::Item(match context.item(value) {
				Item::Signed8(integer) => Item::Signed8(-integer),
				Item::Signed16(integer) => Item::Signed16(-integer),
				Item::Signed32(integer) => Item::Signed32(-integer),
				Item::Signed64(integer) => Item::Signed64(-integer),
				_ => panic!("Negation on invalid value"),
			})
		},
		Compound::Binary(operator, left, right) => {
			let (left, right) = (&context.item(left), &context.item(right));
			binary_integer!(operator, left, right, Unsigned8);
			binary_integer!(operator, left, right, Unsigned16);
			binary_integer!(operator, left, right, Unsigned32);
			binary_integer!(operator, left, right, Unsigned64);
			binary_integer!(operator, left, right, Signed8);
			binary_integer!(operator, left, right, Signed16);
			binary_integer!(operator, left, right, Signed32);
			binary_integer!(operator, left, right, Signed64);
			EvaluationItem::Item(match (left, right) {
				(Item::Truth(left), Item::Truth(right)) => match operator {
					BinaryOperator::Equality => Item::Truth(left == right),
					_ => panic!("Invalid operation: {:?}, on truth values", operator),
				}
				_ => panic!("Invalid binary operation on items: {:?}, and: {:?}", left, right)
			})
		}
		Compound::FunctionCall(_, _) => unreachable!(),
	})
}
