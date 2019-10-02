use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
	Data,
	Function,
	Module,
	Export,
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
	Wildcard,
	Let,
	Loop,
	If,
	Equals,
	Implies,
	Swap,
	Mutable,
	Add,
	Minus,
	AddAssign,
	MinusAssign,
	LineBreak,
	End,
}

#[derive(Debug, Clone)]
pub enum LexerToken {
	Token(Token),
	Indent,
}
