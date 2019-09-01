use polytype::{Context, Type, UnificationError};

use crate::node::Identifier;

/// Constrains two types to be equal within the context.
pub fn unify<'a>(mut left: Type<Identifier<'a>>, mut right: Type<Identifier<'a>>,
                 context: &mut Context<Identifier<'a>>) -> Result<(), UnificationError<Identifier<'a>>> {
	super::application::apply(&context, &mut left);
	super::application::apply(&context, &mut right);
	context.unify_fast(left, right)
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
