use crate::intrinsics::IntrinsicStore;
use crate::node::*;
use crate::source::Spanned;

use super::{Element, Evaluation, FunctionContext};

#[derive(Debug)]
pub struct Translator<'a, 'b> {
	context: FunctionContext<'a>,
	// TODO: Replace with any entropic function
	intrinsics: &'b IntrinsicStore,
}

impl<'a, 'b> Translator<'a, 'b> {
	pub fn new(intrinsics: &'b IntrinsicStore) -> Self {
		Self { context: FunctionContext::default(), intrinsics }
	}
}

impl<'a, 'b> NodeVisitor<'a> for Translator<'a, 'b> {
	type Result = Vec<Spanned<Element>>;

	fn syntax_unit(&mut self, syntax_unit: &mut Spanned<SyntaxUnit<'a>>) -> Self::Result {
		syntax_unit.functions.iter_mut()
			.flat_map(|(_, function)| {
				self.context.push_frame();
				let elements = function.accept(self);
				self.context = FunctionContext::default();
				elements
			}).collect()
	}

	fn function(&mut self, function: &mut Spanned<Function<'a>>) -> Self::Result {
		super::function_parameters(function, &mut self.context);
		let mut function_elements = function.expression_block.accept(self);

		let mut elements = super::function_locals(function.span, &self.context);
		elements.append(&mut super::function_header(function));
		elements.append(&mut super::function_arguments(function));
		elements.append(&mut function_elements);

		let return_value = self.context.pop_evaluation();
		elements.append(&mut super::function_drops(&mut self.context, &return_value));
		elements.append(&mut super::function_return(function, return_value));
		elements
	}

	fn expression(&mut self, expression: &mut Spanned<ExpressionNode<'a>>) -> Self::Result {
		match expression.node.as_mut() {
			Expression::Unit => self.context.push_evaluation(Evaluation::Unit),
			Expression::Variable(variable) => {
				let variable = self.context.get_variable(variable);
				self.context.push_evaluation(Evaluation::Local(variable));
			}
			Expression::Primitive(primitive) => {
				let primitive = Spanned::new(primitive.clone(), expression.span);
				self.context.push_evaluation(Evaluation::Immediate(primitive));
			}
			Expression::BinaryOperation(binary_operation) => return binary_operation.accept(self),
			Expression::WhenConditional(when_conditional) => return when_conditional.accept(self),
			Expression::ExpressionBlock(expression_block) => return expression_block.accept(self),
			Expression::FunctionCall(function_call) => return function_call.accept(self),
		}
		Vec::new()
	}

	fn expression_block(&mut self, expression_block: &mut Spanned<ExpressionBlock<'a>>) -> Self::Result {
		let mut elements = expression_block.block.accept(self);
		elements.append(&mut expression_block.expression.accept(self));
		elements
	}

	fn block(&mut self, block: &mut Spanned<Block<'a>>) -> Self::Result {
		block.statements.iter_mut().flat_map(|statement| statement.accept(self)).collect()
	}

	fn binary_operation(&mut self, operation: &mut Spanned<BinaryOperation<'a>>) -> Self::Result {
		let mut elements = operation.left.accept(self);
		elements.append(&mut operation.right.accept(self));
		elements.append(&mut super::binary_operation(operation, &mut self.context));
		elements
	}

	fn binding(&mut self, binding: &mut Spanned<Binding<'a>>) -> Self::Result {
		let mut elements = binding.expression.accept(self);
		let target = binding.variable.target.clone();
		let target = Spanned::new(target, binding.variable.span);
		let local_index = self.context.pop_evaluation().promote(&mut elements, &mut self.context);
		self.context.annotate_local(local_index, target);
		elements
	}

	fn conditional_loop(&mut self, conditional_loop: &mut Spanned<ConditionalLoop<'a>>) -> Self::Result {
		self.context.push_frame();
		let (start_label, end_label) = self.context.pair_labels();
		let mut elements = super::loop_header(conditional_loop.span, start_label, end_label);

		let end_condition = &mut conditional_loop.end_condition;
		let condition = end_condition.accept(self);
		elements.append(&mut super::loop_end_condition(condition, &mut self.context, end_condition, end_label));

		elements.append(&mut conditional_loop.block.accept(self));
		elements.append(&mut super::drop_frame(&mut self.context, &[]));

		let start_condition = conditional_loop.start_condition.as_mut().unwrap();
		let condition = start_condition.accept(self);
		elements.append(&mut super::loop_start_condition(condition, &mut self.context, start_condition, start_label));
		elements.append(&mut super::loop_suffix(conditional_loop.span, start_label, end_label));
		elements
	}

	fn explicit_drop(&mut self, explicit_drop: &mut Spanned<ExplicitDrop<'a>>) -> Self::Result {
		let mut elements = explicit_drop.expression.accept(self);
		super::polarize_reverse(&mut elements);

		let local_index = self.context.drop_variable(&explicit_drop.target);
		let instruction = match self.context.pop_evaluation() {
			Evaluation::Unit => panic!("Unit evaluation cannot be assigned"),
			Evaluation::Local(local) => format!("clone {} {}", local_index, local),
			Evaluation::Immediate(primitive) => format!("reset {} {}", local_index, primitive),
		};

		elements.insert(0, instruction!(Advance, Reverse, instruction, explicit_drop.span));
		elements
	}

	fn function_call(&mut self, function_call: &mut Spanned<FunctionCall<'a>>) -> Self::Result {
		let mut elements: Vec<_> = function_call.arguments.iter_mut()
			.flat_map(|argument| argument.accept(self)).collect();
		elements.append(&mut super::function_call_arguments(function_call, &mut self.context));
		elements.append(&mut super::function_call_value(function_call, &mut self.context, self.intrinsics));
		elements
	}

	fn mutation(&mut self, mutation: &mut Spanned<Mutation<'a>>) -> Self::Result {
		let span = mutation.span;
		match &mut mutation.node {
			Mutation::Swap(left, right) => super::swap(span, left, right, &self.context),
			Mutation::AddAssign(target, expression) => {
				let expression = expression.accept(self);
				super::add_assign(span, target, expression, &mut self.context)
			}
			Mutation::MinusAssign(target, expression) => {
				let expression = expression.accept(self);
				super::minus_assign(span, target, expression, &mut self.context)
			}
			Mutation::MultiplyAssign(target, expression) => {
				let expression = expression.accept(self);
				super::multiply_assign(span, target, expression, &mut self.context)
			}
		}
	}

	fn when_conditional(&mut self, when_conditional: &mut Spanned<WhenConditional<'a>>) -> Self::Result {
		let mut elements = Vec::new();
		let (start_label, end_label) = self.context.pair_labels();
		let branch_labels = super::when_branch_labels(when_conditional, &mut self.context);

		let conditions = when_conditional.branches.iter_mut()
			.map(|branch| branch.condition.accept(self)).collect();
		elements.append(&mut super::when_entry(when_conditional, &branch_labels,
			conditions, &mut self.context, start_label, end_label));

		let expressions = when_conditional.branches.iter_mut()
			.map(|branch| branch.expression_block.accept(self)).collect();
		elements.append(&mut super::when_expressions(when_conditional, &branch_labels,
			expressions, start_label, end_label));

		let end_conditions = when_conditional.branches.iter_mut()
			.map(|branch| branch.condition.accept(self)).collect();
		elements.append(&mut super::when_reverse_entry(when_conditional, &branch_labels,
			end_conditions, &mut self.context, start_label, end_label));
		elements
	}
}
