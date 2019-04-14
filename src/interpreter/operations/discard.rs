use super::{Context, InterpreterResult, Size};

#[derive(Debug)]
pub struct Discard {
	size: Size,
}

impl Discard {
	pub fn new(size: Size) -> Discard {
		Discard { size }
	}

	pub fn execute(&self, context: &mut Context) -> InterpreterResult<()> {
		let byte_count = self.size.byte_count();
		for _ in 0..byte_count {
			context.drop_stack().pop_byte()?;
		}
		Ok(())
	}
}
