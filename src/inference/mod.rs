use engine::TypeEngine;
pub use environment::{TypeContext, TypeContexts};
use environment::Environment;
pub use function::function;
use inference::{InferenceType, TypeVariable};
pub use inference::{InferenceError, TypeResolution};

mod inference;
mod function;
mod engine;
mod pattern;
mod environment;
