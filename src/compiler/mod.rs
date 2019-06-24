pub use self::element::*;
pub use self::translator::*;

#[macro_use]
pub mod constructor;
mod translator;
mod element;

pub fn construct<'a>() -> crate::node::Function<'a> {
	use crate::node::*;
	Function {
		identifier: Identifier("fibonacci"),
		parameters: vec![
			Variable {
				identifier: Identifier("n"),
				data_type: Some(DataType(Identifier("u32"))),
				is_mutable: false,
			}
		],
		statements: vec![
			Statement::Binding(Binding {
				variable: Variable {
					identifier: Identifier("first"),
					data_type: None,
					is_mutable: true,
				},
				expression: Expression::LiteralInteger(1),
			}),
			Statement::Binding(Binding {
				variable: Variable {
					identifier: Identifier("second"),
					data_type: None,
					is_mutable: true,
				},
				expression: Expression::LiteralInteger(1),
			}),
			Statement::Binding(Binding {
				variable: Variable {
					identifier: Identifier("counter"),
					data_type: None,
					is_mutable: true,
				},
				expression: Expression::LiteralInteger(1),
			}),
			Statement::ConditionalLoop(ConditionalLoop {
				start_condition: Some(Expression::BinaryOperation(Box::new(BinaryOperation {
					left: Expression::Variable(Identifier("counter")),
					right: Expression::LiteralInteger(1),
					operator: BinaryOperator::Equal,
				}))),
				end_condition: Expression::BinaryOperation(Box::new(BinaryOperation {
					left: Expression::Variable(Identifier("counter")),
					right: Expression::Variable(Identifier("n")),
					operator: BinaryOperator::Equal,
				})),
				statements: vec![
					Statement::Binding(Binding {
						variable: Variable {
							identifier: Identifier("summation"),
							data_type: None,
							is_mutable: false,
						},
						expression: Expression::BinaryOperation(Box::new(BinaryOperation {
							left: Expression::Variable(Identifier("first")),
							right: Expression::Variable(Identifier("second")),
							operator: BinaryOperator::Add,
						})),
					}),
					Statement::Mutation(Mutation::Swap(Identifier("first"), Identifier("second"))),
					Statement::Mutation(Mutation::Swap(Identifier("second"), Identifier("summation"))),
					Statement::ExplicitDrop(ExplicitDrop {
						identifier: Identifier("summation"),
						expression: Expression::BinaryOperation(Box::new(BinaryOperation {
							left: Expression::Variable(Identifier("second")),
							right: Expression::Variable(Identifier("first")),
							operator: BinaryOperator::Minus,
						})),
					}),
					Statement::Mutation(Mutation::AddAssign(
						Identifier("counter"),
						Expression::LiteralInteger(1),
					)),
				],
			}),
			Statement::ExplicitDrop(ExplicitDrop {
				identifier: Identifier("n"),
				expression: Expression::Variable(Identifier("counter")),
			}),
		],
		return_value: Expression::Variable(Identifier("second")),
	}
}
