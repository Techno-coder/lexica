use std::collections::HashMap;
use std::fmt;
use std::ops::Index;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::declaration::{FunctionPath, StructurePath};
use crate::node::ExpressionKey;

pub type ExpressionTypes = RwLock<HashMap<Arc<FunctionPath>, TypeRegister>>;

#[derive(Debug)]
pub struct TypeRegister {
	expression_types: HashMap<ExpressionKey, Arc<DataType>>,
}

impl TypeRegister {
	pub fn new(expression_types: HashMap<ExpressionKey, Arc<DataType>>) -> Self {
		TypeRegister { expression_types }
	}
}

impl Index<&ExpressionKey> for TypeRegister {
	type Output = Arc<DataType>;

	fn index(&self, index: &ExpressionKey) -> &Self::Output {
		self.expression_types.get(index).unwrap_or_else(||
			panic!("Data type does not exist for expression key: {:?}", index))
	}
}

#[derive(Debug)]
pub struct DataType(pub StructurePath, pub Vec<DataType>);

impl fmt::Display for DataType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let DataType(data_type, variables) = self;
		write!(f, "{}", data_type)?;

		if let Some((last, rest)) = variables.split_last() {
			write!(f, "<")?;
			rest.iter().try_for_each(|variable| write!(f, "{}, ", variable))?;
			write!(f, "{}>", last)?;
		}
		Ok(())
	}
}
