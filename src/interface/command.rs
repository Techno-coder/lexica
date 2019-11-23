use std::collections::HashMap;

use crate::context::Context;
use crate::extension::StringExtension;

pub trait Command: std::fmt::Debug {
	fn execute(&self, context: &Context, string: &str) -> String;
	fn symbols(&self, context: &Context, string: &str) -> Vec<String>;
}

#[derive(Debug)]
pub struct Commands {
	commands: HashMap<&'static str, Box<dyn Command>>,
}

impl Commands {
	pub fn new() -> Self {
		let mut commands: HashMap<_, Box<dyn Command>> = HashMap::new();
		commands.insert("context", Box::new(CommandContext));
		Self { commands }
	}

	pub fn execute(&self, context: &Context, string: &str) -> String {
		let split = string.find(char::is_whitespace).unwrap_or(string.len());
		let (command, arguments) = string.split_at(split);
		self.commands.get(command).map(|command| command.execute(context, arguments))
			.unwrap_or("Invalid command".to_owned())
	}

	pub fn symbols(&self, context: &Context, string: &str) -> Vec<String> {
		let split = string.find(char::is_whitespace).unwrap_or(string.len());
		let (command, arguments) = string.split_at(split);
		match self.commands.get(command) {
			None => self.commands.keys()
				.filter(|string| string.prefix_equal(command))
				.map(ToString::to_string).collect(),
			Some(command) => command.symbols(context, arguments),
		}
	}
}

#[derive(Debug)]
struct CommandContext;

impl Command for CommandContext {
	fn execute(&self, context: &Context, string: &str) -> String {
		format!("{:#?}", context)
	}

	fn symbols(&self, context: &Context, string: &str) -> Vec<String> {
		Vec::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_symbols() {
		let commands = Commands::new();
		let context = &Context::default();
		assert_eq!(commands.symbols(context, "con"), &["context"]);
		assert!(commands.symbols(context, "context").is_empty());
	}
}
