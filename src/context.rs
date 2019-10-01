use crate::*;

#[derive(Debug, Default)]
pub struct Context {
	pub errors: error::Errors,
	pub sources: source::SourceMap,
	pub source_keys: source::SourceKeyMap,

	pub modules_pending: declaration::ModulesPending,
	pub declarations_module: declaration::DeclarationsModule,
	pub declarations_function: declaration::DeclarationsFunction,
	pub declarations_structure: declaration::DeclarationsStructure,

	pub node_functions: node::NodeFunctions,
}
