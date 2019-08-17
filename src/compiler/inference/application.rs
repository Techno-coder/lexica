use polytype::{Context, Type};

use crate::node::Identifier;

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
