use std::fmt::{Debug, Display};

pub trait NodeConstruct<'a>: Debug + Display {
}

#[derive(Debug)]
pub enum ExecutionStep {
	Void,
	Repeat,
	Value(i64),
}
