use crate::*;

#[derive(Debug, Default)]
pub struct Context {
	pub errors: error::Errors,
	pub sources: source::Sources,
	pub source_keys: source::SourceKeys,
	pub module_contexts: declaration::ModuleContexts,
	pub declarations_function: declaration::DeclarationsFunction,
	pub declarations_structure: declaration::DeclarationsStructure,
	pub function_types: node::FunctionTypes,
	pub node_functions: node::NodeFunctions,
	pub node_structures: node::NodeStructures,
	pub node_definitions: node::NodeDefinitions,
	pub type_contexts: inference::TypeContexts,
	pub partial_functions: evaluation::PartialFunctions,
	pub basic_functions: basic::BasicFunctions,
}
