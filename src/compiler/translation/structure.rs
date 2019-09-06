use std::rc::Rc;

use crate::interpreter::Size;
use crate::node::{AccessorTarget, DataType, Identifier, StructureMap, VariableTarget};

use super::StorageTarget;

pub fn structure_primitives<'a, F>(target: &VariableTarget<'a>, mut functor: F, reverse: bool,
                                   (data_type, structures): (DataType<'a>, Rc<StructureMap<'a>>))
	where F: FnMut(StorageTarget<'a>, Size) {
	let VariableTarget(identifier, generation, accessor) = target;
	let mut structure = Identifier(data_type.resolved().unwrap());
	for accessory in accessor.split() {
		let field = &structures[&structure].fields[&accessory];
		structure = Identifier(field.data_type.resolved().unwrap());
	}

	let AccessorTarget(accessor) = accessor;
	structure_primitives_recurse(identifier.clone(), *generation, &mut functor,
		accessor.to_string(), "", structure, &structures, reverse);
}

fn structure_primitives_recurse<'a, F>(identifier: Identifier<'a>, generation: usize,
                                       functor: &mut F, mut prefix: String, next: &str,
                                       structure: Identifier<'a>, structures: &Rc<StructureMap<'a>>,
                                       reverse: bool)
	where F: FnMut(StorageTarget<'a>, Size) {
	let Identifier(string) = structure;
	if let Ok(size) = Size::parse(string) {
		let target = StorageTarget(identifier, generation, prefix + next);
		return functor(target, size);
	}

	prefix += next;
	let structure = structures.get(&structure)
		.expect("Undefined structure for variable").clone();
	let mut fields: Vec<_> = structure.fields.values().collect();
	if reverse { fields.reverse(); }

	for field in fields {
		let Identifier(next) = &field.identifier.node;
		let next = ".".to_owned() + next;

		let structure = Identifier(field.data_type.resolved().unwrap());
		structure_primitives_recurse(identifier.clone(), generation,
			functor, prefix.clone(), &next, structure, structures, reverse);
	}
}
