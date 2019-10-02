use std::fmt;

use parking_lot::RwLock;

use crate::*;
use crate::context::Context;
use crate::span::Spanned;

pub type Errors = RwLock<Vec<Diagnostic>>;

#[derive(Debug)]
pub struct Diagnostic {
	pub error: Spanned<CompileError>,
	pub notes: Vec<String>,
}

impl Diagnostic {
	pub fn new<T>(error: Spanned<T>) -> Self where T: Into<CompileError> {
		let error = error.map(|node| node.into());
		Self { error, notes: Vec::new() }
	}

	pub fn note<T>(mut self, note: T) -> Self where T: Into<String> {
		self.notes.push(note.into());
		self
	}
}

impl Context {
	pub fn emit<T>(&self, result: Result<T, Diagnostic>) -> Option<T> {
		match result {
			Ok(value) => Some(value),
			Err(diagnostic) => {
				self.errors.write().push(diagnostic);
				None
			}
		}
	}
}

#[derive(Debug)]
pub enum CompileError {
	Source(source::SourceError),
	Declaration(declaration::DeclarationError),
	Parser(parser::ParserError),
}

impl fmt::Display for CompileError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			CompileError::Source(error) => write!(f, "{}", error),
			CompileError::Declaration(error) => write!(f, "{}", error),
			CompileError::Parser(error) => write!(f, "{}", error),
		}
	}
}
