use crate::node::Structure;
use crate::source::Spanned;

use super::Function;

pub struct BasicUnit<'a> {
	pub structures: Vec<Spanned<Structure<'a>>>,
	pub functions: Vec<Spanned<Function<'a>>>,
}
