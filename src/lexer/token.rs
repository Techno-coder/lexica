use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
	Data,
	Function,
	Module,
	Export,
	Use,
	Identifier(Arc<str>),
	Integer(i128),
	Truth(bool),
	ParenthesisOpen,
	ParenthesisClose,
	BlockOpen,
	BlockClose,
	Dot,
	Separator,
	ListSeparator,
	ReturnSeparator,
	PathSeparator,
	Wildcard,
	Reference,
	Compile,
	Let,
	Loop,
	Drop,
	If,
	Assign,
	AngleLeft,
	AngleRight,
	LessEqual,
	GreaterEqual,
	Equality,
	Implies,
	Swap,
	Template,
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
