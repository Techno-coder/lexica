pub use context::{EvaluationContext, FunctionFrame};
pub use evaluation::{EvaluationError, expression, function};
pub use item::{EvaluationInstance, EvaluationItem};
use item::FrameIndex;
pub use partial::{partial_function, PartialFunctions};
pub use value::{DropStack, ValueContext, ValueFrame};

mod context;
mod evaluation;
mod mutation;
mod binding;
mod value;
mod item;
mod partial;
