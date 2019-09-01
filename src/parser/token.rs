#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
	Data,
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
	Accessor,
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
	MinusAssign,
	MultiplyAssign,
	Boolean(bool),
	UnsignedInteger(u64),
	SignedInteger(i64),
	Float(f64),
}

impl<'a> Token<'a> {
	pub fn structure_separator(&self) -> bool {
		match self {
			Token::Data => true,
			Token::Function => true,
			_ => false,
		}
	}
}
