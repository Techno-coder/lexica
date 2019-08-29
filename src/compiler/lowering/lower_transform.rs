use hashbrown::HashMap;

use crate::basic;
use crate::node::*;
use crate::source::Spanned;

use super::Component;

type BindingFrame<'a> = HashMap<VariableTarget<'a>, Spanned<Variable<'a>>>;

#[derive(Debug, Default)]
pub struct LowerTransform<'a> {
	functions: Vec<Spanned<basic::Function<'a>>>,
	bindings: Vec<BindingFrame<'a>>,

	evaluation_stack: Vec<basic::Value<'a>>,
	component_stack: Vec<Component<'a>>,
	next_temporary: usize,
	next_block: usize,
}

impl<'a> LowerTransform<'a> {
	pub fn next_temporary(&mut self) -> VariableTarget<'a> {
		self.next_temporary += 1;
		VariableTarget(Identifier::TEMPORARY_LOWER, self.next_temporary - 1)
	}

	pub fn next_block(&mut self) -> basic::BlockTarget {
		self.next_block += 1;
		basic::BlockTarget(self.next_block - 1)
	}

	pub fn push_evaluation(&mut self, evaluation: basic::Value<'a>) {
		self.evaluation_stack.push(evaluation);
	}

	pub fn pop_evaluation(&mut self) -> basic::Value<'a> {
		self.evaluation_stack.pop().expect("Evaluation stack is empty")
	}

	pub fn pop_expression(&mut self) -> (Spanned<basic::Expression<'a>>, Component<'a>) {
		let value = self.pop_evaluation();
		if let basic::Value::Expression(expression) = value {
			return (expression, Component::new_empty(self.next_block()));
		}

		let span = value.span();
		let (target, data_type) = (self.next_temporary(), value.data_type());
		let binding_variable = Variable { target, data_type, is_mutable: false };
		let variable = Spanned::new(binding_variable.clone(), span);

		let binding = Spanned::new(basic::Binding { variable, value }, span);
		let statement = Spanned::new(basic::Statement::Binding(binding), span);
		let block = basic::BasicBlock::new_single(statement);

		(Spanned::new(basic::Expression::Variable(binding_variable), span),
			Component::new_single(self.next_block(), block))
	}

	pub fn push_component(&mut self, component: Component<'a>) {
		self.component_stack.push(component);
	}

	pub fn pop_component(&mut self) -> Component<'a> {
		self.component_stack.pop().expect("Component stack is empty")
	}

	pub fn push_frame(&mut self) {
		self.bindings.push(BindingFrame::new());
	}

	pub fn pop_frame(&mut self) -> Component<'a> {
		let mut component = Component::new_empty(self.next_block());
		let frame = self.bindings.pop().expect("Binding frame stack is empty");
		for (_, variable) in frame {
			let span = variable.span;
			let target = Spanned::new(variable.node.target, span);
			let implicit_drop = Spanned::new(basic::ImplicitDrop { target }, span);
			let statement = basic::Statement::ImplicitDrop(implicit_drop);
			component = component.append(self.next_block(), Spanned::new(statement, span));
		}
		component
	}

	pub fn bind_variable(&mut self, variable: Spanned<Variable<'a>>) {
		self.bindings.last_mut().expect("Binding frame stack is empty")
			.insert(variable.target.clone(), variable);
	}

	pub fn drop_binding(&mut self, target: &VariableTarget<'a>) {
		for frame in self.bindings.iter_mut().rev() {
			frame.remove(target);
		}
	}

	pub fn get_binding(&self, target: &VariableTarget<'a>) -> &Variable<'a> {
		let error = format!("Binding for: {}, does not exist", target);
		self.bindings.iter().rev().find_map(|frame| frame.get(target)).expect(&error)
	}

	pub fn functions(self) -> Vec<Spanned<basic::Function<'a>>> {
		self.functions
	}
}

impl<'a> NodeVisitor<'a> for LowerTransform<'a> {
	type Result = ();

	fn syntax_unit(&mut self, syntax_unit: &mut Spanned<SyntaxUnit<'a>>) -> Self::Result {
		syntax_unit.functions.values_mut().for_each(|function| function.accept(self));
	}

	fn function(&mut self, function: &mut Spanned<Function<'a>>) -> Self::Result {
		self.bindings = vec![BindingFrame::new()];
		self.next_temporary = 0;
		self.next_block = 0;

		let function = super::function(self, function);
		self.functions.push(function);

		assert!(self.evaluation_stack.is_empty());
		assert!(self.component_stack.is_empty());
	}

	fn expression(&mut self, expression: &mut Spanned<ExpressionNode<'a>>) -> Self::Result {
		super::expression(self, expression);
	}

	fn expression_block(&mut self, expression_block: &mut Spanned<ExpressionBlock<'a>>) -> Self::Result {
		super::expression_block(self, expression_block);
	}

	fn block(&mut self, block: &mut Spanned<Block<'a>>) -> Self::Result {
		super::block(self, block);
	}

	fn statement(&mut self, statement: &mut Spanned<Statement<'a>>) -> Self::Result where Self: Sized {
		super::statement(self, statement);
	}

	fn binary_operation(&mut self, operation: &mut Spanned<BinaryOperation<'a>>) -> Self::Result {
		super::binary_operation(self, operation);
	}

	fn binding(&mut self, binding: &mut Spanned<Binding<'a>>) -> Self::Result {
		super::binding(self, binding);
	}

	fn conditional_loop(&mut self, conditional_loop: &mut Spanned<ConditionalLoop<'a>>) -> Self::Result {
		super::conditional_loop(self, conditional_loop);
	}

	fn explicit_drop(&mut self, explicit_drop: &mut Spanned<ExplicitDrop<'a>>) -> Self::Result {
		super::explicit_drop(self, explicit_drop);
	}

	fn function_call(&mut self, function_call: &mut Spanned<FunctionCall<'a>>) -> Self::Result {
		super::function_call(self, function_call);
	}

	fn mutation(&mut self, mutation: &mut Spanned<Mutation<'a>>) -> Self::Result {
		super::mutation(self, mutation);
	}

	fn when_conditional(&mut self, when_conditional: &mut Spanned<WhenConditional<'a>>) -> Self::Result {
		super::when_conditional(self, when_conditional);
	}
}
