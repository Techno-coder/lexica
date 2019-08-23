use crate::basic;
use crate::node::{NodeConstruct, Variable};
use crate::source::Spanned;

use super::{Component, LowerTransform};

type WhenConditional<'a> = Spanned<crate::node::WhenConditional<'a>>;

pub fn when_conditional<'a>(transform: &mut LowerTransform<'a>, when_conditional: &mut WhenConditional<'a>) {
	let (entry_target, exit_target) = (transform.next_block(), transform.next_block());
	let mut component = Component::new_paired(entry_target.clone(), exit_target.clone());
	let mut temporary: Option<Variable> = None;

	let mut last_condition: Option<basic::BlockTarget> = None;
	let mut last_end_condition: Option<basic::BlockTarget> = None;
	for branch in when_conditional.branches.iter_mut() {
		branch.condition.accept(transform);
		let (condition, other) = transform.pop_expression();
		let condition_span = condition.span;
		let mut condition_component = transform.pop_component()
			.join(other, condition_span);
		component.incorporate(&mut condition_component);

		branch.end_condition.as_mut().unwrap().accept(transform);
		let (end_condition, other) = transform.pop_expression();
		let end_condition_span = condition.span;
		let mut end_condition_component = transform.pop_component()
			.join(other, condition.span).invert();
		component.incorporate(&mut end_condition_component);

		branch.expression_block.accept(transform);
		let (expression, other) = transform.pop_expression();
		let expression_span = expression.span;
		let mut block_component = transform.pop_component()
			.join(other, expression_span);

		if !expression.is_unit() {
			let data_type = expression.data_type();
			let variable = temporary.get_or_insert_with(|| {
				let target = transform.next_temporary();
				Variable { target, data_type, is_mutable: false }
			});

			let target = Spanned::new(variable.target.clone(), expression_span);
			let assignment = basic::Assignment { target, expression };
			let assignment = Spanned::new(assignment, expression_span);
			let statement = basic::Statement::Assignment(assignment);
			let statement = Spanned::new(statement, expression_span);
			block_component = block_component.append(transform.next_block(), statement);
		}

		component.incorporate(&mut block_component);
		component.link_advance(&block_component.advance_block, &exit_target, expression_span);
		component.link_reverse(&block_component.reverse_block, &entry_target, expression_span);

		let (target, default) = (block_component.reverse_block, basic::BlockTarget::SENTINEL);
		component[&target].in_advance.push(condition_component.advance_block.clone());
		let basic_branch = basic::ConditionalBranch { condition, target, default };
		let basic_branch = basic::Branch::Conditional(basic_branch);
		let basic_branch = Spanned::new(basic_branch, branch.condition.span);
		component[&condition_component.advance_block].advance = basic_branch;

		let (target, default) = (block_component.advance_block, basic::BlockTarget::SENTINEL);
		component[&target].in_reverse.push(end_condition_component.reverse_block.clone());
		let basic_branch = basic::ConditionalBranch { condition: end_condition, target, default };
		let basic_branch = basic::Branch::Conditional(basic_branch);
		let basic_branch = Spanned::new(basic_branch, branch.end_condition.as_ref().unwrap().span);
		component[&end_condition_component.reverse_block].reverse = basic_branch;

		component.link_reverse(&condition_component.reverse_block,
			last_condition.as_ref().unwrap_or(&entry_target), condition_span);
		component.link_advance(&end_condition_component.advance_block,
			last_end_condition.as_ref().unwrap_or(&exit_target), end_condition_span);

		match &last_condition {
			Some(last_condition) => {
				let default_target = condition_component.reverse_block;
				component[&default_target].in_advance.push(last_condition.clone());
				let mapping = map! { basic::BlockTarget::SENTINEL => default_target };
				component[last_condition].advance.replace(&mapping);

				let last_end_condition = last_end_condition.as_ref().unwrap();
				let default_target = end_condition_component.advance_block;
				component[&default_target].in_reverse.push(last_end_condition.clone());
				let mapping = map! { basic::BlockTarget::SENTINEL => default_target };
				component[last_end_condition].reverse.replace(&mapping);
			}
			None => {
				let reverse_block = condition_component.reverse_block;
				component.link_advance(&entry_target, &reverse_block, condition_span);
				let advance_block = end_condition_component.advance_block;
				component.link_reverse(&exit_target, &advance_block, end_condition_span);
			}
		}

		last_condition = Some(condition_component.advance_block);
		last_end_condition = Some(end_condition_component.reverse_block);
	}

	let mapping = map! { basic::BlockTarget::SENTINEL => exit_target };
	component[&last_condition.unwrap()].advance.replace(&mapping);
	let mapping = map! { basic::BlockTarget::SENTINEL => entry_target.clone() };
	component[&last_end_condition.unwrap()].reverse.replace(&mapping);

	let span = when_conditional.span;
	let expression = Spanned::new(match temporary {
		Some(temporary) => {
			let variable = Spanned::new(temporary.clone(), span);
			let value = basic::Value::Uninitialized(span);
			let binding = basic::Binding { variable, value };
			let binding = Spanned::new(binding, span);

			let statement = basic::Statement::Binding(binding);
			let statement = Spanned::new(statement, span);
			component[&entry_target].statements.push(statement);
			basic::Expression::Variable(temporary)
		}
		None => basic::Expression::Unit,
	}, span);

	let value = basic::Value::Expression(expression);
	transform.push_component(component);
	transform.push_evaluation(value);
}
