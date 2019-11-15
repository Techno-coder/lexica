use crate::basic::{Direction, Item, Location, Reversibility, Value};
use crate::node::{Arithmetic, MutationKind};

use super::{DropStack, EvaluationError, EvaluationItem, ValueContext};

macro_rules! arithmetic {
    ($stack:expr, $reversibility:expr, $direction:expr,
    $arithmetic:expr, $mutable:expr, $item:expr, $identifier:ident) => {
		if let EvaluationItem::Item(Item::$identifier(integer)) = $mutable {
			if let EvaluationItem::Item(Item::$identifier(other)) = $item {
				return Ok(match $direction {
					Direction::Advance => match $arithmetic {
						Arithmetic::Add => *integer = integer.wrapping_add(other),
						Arithmetic::Minus => *integer = integer.wrapping_sub(other),
						Arithmetic::Multiply => {
							let value = integer.checked_mul(other)
								.ok_or(EvaluationError::ArithmeticOverflow)?;
							if value == 0 && $reversibility == &Reversibility::Reversible {
								$stack.drop(EvaluationItem::Item(Item::$identifier(*integer)))
							}
							*integer = value
						}
					}
					Direction::Reverse => match $arithmetic {
						Arithmetic::Add => *integer = integer.wrapping_sub(other),
						Arithmetic::Minus => *integer = integer.wrapping_add(other),
						Arithmetic::Multiply => match integer {
							0 => *$mutable = $stack.restore(),
							_ => *integer /= other,
						},
					}
				});
			}
		}
    };
}

pub fn mutation(context: &mut ValueContext, reversibility: &Reversibility, direction: Direction,
                mutation: &MutationKind, location: &Location, value: &Value) -> Result<(), EvaluationError> {
	let item = context.value(value);
	let mutable = context.values.location(location);
	match mutation {
		MutationKind::Arithmetic(operator) => arithmetic(&mut context.stack,
			reversibility, direction, operator, mutable, item),
		MutationKind::Assign => Ok(*mutable = item),
		MutationKind::Swap => match value {
			Value::Item(_) => panic!("Cannot swap location: {}, with item immediate", location),
			Value::Location(other) => {
				let item = std::mem::replace(mutable, item);
				std::mem::replace(context.location(other), item);
				Ok(())
			}
		},
	}
}

fn arithmetic(stack: &mut DropStack, reversibility: &Reversibility, direction: Direction,
              arithmetic: &Arithmetic, mutable: &mut EvaluationItem, item: EvaluationItem)
              -> Result<(), EvaluationError> {
	arithmetic!(stack, reversibility, direction, arithmetic, mutable, item, Unsigned8);
	arithmetic!(stack, reversibility, direction, arithmetic, mutable, item, Unsigned16);
	arithmetic!(stack, reversibility, direction, arithmetic, mutable, item, Unsigned32);
	arithmetic!(stack, reversibility, direction, arithmetic, mutable, item, Unsigned64);
	arithmetic!(stack, reversibility, direction, arithmetic, mutable, item, Signed8);
	arithmetic!(stack, reversibility, direction, arithmetic, mutable, item, Signed16);
	arithmetic!(stack, reversibility, direction, arithmetic, mutable, item, Signed32);
	arithmetic!(stack, reversibility, direction, arithmetic, mutable, item, Signed64);
	panic!("Mutation arithmetic on invalid values")
}
