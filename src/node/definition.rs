use std::sync::Arc;

use chashmap::CHashMap;

use crate::context::Context;
use crate::declaration::{DeclarationPath, FunctionPath, ModuleContext, ModulePath, StructurePath};
use crate::error::Diagnostic;
use crate::span::Spanned;

use super::NodeError;

pub type NodeDefinitions = CHashMap<Arc<FunctionPath>, Arc<Definition>>;

#[derive(Debug)]
pub struct Definition {
	pub declaration: Arc<ModulePath>,
	pub structure: Spanned<StructurePath>,
	pub templates: Vec<Spanned<Arc<str>>>,
}

/// Loads all definitions from module contexts.
pub fn load_definitions(context: &Context) {
	for (module_path, module) in context.module_contexts.read().iter() {
		context.emit(module_definitions(context, module_path.clone(), module));
	}
}

fn module_definitions(context: &Context, module_path: Arc<ModulePath>,
                      module: &ModuleContext) -> Result<(), Diagnostic> {
	module.definitions.iter().try_for_each(|definition| {
		let mut node_definition = crate::parser::definition(context,
			module_path.clone(), &definition.declaration)?;
		super::resolution::resolve_structure_path(context,
			module, &mut node_definition.structure)?;

		let node_definition = Arc::new(node_definition);
		let StructurePath(path) = &node_definition.structure.node;
		let module_path = path.module_path.clone().push(path.identifier.clone());

		definition.methods.iter().cloned().try_for_each(|(identifier, declaration)| {
			let module_path = module_path.clone();
			let path = DeclarationPath { module_path, identifier: identifier.clone() };
			let function_path = Arc::new(FunctionPath(path));

			match context.declarations_function.get(&function_path) {
				None => {
					context.declarations_function.insert(function_path.clone(),
						declaration.node).unwrap_none();
					Ok(context.node_definitions.insert(function_path,
						node_definition.clone()).unwrap_none())
				}
				Some(duplicate) => {
					let location = duplicate.span().location(context);
					let structure = node_definition.structure.node.clone();
					let error = NodeError::DuplicateMethod(structure, identifier);
					Err(Diagnostic::new(Spanned::new(error, declaration.span))
						.note(format!("Duplicate declared in: {}", location)))
				}
			}
		})
	})
}
