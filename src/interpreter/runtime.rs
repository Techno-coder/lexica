use super::{Context, Dependency, DependencyBuffer, Direction, ExecutionStep};

#[derive(Debug)]
pub struct RuntimeInterpreter<'a> {
	context: Context<'a>,
	evaluation_stack: Vec<(Dependency<'a>, DependencyBuffer<'a>)>,
}

impl<'a> RuntimeInterpreter<'a> {
	pub fn new(context: Context<'a>) -> Self {
		RuntimeInterpreter {
			context,
			evaluation_stack: Vec::new(),
		}
	}

	pub fn stack_dependency(&mut self, dependency: Dependency<'a>, step_direction: Direction) {
		let dependencies = match step_direction {
			Direction::Advance => dependency.node.dependencies(&mut self.context),
			Direction::Reverse => dependency.node.reverse_dependencies(&mut self.context),
		};
		self.evaluation_stack.push((dependency, DependencyBuffer::new(dependencies, step_direction)));
	}

	pub fn run(&mut self, direction: Direction) -> i64 {
		let mut last_value = 0;
		while !self.evaluation_stack.is_empty() {
			if let Some(ExecutionStep::Value(value)) = self.step(direction) {
				last_value = value;
			}
		}
		last_value
	}

	// TODO: THIS IS JUST AN ABSOLUTE MESS
	// 3 days spent on this

	pub fn step(&mut self, step_direction: Direction) -> Option<ExecutionStep> {
		let (dependency, child_dependencies) = self.evaluation_stack.last_mut()?;
		println!("{}\n", dependency.node);

		let execution_direction = Direction::compose(&dependency.direction, &step_direction);
		let child_dependency = match execution_direction {
			Direction::Advance => child_dependencies.advance(),
			Direction::Reverse => child_dependencies.reverse(),
		};

		if let Some(mut child) = child_dependency.cloned() {
			if execution_direction == Direction::Reverse {
				child.direction = child.direction.invert();
			}
			self.stack_dependency(child, step_direction);
		} else {
			let execution_step = match execution_direction {
				Direction::Advance => dependency.node.execute(&mut self.context),
				Direction::Reverse => dependency.node.reverse(&mut self.context),
			}.expect("Error occurred in step");

			match execution_step {
				ExecutionStep::Void => {
					self.evaluation_stack.pop();
				}
				ExecutionStep::Repeat => {
					let new_dependencies = match execution_direction {
						Direction::Advance => dependency.node.dependencies(&mut self.context),
						Direction::Reverse => dependency.node.reverse_dependencies(&mut self.context),
					};
					::std::mem::replace(child_dependencies, DependencyBuffer::new(new_dependencies, step_direction));
				}
				ExecutionStep::Value(value) => {
					self.context.cache_evaluation(dependency.node, value);
					self.evaluation_stack.pop();
					return Some(ExecutionStep::Value(value));
				}
			}
		}
		Some(ExecutionStep::Void)
	}

//	pub fn step(&mut self, direction: Direction) -> Option<ExecutionStep> {
//		let (step_direction, dependency, child_dependencies) = self.evaluation_stack.last_mut()?;
//		println!("{}\n", dependency.node);
//
//		let child_dependency = match Direction::compose(step_direction, &direction) {
//			Direction::Advance => child_dependencies.advance(),
//			Direction::Reverse => child_dependencies.reverse(),
//		};
//
//		if let Some(mut child) = child_dependency.cloned() {
//			child.direction = Direction::compose(step_direction, &direction);
//			self.stack_dependency(child, direction);
//		} else {
//			let execution_step = match dependency.direction {
//				Direction::Advance => dependency.node.execute(&mut self.context),
//				Direction::Reverse => dependency.node.reverse(&mut self.context),
//			}.expect("Error occurred in step");
//
//			match execution_step {
//				ExecutionStep::Void => {
//					self.evaluation_stack.pop();
//				}
//				ExecutionStep::Repeat => {
//					let new_dependencies = match dependency.direction {
//						Direction::Advance => dependency.node.dependencies(&mut self.context),
//						Direction::Reverse => dependency.node.reverse_dependencies(&mut self.context),
//					};
//					::std::mem::replace(child_dependencies, DependencyBuffer::new(new_dependencies));
//				}
//				ExecutionStep::Value(value) => {
//					self.context.cache_evaluation(dependency.node, value);
//					self.evaluation_stack.pop();
//					return Some(ExecutionStep::Value(value));
//				}
//			}
//		}
//		Some(ExecutionStep::Void)
//	}

//	pub fn step(&mut self, direction: Direction) -> Option<ExecutionStep> {
//		let (dependency, child_dependencies) = self.evaluation_stack.last_mut()?;
//		let child_dependency = match direction {
//			Direction::Advance => child_dependencies.advance(),
//			Direction::Reverse => child_dependencies.reverse(),
//		};
//
//		if let Some(child) = child_dependency.cloned() {
//			self.stack_dependency(child);
//		} else {
//			let execution_step = match (dependency.direction, direction) {
//				(Direction::Advance, Direction::Advance) => dependency.node.execute(&mut self.context),
//				(Direction::Reverse, Direction::Reverse) => dependency.node.reverse(&mut self.context),
//				_ => Ok(ExecutionStep::Void),
//			}.expect("Error occurred in step");
//
//			match execution_step {
//				ExecutionStep::Void => {
//					self.evaluation_stack.pop();
//				}
//				ExecutionStep::Repeat => {
//					let new_dependencies = match dependency.direction {
//						Direction::Advance => dependency.node.dependencies(&mut self.context),
//						Direction::Reverse => dependency.node.reverse_dependencies(&mut self.context),
//					};
//					::std::mem::replace(child_dependencies, DependencyBuffer::new(new_dependencies));
//				}
//				ExecutionStep::Value(value) => {
//					self.context.cache_evaluation(dependency.node, value);
//					self.evaluation_stack.pop();
//					return Some(ExecutionStep::Value(value));
//				}
//			}
//		}
//		Some(ExecutionStep::Void)
//	}
}

// TODO: Evaluation frame
