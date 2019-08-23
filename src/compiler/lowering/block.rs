use crate::basic;
use crate::node::{Block, Function, NodeConstruct};
use crate::source::Spanned;

use super::{Component, LowerTransform};

type ExpressionBlock<'a> = Spanned<crate::node::ExpressionBlock<'a>>;

pub fn expression_block<'a>(transform: &mut LowerTransform<'a>, expression_block: &mut ExpressionBlock<'a>) {
	expression_block.block.accept(transform);
	let component = transform.pop_component();
	expression_block.expression.accept(transform);

	let expression = transform.pop_component();
	let component = component.join(expression, expression_block.span);
	transform.push_component(component);
}


pub fn block<'a>(transform: &mut LowerTransform<'a>, block: &mut Spanned<Block<'a>>) {
	let mut component = Component::new_empty(transform.next_block());
	for statement in &mut block.statements {
		statement.accept(transform);
		let other = transform.pop_component();
		component = component.join(other, statement.span);
	}
	transform.push_component(component);
}

pub fn function<'a>(transform: &mut LowerTransform<'a>, function: &mut Spanned<Function<'a>>)
                    -> Spanned<basic::Function<'a>> {
	function.parameters.iter()
		.for_each(|parameter| transform.bind_variable(parameter.node.clone()));

	function.expression_block.accept(transform);
	let component = transform.pop_component();
	let (expression, other) = transform.pop_expression();
	let mut component = component.join(other, expression.span);

	let reverse_branch = Spanned::new(basic::Expression::Unit, function.span);
	component.blocks.get_mut(&component.reverse_block).unwrap()
		.reverse = Spanned::new(basic::Branch::Return(reverse_branch), function.span);
	component.blocks.get_mut(&component.advance_block).unwrap()
		.advance = Spanned::new(basic::Branch::Return(expression), function.span);
	Spanned::new(component.compress_function(function), function.span)
}
