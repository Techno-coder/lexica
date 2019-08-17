//! Transforms the abstract syntax tree to a basic graph

pub use component::*;
pub use lower_transform::*;

mod lower_transform;
mod component;
