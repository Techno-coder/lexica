#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
	Function,
	Binding,
	Drop,
	While,
	ReturnSeparator,
	Identifier(&'a str),
	ParenthesisOpen,
	ParenthesisClose,
	BlockOpen,
	BlockClose,
	VariableSeparator,
	ListSeparator,
	Terminator,
	MutableModifier,
	Assign,
	Equal,
	LessThan,
	LessThanEqual,
	Swap,
	Implies,
	Add,
	Minus,
	Multiply,
	AddAssign,
	UnsignedInteger(u64),
}

impl<'a> Token<'a> {
	pub fn function_separator(&self) -> bool {
		match self {
			Token::Function => true,
			_ => false,
		}
	}
}
