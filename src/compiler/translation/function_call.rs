use crate::interpreter::Size;
use crate::intrinsics::IntrinsicStore;
use crate::node::{DataType, FunctionCall, Identifier};
use crate::source::Spanned;

use super::{Element, Evaluation, FunctionContext};

pub fn function_call_value(function_call: &mut Spanned<FunctionCall>, context: &mut FunctionContext,
                           intrinsics: &IntrinsicStore) -> Vec<Spanned<Element>> {
	let instruction = format!("call {}", function_call.function);
	let Identifier(identifier) = &function_call.function.node;
	let mut elements = vec![match intrinsics.get(identifier) {
		Some(_) => instruction!(Advance, Advance, instruction, function_call.span),
		None => instruction!(Advance, instruction, function_call.span),
	}];

	match function_call.evaluation_type == DataType::UNIT_TYPE {
		true => context.push_evaluation(Evaluation::Unit),
		false => {
			let return_type = function_call.evaluation_type.resolved().unwrap();
			let local = context.register_local(Size::parse(return_type)
				.expect("Invalid return type"));

			elements.push(instruction!(Advance, format!("restore {}", local), function_call.span));
			context.register_intermediate(local, function_call.span);
			context.push_evaluation(Evaluation::Local(local));
		}
	}

	elements
}

pub fn function_call_arguments(function_call: &FunctionCall, context: &mut FunctionContext)
                               -> Vec<Spanned<Element>> {
	function_call.arguments.iter().rev()
		.map(|argument| {
			let evaluation = context.pop_evaluation();
			let argument = Spanned::new(evaluation, argument.span);
			function_call_argument(argument, context)
		}).collect()
}

pub fn function_call_argument(argument: Spanned<Evaluation>, context: &mut FunctionContext)
                              -> Spanned<Element> {
	match argument.node {
		Evaluation::Unit => Spanned::new(Element::Other("".to_owned()), argument.span),
		Evaluation::Local(local) => {
			// TODO: Assume all arguments are consuming
			context.drop_intermediate(&local);
			let instruction = format!("drop {}", local);
			instruction!(Advance, instruction, argument.span)
		}
		Evaluation::Immediate(primitive) => {
			let instruction = format!("drop.i {} {}", primitive.size(), primitive);
			instruction!(Advance, instruction, primitive.span)
		}
	}
}
