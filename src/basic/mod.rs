pub use basic::BasicFunction;
pub use node::{BasicNode, Branch, NodeTarget, Statement, Divergence, Discriminant};
pub use value::{Compound, Instance, Object, Value};

mod basic;
mod value;
mod node;
