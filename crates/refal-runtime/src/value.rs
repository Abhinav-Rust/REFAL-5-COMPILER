//! Runtime values for Refal object expressions.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Char(char),
    Identifier(String),
    Number(String),
    Bracket(Vec<Value>),
}

impl Value {
    pub fn identifier(name: impl Into<String>) -> Self {
        Self::Identifier(name.into())
    }

    pub fn number(number: impl Into<String>) -> Self {
        Self::Number(number.into())
    }
}
