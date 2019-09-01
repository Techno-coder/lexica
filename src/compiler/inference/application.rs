use polytype::{Context, Type, UnificationError};

use crate::node::{DataType, Identifier, StructureMap};
use crate::source::{Span, Spanned};

use super::{TypeError, TypeResult};

/// Constrains two types to be equal within the context.
pub fn unify<'a>(mut left: Type<Identifier<'a>>, mut right: Type<Identifier<'a>>,
                 context: &mut Context<Identifier<'a>>) -> Result<(), UnificationError<Identifier<'a>>> {
	super::application::apply(&context, &mut left);
	super::application::apply(&context, &mut right);
	context.unify_fast(left, right)
}

/// Verifies that the structure has been defined.
pub fn defined<'a>(structures: &StructureMap<'a>, data_type: &DataType<'a>, span: Span) -> TypeResult<'a, ()> {
	let DataType(internal_type) = data_type;
	let type_error = TypeError::UnresolvedType(internal_type.clone());
	let identifier = data_type.resolved().ok_or(Spanned::new(type_error, span))?;

	if !data_type.is_intrinsic() {
		let identifier = Identifier(identifier);
		if !structures.contains_key(&identifier) {
			let error = TypeError::UndefinedStructure(identifier);
			return Err(Spanned::new(error, span).into());
		}
	}
	Ok(())
}

/// Recursively applies a type until it is final.
pub fn apply<'a>(context: &Context<Identifier<'a>>, internal_type: &mut Type<Identifier<'a>>) {
	loop {
		let resolved_type = internal_type.apply(context);
		match internal_type == &resolved_type {
			false => *internal_type = resolved_type,
			true => break,
		}
	}
}
