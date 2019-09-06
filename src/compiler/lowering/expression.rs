use crate::basic;
use crate::node::{BinaryOperation, Expression, ExpressionNode, FunctionCall, NodeConstruct};
use crate::source::Spanned;

use super::{Component, LowerTransform};

pub fn expression<'a>(transform: &mut LowerTransform<'a>, expression: &mut Spanned<ExpressionNode<'a>>) {
	let expression_span = expression.span;
	let expression = Spanned::new(match expression.node.expression.as_mut() {
		Expression::Unit => basic::Expression::Unit,
		Expression::Primitive(primitive) => basic::Expression::Primitive(primitive.clone()),
		Expression::Variable(target) => {
			let mut variable = transform.get_binding(&target.root()).clone();
			variable.data_type = expression.node.evaluation_type.clone();
			variable.target = target.clone();
			basic::Expression::Variable(variable)
		}
		Expression::BinaryOperation(binary_operation) => return binary_operation.accept(transform),
		Expression::WhenConditional(when_conditional) => return when_conditional.accept(transform),
		Expression::ExpressionBlock(expression_block) => return expression_block.accept(transform),
		Expression::FunctionCall(function_call) => return function_call.accept(transform),
		Expression::Accessor(accessor) => return accessor.accept(transform),
	}, expression_span);

	let next_block = transform.next_block();
	transform.push_component(Component::new_empty(next_block));
	transform.push_evaluation(basic::Value::Expression(expression));
}

pub fn binary_operation<'a>(transform: &mut LowerTransform<'a>, operation: &mut Spanned<BinaryOperation<'a>>) {
	operation.left.accept(transform);
	let (left, other) = transform.pop_expression();
	let component = transform.pop_component().join(other, left.span);

	operation.right.accept(transform);
	let (right, other) = transform.pop_expression();
	let other = transform.pop_component().join(other, right.span);
	let component = component.join(other, right.span);
	transform.push_component(component);

	let (span, operator) = (operation.span, operation.operator.clone());
	let operation = basic::BinaryOperation { left, right, operator };
	let operation = Spanned::new(operation, span);

	let value = basic::Value::BinaryOperation(operation);
	transform.push_evaluation(value);
}

pub fn function_call<'a>(transform: &mut LowerTransform<'a>, function_call: &mut Spanned<FunctionCall<'a>>) {
	let mut arguments = Vec::new();
	let mut component = Component::new_empty(transform.next_block());
	for argument in &mut function_call.arguments {
		argument.accept(transform);
		let (expression, other) = transform.pop_expression();
		let other = transform.pop_component().join(other, expression.span);
		component = component.join(other, expression.span);
		arguments.push(expression);
	}

	let span = function_call.span;
	let function = function_call.function.clone();
	let evaluation_type = function_call.evaluation_type.clone();
	let function_call = basic::FunctionCall { function, arguments, evaluation_type };

	let function_call = Spanned::new(function_call, span);
	let value = basic::Value::FunctionCall(function_call);
	transform.push_component(component);
	transform.push_evaluation(value);
}
