use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
	Data,
	Function,
	Module,
	Export,
	Identifier(Arc<str>),
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
	LineBreak,
	End,
}

#[derive(Debug, Clone)]
pub enum LexerToken {
	Token(Token),
	Indent,
}
