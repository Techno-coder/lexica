use std::sync::Arc;

use crate::basic::{BasicFunction, Branch, Compound, Direction, Discriminant, Item,
	NodeTarget, Reversibility, Statement, Value};
use crate::context::Context;
use crate::error::Diagnostic;
use crate::span::Spanned;

use super::{EvaluationError, EvaluationItem, ValueContext, ValueFrame};

#[derive(Debug)]
pub struct EvaluationContext<'a> {
	pub values: ValueContext,
	pub functions: Vec<FunctionFrame>,
	reversibility: Reversibility,
	context: &'a Context,
}

impl<'a> EvaluationContext<'a> {
	pub fn new(context: &'a Context, reversibility: Reversibility,
	           function: FunctionFrame, values: ValueFrame) -> Result<Self, Diagnostic> {
		let (values, functions) = (ValueContext::new(values), vec![function]);
		Ok(EvaluationContext { values, functions, reversibility, context })
	}

	pub fn resume(&mut self, direction: Direction) -> Result<EvaluationItem, Diagnostic> {
		loop {
			if let Some(item) = self.step(direction)? {
				return Ok(item);
			}
		}
	}

	pub fn step(&mut self, direction: Direction) -> Result<Option<EvaluationItem>, Diagnostic> {
		match direction {
			Direction::Advance => self.advance(),
			Direction::Reverse => self.reverse(),
		}
	}

	fn advance(&mut self) -> Result<Option<EvaluationItem>, Diagnostic> {
		let frame = self.frame();
		let node = &frame.function[&frame.node];
		if frame.statement == node.statements.len() {
			match self.branch(Direction::Advance)? {
				Some(item) => match self.functions.last_mut() {
					Some(frame) => match &frame.statement().node {
						Statement::Binding(variable, Compound::FunctionCall(_, _)) => {
							self.values.frame().items.insert(variable.clone(), item);
							frame.statement += 1;
							Ok(None)
						}
						_ => panic!("Cannot return into statement that is not function call"),
					},
					None => Ok(Some(item))
				}
				None => {
					self.frame().statement = 0;
					Ok(None)
				}
			}
		} else {
			if !self.execute(Direction::Advance)? {
				self.frame().statement += 1;
			}
			Ok(None)
		}
	}

	fn reverse(&mut self) -> Result<Option<EvaluationItem>, Diagnostic> {
		let frame = self.frame();
		if frame.statement == 0 {
			match self.branch(Direction::Reverse)? {
				Some(item) => match self.functions.last_mut() {
					Some(frame) => match &frame.statement().node {
						Statement::Binding(_, Compound::FunctionCall(_, values)) => match item {
							EvaluationItem::Item(Item::Instance(mut instance)) => {
								for (index, value) in values.iter().enumerate() {
									if let Value::Location(location) = value {
										*self.values.location(location) = instance.fields
											.remove(index.to_string().as_str()).unwrap();
									}
								}
								Ok(None)
							}
							_ => panic!("Parameter item must be tuple instance")
						},
						_ => panic!("Cannot return into statement that is not function call"),
					},
					None => Ok(Some(item))
				}
				None => {
					let frame = self.frame();
					frame.statement = frame.function[&frame.node].statements.len();
					Ok(None)
				}
			}
		} else {
			self.frame().statement -= 1;
			self.execute(Direction::Reverse)?;
			Ok(None)
		}
	}

	/// Executes the current statement. Returns true if a function call was invoked.
	fn execute(&mut self, direction: Direction) -> Result<bool, Diagnostic> {
		let frame = self.functions.last_mut().expect("Evaluation function stack is empty");
		let direction = direction ^ frame.function[&frame.node].direction;
		let statement = frame.statement();
		let values = &mut self.values;
		match &statement.node {
			Statement::Binding(variable, compound) => match compound {
				Compound::FunctionCall(path, arguments) => {
					let function = crate::basic::function(&self.context, path, self.reversibility)?;
					let frame = match direction {
						Direction::Advance => ValueFrame::advance(&function, arguments
							.iter().map(|argument| values.value(argument))),
						Direction::Reverse => ValueFrame::reverse(&function,
							values.frame().items[variable].clone()),
					};

					self.functions.push(FunctionFrame::new(function, direction));
					values.frames.push(frame);
					return Ok(true);
				}
				_ => super::binding::binding(&mut self.values, direction, variable, compound),
			}
			Statement::Mutation(mutation, location, value) =>
				super::mutation::mutation(&mut self.values, &self.reversibility,
					direction, mutation, location, value),
			Statement::ImplicitDrop(location) => Ok(match direction {
				Direction::Reverse => *self.values.location(location) = self.values.stack.restore(),
				Direction::Advance => {
					let item = self.values.location(location).clone();
					self.values.stack.drop(item);
				}
			}),
		}.map_err(|error| Diagnostic::new(Spanned::new(error, statement.span))).map(|_| false)
	}

	/// Evaluates the current node branch. Returns an item on a return branch.
	fn branch(&mut self, direction: Direction) -> Result<Option<EvaluationItem>, Diagnostic> {
		let frame = self.functions.last_mut().expect("Evaluation function stack is empty");
		let node = &frame.function[&frame.node];
		let branch = &node[direction];
		match &branch.node {
			Branch::Jump(target) => frame.node = *target,
			Branch::Divergence(divergence) => {
				let item = self.values.value(&divergence.discriminant);
				let discriminant = Discriminant::item(item.collapse().unwrap());
				frame.node = divergence.branches.iter().find(|(value, _)| value == &discriminant)
					.map(|(_, target)| *target).unwrap_or(divergence.default);
			}
			Branch::Return(value) => {
				let item = self.values.value(value);
				self.values.frames.pop().unwrap();
				self.functions.pop().unwrap();
				return Ok(Some(item));
			}
			Branch::Unreachable => {
				let error = EvaluationError::UnreachableBranch;
				return Err(Diagnostic::new(Spanned::new(error, branch.span)));
			}
		}
		Ok(None)
	}

	fn frame(&mut self) -> &mut FunctionFrame {
		self.functions.last_mut().expect("Evaluation function stack is empty")
	}
}

#[derive(Debug)]
pub struct FunctionFrame {
	node: NodeTarget,
	statement: usize,
	function: Arc<BasicFunction>,
}

impl FunctionFrame {
	pub fn new(function: Arc<BasicFunction>, direction: Direction) -> Self {
		let node = function.component.endpoint(!direction);
		let statement = match direction {
			Direction::Advance => 0,
			Direction::Reverse => function[&node].statements.len(),
		};
		FunctionFrame { node, statement, function }
	}

	fn statement(&self) -> &Spanned<Statement> {
		let node = &self.function[&self.node];
		node.statements.get(self.statement).unwrap_or_else(||
			panic!("Statement index: {}, is invalid in node: {}, of length: {}",
				self.statement, self.node, node.statements.len()))
	}
}
