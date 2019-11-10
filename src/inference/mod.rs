use engine::TypeEngine;
pub use environment::{TypeContext, TypeContexts};
use environment::Environment;
use expression::expression;
pub use function::function;
use inference::{InferenceType, TypeVariable};
pub use inference::{InferenceError, TypeResolution};

mod inference;
mod function;
mod expression;
mod engine;
mod pattern;
mod environment;
