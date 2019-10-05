use inference::{InferenceType, TypeVariable};
pub use inference::InferenceError;
pub use type_register::{DataType, ExpressionTypes, TypeRegister};

mod inference;
mod type_context;
mod type_register;
mod function;
