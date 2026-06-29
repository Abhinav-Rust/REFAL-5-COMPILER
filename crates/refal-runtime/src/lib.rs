//! Runtime object model and pattern matcher for Refal execution.

pub mod matcher;
pub mod value;

pub use matcher::{Bindings, MatchError, match_pattern};
pub use value::Value;
