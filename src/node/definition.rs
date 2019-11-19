use std::sync::Arc;

use chashmap::CHashMap;

use crate::context::Context;
use crate::declaration::{DeclarationPath, FunctionPath, ModuleContext, StructurePath};
use crate::error::Diagnostic;
use crate::span::Spanned;

pub type NodeDefinitions = CHashMap<FunctionPath, Arc<Definition>>;

#[derive(Debug)]
pub struct Definition {
	pub structure: Spanned<StructurePath>,
	pub templates: Vec<Spanned<Arc<str>>>,
}

/// Loads all definitions from module contexts.
pub fn load_definitions(context: &Context) {
	for module in context.module_contexts.read().values() {
		context.emit(module_definitions(context, module));
	}
}

fn module_definitions(context: &Context, module: &ModuleContext) -> Result<(), Diagnostic> {
	module.definitions.iter().try_for_each(|definition| {
		let mut node_definition = crate::parser::definition(context, &definition.declaration)?;
		super::resolution::resolve_structure_path(context, module, &mut node_definition.structure)?;

		let node_definition = Arc::new(node_definition);
		let StructurePath(path) = &node_definition.structure.node;
		let module_path = path.module_path.clone().push(path.identifier.clone());

		Ok(for identifier in definition.methods.keys().cloned() {
			let module_path = module_path.clone();
			let function_path = FunctionPath(DeclarationPath { module_path, identifier });
			context.node_definitions.insert(function_path, node_definition.clone()).unwrap_none();
		})
	})
}
