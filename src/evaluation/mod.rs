use context::{DropStack, EvaluationContext};
pub use evaluation::{function, EvaluationError, expression};
use frame::{EvaluationFrame, FrameContext};
pub use partial::{partial_function, PartialFunctions};

mod context;
mod binding;
mod mutation;
mod evaluation;
mod frame;
mod partial;
