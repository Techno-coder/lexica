pub use context::{Inclusion, InclusionTerminal, ModuleContext, ModuleContexts, Definition};
pub use declaration::{Declaration, DeclarationError, DeclarationsFunction,
	DeclarationsStructure, module_root};
use declaration::{load_module, ModulePending};
use parser::SourceParse;
pub use path::{DeclarationPath, FunctionPath, ModulePath, StructurePath};

mod declaration;
mod parser;
mod block;
mod construct;
mod context;
mod inclusion;
mod path;
