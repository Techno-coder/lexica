use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::declaration::FunctionPath;
use crate::span::Spanned;

use super::{Ascription, AscriptionPattern, BindingPattern, Expression, ExpressionKey};

pub type NodeFunctions = RwLock<HashMap<Arc<FunctionPath>, Function>>;

#[derive(Debug, Clone)]
pub struct Function {
	pub context: FunctionContext,
	pub parameters: Vec<(BindingPattern, AscriptionPattern)>,
	pub return_type: Spanned<Ascription>,
	pub expression: ExpressionKey,
}

#[derive(Debug, Clone)]
pub struct FunctionContext {
	pub function_path: Arc<FunctionPath>,
	pub expressions: Vec<Spanned<Expression>>,
}

impl FunctionContext {
	pub fn new(function_path: Arc<FunctionPath>) -> Self {
		FunctionContext { function_path, expressions: Vec::new() }
	}
}

