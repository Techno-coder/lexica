use std::collections::HashMap;

use polytype::{Context, tp, Type, UnificationError};

use crate::interpreter::Size;
use crate::node::*;
use crate::source::{ErrorCollate, Spanned};

use super::TypeAnnotator;

// TODO: Type inference based on identifiers can result in overlapping identifiers for different
// TODO: variables; may require high level representation lowering
// TODO: Are all of these meant to return a type?
#[derive(Debug, Default)]
pub struct InferenceEngine<'a> {
	// TODO: Use customized type to support reduced lifetime
	environment: HashMap<Identifier<'a>, Type>,
	context: Context,
}

impl<'a> NodeVisitor<'a> for InferenceEngine<'a> {
	type Result = Result<Type, ErrorCollate<UnificationError>>;

	fn binary_operation(&mut self, operation: &mut Spanned<&mut BinaryOperation<'a>>) -> Self::Result {
		let left = operation.left.accept(self)?;
		let right = operation.right.accept(self)?;
		self.context.unify(&left, &right)?;

		Ok(match *operation.node.operator {
			BinaryOperator::Equal => tp!(bool),
			_ => left
		})
	}

	fn binding(&mut self, binding: &mut Spanned<Binding<'a>>) -> Self::Result {
		let binding_type = match &binding.variable.data_type {
			Type::Constructed(data_type) => {
				let DataType(Identifier(binding_type)) = data_type;
				// TODO: Let us pretend it is static ...
				let binding_type = Size::parse(binding_type).unwrap().to_string();
				Type::Constructed(binding_type, Vec::new())
			}
			Type::Variable(_) => self.context.new_variable(),
		};

		let identifier = binding.variable.identifier.clone();
		let expression_type = binding.expression.accept(self)?;
		self.environment.insert(identifier, expression_type.clone());

		self.context.unify(&binding_type, &expression_type)?;
		Ok(binding_type)
	}

	fn conditional_loop(&mut self, conditional_loop: &mut Spanned<ConditionalLoop<'a>>) -> Self::Result {
		let start_condition = conditional_loop.start_condition.as_mut().unwrap();
		let start_type = start_condition.accept(self)?;
		let end_type = conditional_loop.end_condition.accept(self)?;

		self.context.unify(&start_type, &tp!(bool))?;
		self.context.unify(&end_type, &tp!(bool))?;

		for statement in &mut conditional_loop.statements {
			let _ = statement.accept(self)?;
		}
		Ok(tp!(undefined))
	}

	fn explicit_drop(&mut self, explicit_drop: &mut Spanned<ExplicitDrop<'a>>) -> Self::Result {
		let identifier_type = self.environment[&explicit_drop.identifier].clone();
		let expression_type = explicit_drop.expression.accept(self)?;
		self.context.unify(&identifier_type, &expression_type)?;
		Ok(tp!(undefined))
	}

	fn expression(&mut self, expression: &mut Spanned<Expression<'a>>) -> Self::Result {
		Ok(match &mut expression.node {
			Expression::Variable(identifier) => {
				match self.environment.get(identifier) {
					Some(variable_type) => variable_type.clone(),
					None => {
						let variable_type = self.context.new_variable();
						self.environment.insert(identifier.clone(), variable_type.clone());
						variable_type
					}
				}
			}
			Expression::Primitive(primitive) => Type::Constructed(primitive.size().to_string(), Vec::new()),
			Expression::BinaryOperation(_) => expression.binary_operation().accept(self)?,
			Expression::FunctionCall(_) => expression.function_call().accept(self)?,
		})
	}

	fn function(&mut self, function: &mut Spanned<Function<'a>>) -> Self::Result {
		for parameter in &function.parameters {
			let DataType(Identifier(parameter_type)) = parameter.data_type.as_ref().unwrap();
			// TODO: Let us pretend it is static ...
			let parameter_type = Size::parse(parameter_type).unwrap().to_string();
			let parameter_type = Type::Constructed(parameter_type, Vec::new());
			self.environment.insert(parameter.identifier.clone(), parameter_type);
		}

		for statement in &mut function.statements {
			let _ = statement.accept(self)?;
		}
		Ok(tp!(undefined))
	}

	fn function_call(&mut self, function_call: &mut Spanned<&mut FunctionCall<'a>>) -> Self::Result {
		// TODO: Infer function call return type and unify against function
		for argument in &mut function_call.arguments {
			let _ = argument.accept(self)?;
		}

		// TODO: Correct return type
		Ok(tp!(u64))
	}

	fn mutation(&mut self, mutation: &mut Spanned<Mutation<'a>>) -> Self::Result {
		match &mut mutation.node {
			Mutation::Swap(left, right) => {
				let left = &self.environment[left];
				let right = &self.environment[right];
				self.context.unify(left, right)?;
			}
			Mutation::AddAssign(identifier, expression) |
			Mutation::MultiplyAssign(identifier, expression) => {
				let identifier_type = self.environment[identifier].clone();
				let expression_type = expression.accept(self)?;
				self.context.unify(&identifier_type, &expression_type)?;
			}
		}
		Ok(tp!(undefined))
	}

	fn statement(&mut self, statement: &mut Spanned<Statement<'a>>) -> Self::Result {
		match &mut statement.node {
			Statement::Binding(binding) => binding.accept(self),
			Statement::Mutation(mutation) => mutation.accept(self),
			Statement::ExplicitDrop(explicit_drop) => explicit_drop.accept(self),
			Statement::ConditionalLoop(conditional_loop) => conditional_loop.accept(self),
		}
	}

	fn syntax_unit(&mut self, syntax_unit: &mut Spanned<SyntaxUnit<'a>>) -> Self::Result {
		for (_, function) in &mut syntax_unit.functions {
			let _ = function.accept(self)?;
			let mut environment = HashMap::new();

			for (identifier, mut data_type) in self.environment.drain() {
				data_type.apply_mut(&self.context);
				environment.insert(identifier.clone(), match data_type {
					Type::Constructed(data_type, _) => DataType(Identifier(data_type)),
					Type::Variable(_) => panic!("Type could not be inferred for variable: {}", identifier),
				});
			}

			function.accept(&mut TypeAnnotator::new(environment));
			self.context = Context::default();
		}
		Ok(tp!(undefined))
	}
}
