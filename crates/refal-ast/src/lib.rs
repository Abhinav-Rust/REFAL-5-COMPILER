//! Core syntax tree types for the Refal compiler.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    Function(Function),
    Declaration(Declaration),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub name: String,
    pub visibility: Visibility,
    pub sentences: Vec<Sentence>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Declaration {
    pub kind: DeclarationKind,
    pub names: Vec<String>,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclarationKind {
    Extern,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Entry,
    Local,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sentence {
    pub pattern: Vec<Term>,
    pub conditions: Vec<Condition>,
    pub result: Vec<Term>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Condition {
    pub result: Vec<Term>,
    pub pattern: Vec<Term>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Term {
    pub kind: TermKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TermKind {
    Symbol(Symbol),
    Variable(Variable),
    Bracket(Vec<Term>),
    Call { name: String, args: Vec<Term> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Symbol {
    Char(char),
    Identifier(String),
    Number(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    pub kind: VariableKind,
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VariableKind {
    Symbol,
    Term,
    Expression,
}
