use engine::TypeEngine;
pub use function::function;
use inference::{InferenceType, TypeVariable, Environment};
pub use inference::InferenceError;

mod inference;
mod function;
mod engine;
mod pattern;
mod intrinsic;
