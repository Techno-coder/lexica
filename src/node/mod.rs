pub use context::{function, function_type, structure};
pub use expression::{Arithmetic, BinaryOperator, Expression, ExpressionKey,
	MutationKind, ConditionStart, ConditionEnd, UnaryOperator, Branch};
pub use function::{Function, FunctionContext, FunctionType, FunctionTypes, NodeFunctions,
	Parameter};
pub use node::NodeError;
pub use structure::{NodeStructures, Structure};
pub use variable::{Ascription, AscriptionPattern, BindingPattern, BindingVariable,
	ExpressionPattern, Mutability, Pattern, Variable, VariablePattern};

mod function;
mod expression;
mod variable;
mod context;
mod structure;
mod resolution;
mod shadow;
mod node;
