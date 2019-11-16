use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use crate::basic::{Item, Location, Projection, Value};
use crate::node::Variable;

use super::{EvaluationInstance, EvaluationItem};

#[derive(Debug)]
pub struct ValueContext {
	pub stack: DropStack,
	pub values: ValueStack,
}

impl ValueContext {
	pub fn new(frame: ValueFrame) -> Self {
		ValueContext {
			stack: DropStack::default(),
			values: ValueStack {
				frames: vec![frame],
			},
		}
	}
}

impl Deref for ValueContext {
	type Target = ValueStack;

	fn deref(&self) -> &Self::Target {
		&self.values
	}
}

impl DerefMut for ValueContext {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.values
	}
}

#[derive(Debug)]
pub struct ValueStack {
	pub frames: Vec<ValueFrame>,
}

impl ValueStack {
	pub fn item(&mut self, value: &Value) -> Item<EvaluationInstance> {
		match self.value(value) {
			EvaluationItem::Item(item) => item,
			_ => panic!("Value must be an item"),
		}
	}

	pub fn value(&mut self, value: &Value) -> EvaluationItem {
		match value {
			Value::Item(item) => EvaluationItem::item(item),
			Value::Location(location) => self.location(location).clone(),
		}
	}

	pub fn location(&mut self, location: &Location) -> &mut EvaluationItem {
		self.frame().location(location)
	}

	pub fn frame_index(&mut self) -> usize {
		self.frames.len().checked_sub(1).expect("Value frame stack is empty")
	}

	pub fn frame(&mut self) -> &mut ValueFrame {
		self.frames.last_mut().expect("Value frame stack is empty")
	}
}

#[derive(Debug, Default)]
pub struct ValueFrame {
	pub items: HashMap<Variable, EvaluationItem>,
}

impl ValueFrame {
	pub fn location(&mut self, location: &Location) -> &mut EvaluationItem {
		let item = self.items.get_mut(&location.variable).unwrap_or_else(||
			panic!("Variable: {}, does not exist in frame", location.variable));
		location.projections.iter().fold(item, |item, projection| match projection {
			Projection::Field(field) => match item {
				EvaluationItem::Item(item) => match item {
					Item::Instance(instance) => instance.fields.get_mut(field)
						.unwrap_or_else(|| panic!("Field: {}, does not exist on instance", field)),
					_ => panic!("Field access can only be performed on instance"),
				},
				EvaluationItem::Reference(_, _) => panic!("Field access cannot be performed on reference"),
			},
		})
	}
}

#[derive(Debug, Default)]
pub struct DropStack {
	stack: Vec<EvaluationItem>,
}

impl DropStack {
	pub fn drop(&mut self, item: EvaluationItem) {
		self.stack.push(item);
	}

	pub fn restore(&mut self) -> EvaluationItem {
		self.stack.pop().expect("Cannot restore from empty drop stack")
	}
}
