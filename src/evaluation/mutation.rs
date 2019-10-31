use crate::basic::{Object, Value};
use crate::node::{Arithmetic, MutationKind, Variable};

use super::{EvaluationError, FrameContext};

pub fn mutation(frame: &mut FrameContext, mutation: &MutationKind,
                variable: &Variable, value: &Value) -> Result<(), EvaluationError> {
	frame.variable(variable, |frame, mutable| {
		match mutation {
			MutationKind::Arithmetic(operator) => arithmetic(frame, mutable, value, operator),
			MutationKind::Assign => Ok(*mutable = frame.value(value).clone()),
			MutationKind::Swap => Ok(swap(frame, variable, mutable, value)),
		}
	})
}

fn arithmetic(frame: &FrameContext, mutable: &mut Object, value: &Value,
              arithmetic: &Arithmetic) -> Result<(), EvaluationError> {
	let object = frame.value(value);
	match (mutable, object) {
		(Object::Unsigned64(mutable), Object::Unsigned64(object)) => match arithmetic {
			Arithmetic::Add => Ok(*mutable = mutable.wrapping_add(*object)),
			Arithmetic::Minus => Ok(*mutable = mutable.wrapping_sub(*object)),
			Arithmetic::Multiply => Ok(*mutable = mutable.checked_mul(*object)
				.ok_or(EvaluationError::ArithmeticOverflow)?),
		}
		_ => panic!("Cannot perform mutation arithmetic on invalid values")
	}
}

fn swap(frame: &mut FrameContext, variable: &Variable, mutable: &mut Object, value: &Value) {
	match value {
		Value::Variable(other) => frame.variable(other, |_, object| std::mem::swap(mutable, object)),
		Value::Object(_) => panic!("Cannot swap variable: {}, with immediate object", variable),
	}
}
