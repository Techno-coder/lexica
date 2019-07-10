#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
	Function,
	Binding,
	Drop,
	Loop,
	When,
	BlockSeparator,
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
	MultiplyAssign,
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
