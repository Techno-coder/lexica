use crate::interpreter::{Integer, Size};
use crate::node::DataType;

use super::{Intrinsic, IntrinsicFunction};

pub fn trace() -> Intrinsic {
	Intrinsic {
		identifier: "trace",
		return_type: DataType::UNIT_TYPE,
		parameters: vec![Size::Unsigned64.into()],
		function: IntrinsicFunction(|context| {
			let mut integer = Integer::new_unsigned(0);
			integer.restore(context.drop_stack())?;
			Ok(println!("{}", integer))
		}),
	}
}
