use crate::interpreter::Size;
use crate::node::FunctionCall;
use crate::source::Spanned;

use super::{Element, Evaluation, FunctionContext};

pub fn function_call_value(function_call: &Spanned<&mut FunctionCall>, context: &mut FunctionContext)
                           -> Vec<Spanned<Element>> {
	let mut elements = Vec::new();
	let instruction = format!("call {}", function_call.function);
	elements.push(instruction!(Advance, instruction, function_call.span));

	// TODO: Parse function return size
	let local = context.register_local(Size::Unsigned64);
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
