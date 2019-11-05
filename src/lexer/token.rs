use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
	Data,
	Function,
	Module,
	Export,
	Use,
	Identifier(Arc<str>),
	Unsigned(u64),
	Signed(i64),
	Truth(bool),
	ParenthesisOpen,
	ParenthesisClose,
	BlockOpen,
	BlockClose,
	Separator,
	ListSeparator,
	ReturnSeparator,
	PathSeparator,
	Wildcard,
	Let,
	Loop,
	Drop,
	If,
	Assign,
	LessThan,
	GreaterThan,
	Equality,
	Implies,
	Swap,
	Mutable,
	Add,
	Minus,
	Asterisk,
	AddAssign,
	MinusAssign,
	MultiplyAssign,
	LineBreak,
	End,
}

#[derive(Debug, Clone)]
pub enum LexerToken {
	Token(Token),
	Indent,
}
