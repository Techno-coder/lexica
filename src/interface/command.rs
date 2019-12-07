use std::collections::HashMap;

use crate::context::Context;
use crate::error::Diagnostic;
use crate::extension::StringExtension;

pub trait Command: std::fmt::Debug {
	fn execute(&self, context: &Context, string: &str) -> Result<String, Diagnostic>;
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
		commands.insert("basic", Box::new(super::function::CommandBasic));
		commands.insert("evaluate", Box::new(super::function::CommandEvaluate));
		commands.insert("cycle", Box::new(super::function::CommandCycle));
		Self { commands }
	}

	pub fn execute(&self, context: &Context, string: &str) -> String {
		let split = string.find(char::is_whitespace).unwrap_or(string.len());
		let (command, arguments) = string.split_at(split);
		self.commands.get(command)
			.map(|command| match command.execute(context, arguments.trim()) {
				Err(diagnostic) => crate::error::string(context, &diagnostic),
				Ok(string) => string,
			}).unwrap_or("Invalid command".to_owned())
	}

	pub fn symbols(&self, context: &Context, string: &str) -> Vec<String> {
		let split = string.find(char::is_whitespace).unwrap_or(string.len());
		let (command, arguments) = string.split_at(split);
		match self.commands.get(command) {
			None => self.commands.keys()
				.filter(|string| string.prefix_equal(command))
				.map(ToString::to_string).collect(),
			Some(command) => command.symbols(context, arguments.trim()),
		}
	}
}

#[derive(Debug)]
struct CommandContext;

impl Command for CommandContext {
	fn execute(&self, context: &Context, _: &str) -> Result<String, Diagnostic> {
		Ok(format!("{:#?}", context))
	}

	fn symbols(&self, _: &Context, _: &str) -> Vec<String> {
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
