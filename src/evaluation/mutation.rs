use crate::basic::{Direction, Item, Location, Value};
use crate::node::{Arithmetic, MutationKind};

use super::{DropStack, EvaluationError, FrameContext};

pub fn mutation(frame: &mut FrameContext, stack: &mut DropStack, mutation: &MutationKind,
                location: &Location, value: &Value, direction: Direction) -> Result<(), EvaluationError> {
	frame.location(location, |frame, mutable| {
		match mutation {
			MutationKind::Arithmetic(operator) => arithmetic(frame, stack,
				mutable, value, operator, direction),
			MutationKind::Assign => Ok(*mutable = frame.value(value).clone()),
			MutationKind::Swap => Ok(swap(frame, location, mutable, value)),
		}
	})
}

macro_rules! arithmetic {
    ($stack:expr, $mutable:expr, $object:expr, $arithmetic:expr, $direction:expr, $item:ident) => {
		if let Item::$item(integer) = $mutable {
			if let Item::$item(object) = $object {
				return match $direction {
					Direction::Advance => match $arithmetic {
						Arithmetic::Add => Ok(*integer = integer.wrapping_add(*object)),
						Arithmetic::Minus => Ok(*integer = integer.wrapping_sub(*object)),
						Arithmetic::Multiply => {
							let value = integer.checked_mul(*object)
								.ok_or(EvaluationError::ArithmeticOverflow)?;
							if value == 0 { $stack.drop(Item::$item(*integer)) }
							Ok(*integer = value)
						}
					}
					Direction::Reverse => match $arithmetic {
						Arithmetic::Add => Ok(*integer = integer.wrapping_sub(*object)),
						Arithmetic::Minus => Ok(*integer = integer.wrapping_add(*object)),
						Arithmetic::Multiply => Ok(match integer {
							0 => *$mutable = $stack.restore(),
							_ => *integer /= object,
						}),
					}
				};
			}
		}
    };
}

fn arithmetic(frame: &FrameContext, stack: &mut DropStack, mutable: &mut Item, value: &Value,
              arithmetic: &Arithmetic, direction: Direction) -> Result<(), EvaluationError> {
	let object = frame.value(value);
	arithmetic!(stack, mutable, object, arithmetic, direction, Unsigned64);
	arithmetic!(stack, mutable, object, arithmetic, direction, Signed64);
	panic!("Cannot perform mutation arithmetic on invalid values")
}

fn swap(frame: &mut FrameContext, location: &Location, mutable: &mut Item, value: &Value) {
	match value {
		Value::Location(other) => frame.location(other, |_, object| std::mem::swap(mutable, object)),
		Value::Item(_) => panic!("Cannot swap location: {}, with immediate object", location),
	}
}
