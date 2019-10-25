use std::sync::Arc;

use chashmap::CHashMap;

use crate::declaration::FunctionPath;
use crate::span::Spanned;

use super::{AscriptionPattern, BindingPattern, Expression, ExpressionKey};

pub type FunctionTypes = CHashMap<Arc<FunctionPath>, Arc<FunctionType>>;
pub type NodeFunctions = CHashMap<Arc<FunctionPath>, Arc<Function>>;

#[derive(Debug, Clone)]
pub struct FunctionType {
	pub parameters: Vec<Spanned<Parameter>>,
	pub return_type: Spanned<AscriptionPattern>,
	pub function_byte_offset: usize,
}

impl FunctionType {
	pub fn new(parameters: Vec<Spanned<Parameter>>, return_type: Spanned<AscriptionPattern>,
	           function_byte_offset: usize) -> Self {
		FunctionType { parameters, return_type, function_byte_offset }
	}
}

#[derive(Debug, Clone)]
pub struct Parameter(pub BindingPattern, pub AscriptionPattern);

#[derive(Debug, Clone)]
pub struct Function {
	pub context: FunctionContext,
	pub expression: ExpressionKey,
}

impl Function {
	pub fn new(context: FunctionContext, expression: ExpressionKey) -> Function {
		Function { context, expression }
	}
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

