use std::fmt;

pub struct IndentWriter<'a, T> {
	inner: &'a mut T,
	on_new_line: bool,
}

impl<'a, T> IndentWriter<'a, T> where T: fmt::Write {
	pub fn wrap(inner: &'a mut T) -> Self {
		Self { inner, on_new_line: true }
	}
}

impl<'a, T> fmt::Write for IndentWriter<'a, T> where T: fmt::Write {
	fn write_str(&mut self, mut s: &str) -> Result<(), fmt::Error> {
		while !s.is_empty() {
			if self.on_new_line {
				self.inner.write_char('\t')?;
			}

			let split = match s.find('\n') {
				Some(position) => {
					self.on_new_line = true;
					position + 1
				}
				None => {
					self.on_new_line = false;
					s.len()
				}
			};

			self.inner.write_str(&s[..split])?;
			s = &s[split..];
		}
		Ok(())
	}
}
