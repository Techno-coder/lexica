pub use context::EvaluationContext;
pub use evaluation::EvaluationError;
pub use frame::{EvaluationFrame, FrameContext};

mod context;
mod binding;
mod mutation;
mod evaluation;
mod frame;
