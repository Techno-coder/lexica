use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use crate::basic::{BasicFunction, Branch, Item, Location, Projection, Value};
use crate::node::Variable;

use super::{EvaluationInstance, EvaluationItem, FrameIndex};

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
		let frame = self.frame_index();
		self.dereference(frame, location)
	}

	pub fn dereference(&mut self, frame: FrameIndex, location: &Location) -> &mut EvaluationItem {
		let stack = UnsafeCell::new(self);
		let frame = unsafe { stack.get().as_mut() }.unwrap().frames.get_mut(frame)
			.unwrap_or_else(|| panic!("Frame: {}, does not exist in stack", frame));
		let item = frame.items.get_mut(&location.variable).unwrap_or_else(||
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
			Projection::Dereference => match item {
				EvaluationItem::Item(_) => panic!("Dereference cannot be performed on item"),
				EvaluationItem::Reference(frame, location) => {
					// This is safe as the function never modifies the mutable reference
					// directly. The function returns only one mutable reference and it
					// is associated with the self parameter so it is not possible to
					// obtain more than one mutable reference.
					let stack = unsafe { stack.get().as_mut() }.unwrap();
					stack.dereference(*frame, location)
				}
			}
		})
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
	pub fn advance<I>(function: &BasicFunction, arguments: I) -> Self
		where I: Iterator<Item=EvaluationItem> {
		let mut frame = ValueFrame::default();
		let type_resolution = function.parameter_type();
		let fields = arguments.enumerate().map(|(index, item)|
			(index.to_string().into(), item)).collect();
		let item = Item::Instance(EvaluationInstance { type_resolution, fields });
		frame.items.insert(Variable::new_temporary(0), EvaluationItem::Item(item));
		frame
	}

	pub fn reverse(function: &BasicFunction, return_item: EvaluationItem) -> Self {
		let mut frame = ValueFrame::default();
		let type_resolution = function.parameter_type();
		let fields = function.parameters.iter().enumerate().map(|(index, _)|
			(index.to_string().into(), EvaluationItem::Item(Item::Uninitialised))).collect();
		let item = Item::Instance(EvaluationInstance { type_resolution, fields });
		frame.items.insert(Variable::new_temporary(0), EvaluationItem::Item(item));

		match &function[&function.component.exit].advance.node {
			Branch::Return(Value::Item(_)) => (),
			Branch::Return(Value::Location(location)) => match location.projections.is_empty() {
				true => frame.items.insert(location.variable.clone(), return_item).unwrap_none(),
				false => panic!("Return branch location: {}, cannot have projections", location),
			},
			other => panic!("Branch: {}, must be return in reverse function entry", other),
		}
		frame
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
