use std::fmt;

use crate::source::Spanned;

use super::{DataType, Identifier};

#[derive(Debug, Clone)]
pub struct Structure<'a> {
	pub identifier: Spanned<Identifier<'a>>,
	pub fields: Vec<Spanned<Field<'a>>>,
}

impl<'a> fmt::Display for Structure<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use std::fmt::Write;
		use crate::utility::IndentWriter;
		writeln!(f, "data {} {{", self.identifier)?;
		let mut indent = IndentWriter::wrap(f);
		self.fields.iter().try_for_each(|field| writeln!(indent, "{},", field))?;
		write!(f, "}}")
	}
}

#[derive(Debug, Clone)]
pub struct Field<'a> {
	pub identifier: Spanned<Identifier<'a>>,
	pub data_type: Spanned<DataType<'a>>,
}

impl<'a> fmt::Display for Field<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}: ", self.identifier)?;
		match self.data_type.resolved() {
			Some(data_type) => write!(f, "{}", data_type),
			None => write!(f, "<!>"),
		}
	}
}
