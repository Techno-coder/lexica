use super::*;

pub use self::add::*;
pub use self::add_immediate::*;
pub use self::branch::*;
pub use self::call::*;
pub use self::discard::*;
pub use self::drop::*;
pub use self::jump::*;
pub use self::minus::*;
pub use self::reset::*;
pub use self::restore::*;
pub use self::swap::*;
pub use self::clone::*;

mod swap;
mod add;
mod add_immediate;
mod minus;
mod drop;
mod restore;
mod discard;
mod reset;
mod call;
mod jump;
mod branch;
mod clone;
