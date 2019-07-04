use std::vec::IntoIter;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorCollate<T> {
	Atomic(T),
	Multiple(Vec<T>),
}

impl<T> ErrorCollate<T> {
	pub fn new() -> Self {
		ErrorCollate::Multiple(Vec::new())
	}

	pub fn empty(&self) -> bool {
		match self {
			ErrorCollate::Atomic(_) => false,
			ErrorCollate::Multiple(errors) => errors.is_empty(),
		}
	}

	pub fn combine(&mut self, other: Self) {
		self.promote();
		match self {
			ErrorCollate::Multiple(errors) => match other {
				ErrorCollate::Atomic(error) => errors.push(error),
				ErrorCollate::Multiple(mut others) => errors.append(&mut others),
			}
			_ => unreachable!(),
		}
	}

	pub fn promote(&mut self) {
		if let ErrorCollate::Atomic(_) = self {
			let promotion = std::mem::replace(self, ErrorCollate::new());
			match self {
				ErrorCollate::Multiple(errors) => match promotion {
					ErrorCollate::Atomic(error) => errors.push(error),
					_ => unreachable!(),
				}
				_ => unreachable!(),
			}
		}
	}

	pub fn collapse<V>(self, value: V) -> Result<V, ErrorCollate<T>> {
		match self.empty() {
			true => Ok(value),
			false => Err(self),
		}
	}
}

impl<T> From<T> for ErrorCollate<T> {
	fn from(error: T) -> Self {
		ErrorCollate::Atomic(error)
	}
}

impl<T> IntoIterator for ErrorCollate<T> {
	type Item = T;
	type IntoIter = IntoIter<T>;

	fn into_iter(self) -> Self::IntoIter {
		match self {
			ErrorCollate::Atomic(error) => vec![error],
			ErrorCollate::Multiple(errors) => errors,
		}.into_iter()
	}
}
