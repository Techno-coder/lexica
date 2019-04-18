use super::{Context, InterpreterResult};

#[derive(Debug)]
pub struct Return;

impl Return {
	pub fn execute(context: &mut Context) -> InterpreterResult<()> {
		let return_target = context.frame()?.return_target().clone();
		context.set_next_instruction(return_target);
		context.pop_frame()?;
		Ok(())
	}
}
