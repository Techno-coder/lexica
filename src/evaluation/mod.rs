use context::{EvaluationContext, DropStack};
pub use evaluation::{EvaluationError, evaluate};
use frame::{EvaluationFrame, FrameContext};

mod context;
mod binding;
mod mutation;
mod evaluation;
mod frame;
