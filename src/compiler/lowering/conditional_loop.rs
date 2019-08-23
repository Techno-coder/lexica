use crate::basic;
use crate::node::NodeConstruct;
use crate::source::Spanned;

use super::{Component, LowerTransform};

type ConditionalLoop<'a> = Spanned<crate::node::ConditionalLoop<'a>>;

pub fn conditional_loop<'a>(transform: &mut LowerTransform<'a>, conditional_loop: &mut ConditionalLoop<'a>) {
	let (entry_target, exit_target) = (transform.next_block(), transform.next_block());
	let mut component = Component::new_paired(entry_target.clone(), exit_target.clone());
	transform.push_frame();

	conditional_loop.start_condition.as_mut().unwrap().accept(transform);
	let (start_condition, other) = transform.pop_expression();
	let mut start_component = transform.pop_component()
		.join(other, start_condition.span).invert();
	component.incorporate(&mut start_component);

	conditional_loop.end_condition.accept(transform);
	let (end_condition, other) = transform.pop_expression();
	let mut end_component = transform.pop_component()
		.join(other, end_condition.span);
	component.incorporate(&mut end_component);

	conditional_loop.block.accept(transform);
	let mut block_component = transform.pop_component();
	component.incorporate(&mut block_component);

	let span = conditional_loop.span;
	component.link_advance(&entry_target, &end_component.reverse_block, span);
	component.link_advance(&block_component.advance_block, &end_component.reverse_block, span);
	component.link_reverse(&end_component.reverse_block, &start_component.advance_block, span);
	component.conditional_advance(&end_component.advance_block, basic::ConditionalBranch {
		condition: end_condition,
		target: exit_target.clone(),
		default: block_component.reverse_block.clone(),
	}, span);

	component.link_reverse(&exit_target, &start_component.advance_block, span);
	component.link_reverse(&block_component.reverse_block, &start_component.advance_block, span);
	component.link_advance(&start_component.advance_block, &end_component.reverse_block, span);
	component.conditional_reverse(&start_component.reverse_block, basic::ConditionalBranch {
		condition: start_condition,
		target: entry_target.clone(),
		default: block_component.advance_block.clone(),
	}, span);

	component = component.join(transform.pop_frame(), span);
	transform.push_component(component);
}
