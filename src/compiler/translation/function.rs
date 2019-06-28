use crate::compiler::translation::Element;
use crate::interpreter::Size;
use crate::node::Function;
use crate::source::{Span, Spanned};

use super::FunctionContext;

pub fn function_parameters<'a>(function: &Function<'a>, context: &mut FunctionContext<'a>) {
	for parameter in &function.parameters {
		let parameter = Spanned::new(parameter.identifier.clone(), parameter.span);
		context.register_variable(parameter, Size::Unsigned64); // TODO: Parse parameter type
	}
}

pub fn function_locals(function_span: Span, context: &FunctionContext) -> Vec<Spanned<Element>> {
	let mut elements = Vec::new();
	for local_size in context.local_sizes() {
		let annotation = match local_size {
			Size::Boolean => format!("@local {} false", local_size),
			_ => format!("@local {} 0", local_size),
		};
		elements.push(Spanned::new(Element::Other(annotation), function_span));
	}
	elements
}

pub fn function_header(function: &Spanned<Function>) -> Vec<Spanned<Element>> {
	let (mut elements, span) = (Vec::new(), function.identifier.span);
	elements.push(Spanned::new(Element::Other(format!("~{} {{", function.identifier)), span));
	elements.push(instruction!(Advance, Reverse, "return".to_owned(), function.span));
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

pub fn function_drops(context: &FunctionContext, return_index: usize) -> Vec<Spanned<Element>> {
	let mut elements = Vec::new();
	for (_, (identifier_index, span)) in context.identifier_table() {
		if *identifier_index == return_index {
			continue;
		}

		let instruction = format!("drop {}", identifier_index);
		elements.push(instruction!(Advance, instruction, *span));
	}
	elements
}

pub fn function_return(function: &Spanned<Function>, return_index: usize) -> Vec<Spanned<Element>> {
	let mut elements = Vec::new();
	let return_span = function.return_value.span;
	elements.push(instruction!(Advance, format!("drop {}", return_index), return_span));
	elements.push(instruction!(Advance, Advance, "return".to_owned(), return_span));
	elements.push(Spanned::new(Element::Other("}".to_owned()), function.span));
	elements
}
