use polytype::{Context, Type};

use crate::node::Identifier;

pub const BOOLEAN_TYPE: Type<Identifier<'static>> = Type::Constructed(Identifier("bool"), Vec::new());

pub fn apply<'a>(context: &Context<Identifier<'a>>, internal_type: &mut Type<Identifier<'a>>) {
	loop {
		let resolved_type = internal_type.apply(context);
		match internal_type == &resolved_type {
			false => *internal_type = resolved_type,
			true => break,
		}
	}
}
