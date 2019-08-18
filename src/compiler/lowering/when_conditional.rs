use crate::basic;
use crate::node::{DataType, NodeConstruct, Variable};
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
		let mut condition_component = transform.pop_component().join(other);
		component.incorporate(&mut condition_component);

		branch.end_condition.as_mut().unwrap().accept(transform);
		let (end_condition, other) = transform.pop_expression();
		let mut end_condition_component = transform.pop_component().join(other).invert();
		component.incorporate(&mut end_condition_component);

		branch.expression_block.accept(transform);
		let (expression, other) = transform.pop_expression();
		let mut block_component = transform.pop_component().join(other);

		let data_type = expression.data_type();
		if data_type != DataType::UNIT {
			let expression_span = expression.span;
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
		component.link_advance(&block_component.advance_block, &exit_target);
		component.link_reverse(&block_component.reverse_block, &entry_target);

		let (target, default) = (block_component.reverse_block, basic::BlockTarget::SENTINEL);
		component[&target].in_advance.push(condition_component.advance_block.clone());
		let branch = basic::ConditionalBranch { condition, target, default };
		let branch = basic::Branch::Conditional(branch);
		component[&condition_component.advance_block].advance = branch;

		let (target, default) = (block_component.advance_block, basic::BlockTarget::SENTINEL);
		component[&target].in_reverse.push(end_condition_component.reverse_block.clone());
		let branch = basic::ConditionalBranch { condition: end_condition, target, default };
		let branch = basic::Branch::Conditional(branch);
		component[&end_condition_component.reverse_block].reverse = branch;

		component.link_reverse(&condition_component.reverse_block,
			last_condition.as_ref().unwrap_or(&entry_target));
		component.link_advance(&end_condition_component.advance_block,
			last_end_condition.as_ref().unwrap_or(&exit_target));

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
				component.link_advance(&entry_target, &condition_component.reverse_block);
				component.link_reverse(&exit_target, &end_condition_component.advance_block);
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
