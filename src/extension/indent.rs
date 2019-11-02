use std::fmt;

pub struct Indent<'a, T> {
	inner: &'a mut T,
	new_line: bool,
}

impl<'a, T> Indent<'a, T> where T: fmt::Write {
	pub fn new(inner: &'a mut T) -> Self {
		Self { inner, new_line: true }
	}
}

impl<'a, T> fmt::Write for Indent<'a, T> where T: fmt::Write {
	fn write_str(&mut self, mut s: &str) -> fmt::Result {
		while !s.is_empty() {
			if self.new_line {
				self.inner.write_char('\t')?;
			}

			let split = match s.find('\n') {
				Some(position) => {
					self.new_line = true;
					position + 1
				}
				None => {
					self.new_line = false;
					s.len()
				}
			};

			self.inner.write_str(&s[..split])?;
			s = &s[split..];
		}
		Ok(())
	}
}