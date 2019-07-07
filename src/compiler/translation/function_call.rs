use crate::interpreter::Size;
use crate::intrinsics::IntrinsicStore;
use crate::node::{FunctionCall, Identifier};
use crate::source::Spanned;

use super::{Element, Evaluation, FunctionContext};

pub fn function_call_value(function_call: &Spanned<&mut FunctionCall>, context: &mut FunctionContext,
                           intrinsics: &IntrinsicStore) -> Vec<Spanned<Element>> {
	let instruction = format!("call {}", function_call.function);
	let Identifier(identifier) = &function_call.function.node;
	let mut elements = vec![match intrinsics.get(identifier) {
		Some(_) => instruction!(Advance, Advance, instruction, function_call.span),
		None => instruction!(Advance, instruction, function_call.span),
	}];

	let return_type = function_call.evaluation_type.resolved().unwrap();
	let local = context.register_local(Size::parse(return_type)
		.expect("Invalid return type"));

	elements.push(instruction!(Advance, format!("restore {}", local), function_call.span));
	context.push_evaluation(Evaluation::Local(local));
	elements
}

pub fn function_call_arguments(function_call: &FunctionCall, context: &mut FunctionContext)
                               -> Vec<Spanned<Element>> {
	function_call.arguments.iter().rev()
		.map(|argument| {
			let evaluation = context.pop_evaluation();
			function_call_argument(Spanned::new(evaluation, argument.span))
		}).collect()
}

pub fn function_call_argument(argument: Spanned<Evaluation>) -> Spanned<Element> {
	match argument.node {
		Evaluation::Local(local) => {
			let instruction = format!("drop {}", local);
			instruction!(Advance, instruction, argument.span)
		}
		Evaluation::Immediate(primitive) => {
			let instruction = format!("drop.i {} {}", primitive.size(), primitive);
			instruction!(Advance, instruction, primitive.span)
		}
	}
}
