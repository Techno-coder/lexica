use crate::interpreter::{Size, Integer};

use super::{Intrinsic, IntrinsicFunction};

pub fn trace() -> Intrinsic {
	Intrinsic {
		identifier: "trace",
		// TODO: Correct return type
		return_type: Size::Boolean,
		parameters: vec![Size::Unsigned64],
		function: IntrinsicFunction(|context| {
			let mut integer = Integer::new_unsigned(0);
			integer.restore(context.drop_stack())?;
			// TODO: Satisfies type check but is superfluous
			context.drop_stack().push_byte(0);
			Ok(println!("{}", integer))
		}),
	}
}
