#[derive(Debug)]
pub struct Identifier(pub String);

#[derive(Debug)]
pub struct DataType(pub Identifier);

#[derive(Debug)]
pub struct Binding {
	variable: Variable,
	value: Expression,
	is_mutable: bool,
}

#[derive(Debug)]
pub struct Variable {
	identifier: Identifier,
	data_type: Option<DataType>,
}

#[derive(Debug)]
pub struct ExplicitDrop {
	identifier: Identifier,
	value: Expression,
}

#[derive(Debug)]
pub struct Swap {
	first: Identifier,
	second: Identifier,
}

#[derive(Debug)]
pub enum Statement {
	Swap(Swap),
	Binding(Binding),
	ExplicitDrop(ExplicitDrop),
	ConditionalLoop(ConditionalLoop),
	MutableOperator(MutableOperator),
}

#[derive(Debug)]
pub enum Expression {
	LiteralInteger(i64),
	BinaryOperator(BinaryOperator),
}

#[derive(Debug)]
pub enum Comparator {
	Equal,
	Less,
	LessEqual,
	Greater,
	GreaterEqual,
}

#[derive(Debug)]
pub struct Condition {
	left: Expression,
	right: Expression,
	comparator: Comparator,
}

#[derive(Debug)]
pub struct ConditionalLoop {
	statements: Vec<()>,
	start_condition: Option<Condition>,
	end_condition: Condition,
}

#[derive(Debug)]
pub enum MutableOperator {
	AddAssign,
}

#[derive(Debug)]
pub enum BinaryOperator {
	Add,
	Subtract,
}

#[derive(Debug)]
pub struct Function {
	identifier: Identifier,
	parameters: Vec<Variable>,
	statements: Vec<Statement>,
	return_type: Option<DataType>,
	return_value: Option<Expression>,
}
