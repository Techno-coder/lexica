use crate::interpreter::{ENTRY_POINT, Size};
use crate::node::{Function, Identifier};
use crate::source::{Span, Spanned};

use super::{Element, Evaluation, FunctionContext};

pub fn function_parameters<'a>(function: &Function<'a>, context: &mut FunctionContext<'a>) {
	for parameter in &function.parameters {
		let identifier = Spanned::new(parameter.target.clone(), parameter.span);
		let data_type = parameter.data_type.resolved().unwrap();
		context.register_variable(identifier, Size::parse(data_type)
			.expect("Invalid parameter type"));
	}
}

pub fn function_locals(function_span: Span, context: &FunctionContext) -> Vec<Spanned<Element>> {
	let mut elements = Vec::new();
	for local_size in context.local_sizes() {
		let annotation = Element::Other(format!("@local {}", local_size));
		elements.push(Spanned::new(annotation, function_span));
	}
	elements
}

pub fn function_header(function: &Spanned<Function>) -> Vec<Spanned<Element>> {
	let (mut elements, span) = (Vec::new(), function.identifier.span);
	elements.push(Spanned::new(Element::Other(format!("~{} {{", function.identifier)), span));
	elements.push(match function.identifier.node {
		Identifier(ENTRY_POINT) => instruction!(Reverse, "exit".to_owned(), function.span),
		_ => instruction!(Advance, Reverse, "return".to_owned(), function.span),
	});
	elements
}

pub fn function_arguments(function: &Function) -> Vec<Spanned<Element>> {
	let mut elements = Vec::new();
	for (parameter_index, parameter) in function.parameters.iter().enumerate() {
		let instruction = format!("restore {}", parameter_index);
		elements.push(instruction!(Advance, instruction, parameter.span));
	}
	elements
}

pub fn function_drops(context: &mut FunctionContext, return_value: &Evaluation) -> Vec<Spanned<Element>> {
	match return_value {
		Evaluation::Local(local) => super::drop_frame(context, &[*local]),
		_ => super::drop_frame(context, &[]),
	}
}

pub fn function_return(function: &Spanned<Function>, return_value: Evaluation) -> Vec<Spanned<Element>> {
	let mut elements = Vec::new();
	let return_span = function.return_value.span;
	elements.push(instruction!(Advance, match return_value {
		Evaluation::Unit => "pass".to_owned(),
		Evaluation::Local(local) => format!("drop {}", local),
		Evaluation::Immediate(primitive) => format!("drop.i {} {}", primitive.size(), primitive),
	}, return_span));

	elements.push(match function.identifier.node {
		Identifier(ENTRY_POINT) => instruction!(Advance, "exit".to_owned(), function.span),
		_ => instruction!(Advance, Advance, "return".to_owned(), function.span),
	});

	elements.push(Spanned::new(Element::Other("}".to_owned()), function.span));
	elements
}
