use crate::basic::{Item, Value, Location};
use crate::node::{Arithmetic, MutationKind};

use super::{EvaluationError, FrameContext};

pub fn mutation(frame: &mut FrameContext, mutation: &MutationKind,
                location: &Location, value: &Value) -> Result<(), EvaluationError> {
	frame.location(location, |frame, mutable| {
		match mutation {
			MutationKind::Arithmetic(operator) => arithmetic(frame, mutable, value, operator),
			MutationKind::Assign => Ok(*mutable = frame.value(value).clone()),
			MutationKind::Swap => Ok(swap(frame, location, mutable, value)),
		}
	})
}

fn arithmetic(frame: &FrameContext, mutable: &mut Item, value: &Value,
              arithmetic: &Arithmetic) -> Result<(), EvaluationError> {
	let object = frame.value(value);
	match (mutable, object) {
		(Item::Unsigned64(mutable), Item::Unsigned64(object)) => match arithmetic {
			Arithmetic::Add => Ok(*mutable = mutable.wrapping_add(*object)),
			Arithmetic::Minus => Ok(*mutable = mutable.wrapping_sub(*object)),
			Arithmetic::Multiply => Ok(*mutable = mutable.checked_mul(*object)
				.ok_or(EvaluationError::ArithmeticOverflow)?),
		}
		_ => panic!("Cannot perform mutation arithmetic on invalid values")
	}
}

fn swap(frame: &mut FrameContext, location: &Location, mutable: &mut Item, value: &Value) {
	match value {
		Value::Location(other) => frame.location(other, |_, object| std::mem::swap(mutable, object)),
		Value::Item(_) => panic!("Cannot swap location: {}, with immediate object", location),
	}
}
