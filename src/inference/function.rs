use std::sync::Arc;

use crate::context::Context;
use crate::declaration::FunctionPath;
use crate::error::Diagnostic;

pub fn function(context: &Context, function_path: &Arc<FunctionPath>) -> Result<(), Diagnostic> {
	unimplemented!()
}
