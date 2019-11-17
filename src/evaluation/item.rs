use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use crate::basic::{Instance, Item, Location};
use crate::inference::TypeResolution;

use super::EvaluationError;

pub type FrameIndex = usize;

#[derive(Clone, PartialEq)]
pub enum EvaluationItem {
	Item(Item<EvaluationInstance>),
	Reference(FrameIndex, Location),
}

impl EvaluationItem {
	pub fn item(item: &Item) -> EvaluationItem {
		EvaluationItem::Item(match item {
			Item::Truth(value) => Item::Truth(*value),
			Item::Signed8(value) => Item::Signed8(*value),
			Item::Signed16(value) => Item::Signed16(*value),
			Item::Signed32(value) => Item::Signed32(*value),
			Item::Signed64(value) => Item::Signed64(*value),
			Item::Unsigned8(value) => Item::Unsigned8(*value),
			Item::Unsigned16(value) => Item::Unsigned16(*value),
			Item::Unsigned32(value) => Item::Unsigned32(*value),
			Item::Unsigned64(value) => Item::Unsigned64(*value),
			Item::Instance(instance) => Item::Instance(EvaluationInstance {
				type_resolution: instance.type_resolution.clone(),
				fields: instance.fields.iter().map(|(field, item)|
					(field.clone(), EvaluationItem::item(item))).collect(),
			}),
			Item::Uninitialised => Item::Uninitialised,
			Item::Unit => Item::Unit,
		})
	}

	pub fn collapse(&self) -> Result<Item, EvaluationError> {
		match self {
			EvaluationItem::Reference(_, _) => Err(EvaluationError::RuntimeExpression),
			EvaluationItem::Item(item) => Ok(match item {
				Item::Truth(value) => Item::Truth(*value),
				Item::Signed8(value) => Item::Signed8(*value),
				Item::Signed16(value) => Item::Signed16(*value),
				Item::Signed32(value) => Item::Signed32(*value),
				Item::Signed64(value) => Item::Signed64(*value),
				Item::Unsigned8(value) => Item::Unsigned8(*value),
				Item::Unsigned16(value) => Item::Unsigned16(*value),
				Item::Unsigned32(value) => Item::Unsigned32(*value),
				Item::Unsigned64(value) => Item::Unsigned64(*value),
				Item::Instance(instance) => Item::Instance(Instance {
					type_resolution: instance.type_resolution.clone(),
					fields: instance.fields.iter().map(|(field, item)|
						Ok((field.clone(), item.collapse()?)))
						.collect::<Result<_, _>>()?,
				}),
				Item::Uninitialised => Item::Uninitialised,
				Item::Unit => Item::Unit,
			})
		}
	}
}

impl fmt::Debug for EvaluationItem {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			EvaluationItem::Item(item) => write!(f, "{:?}", item),
			EvaluationItem::Reference(frame, location) => write!(f, "&{}[{}]", frame, location),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct EvaluationInstance {
	pub type_resolution: TypeResolution,
	pub fields: HashMap<Arc<str>, EvaluationItem>,
}
