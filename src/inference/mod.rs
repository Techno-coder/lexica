use engine::TypeEngine;
pub use function::function;
use inference::{Environment, InferenceType, TypeVariable};
pub use inference::InferenceError;

mod inference;
mod function;
mod engine;
mod pattern;
