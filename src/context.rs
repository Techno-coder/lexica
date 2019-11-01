use crate::*;

#[derive(Debug, Default)]
pub struct Context {
	pub errors: error::Errors,
	pub sources: source::Sources,
	pub source_keys: source::SourceKeys,

	pub modules_pending: declaration::ModulesPending,
	pub module_contexts: declaration::ModuleContexts,
	pub declarations_function: declaration::DeclarationsFunction,
	pub declarations_structure: declaration::DeclarationsStructure,

	pub function_types: node::FunctionTypes,
	pub node_functions: node::NodeFunctions,
	pub node_structures: node::NodeStructures,

	pub basic_functions: basic::BasicFunctions,
}
